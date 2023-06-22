#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ComputeUser {
    pub step: usize,
    pub position: crate::Vector2,
    pub velocity: crate::Vector2,
    pub money: u64,
}

impl ComputeUser {
    pub fn get_slot(&self) -> usize {
        crate::Slots::from_position(&self.position)
    }

    pub fn compute(
        &mut self,
        mut steps: Vec<crate::StepCompute>,
        node_slots: &crate::Slots,
        http_client: &reqwest::Client,
    ) -> Option<Self> {
        let times = steps.len();
        if times == 0 {
            return None;
        }

        let latest = steps.pop();
        for mut step in steps.into_iter() {
            step.aoes.clear();
            step.logout = None;
            self.compute_once(step, node_slots, http_client);
        }
        if let Some(latest) = latest {
            self.compute_once(latest, node_slots, http_client)
        } else {
            None
        }
    }

    fn compute_once(
        &mut self,
        crate::StepCompute {
            logout,
            set_velocity,
            aoes, // uid, radius, money
        }: crate::StepCompute,
        node_slots: &crate::Slots,
        http_client: &reqwest::Client,
    ) -> Option<Self> {
        self.step += 1;
        if let Some((_, velocity)) = set_velocity {
            self.velocity = velocity;
        }

        for (uid, radius, money) in aoes.into_iter() {
            // let aoe_nodes = std::collections::HashSet::new();
            let aoe_slots = crate::Slots::from_radius(&self.position, radius);
            let aoe_nodes = node_slots.get_nodes(aoe_slots);
            for (node, _slots) in aoe_nodes.into_iter() {
                if let Ok(addr) = node.get_addr() {
                    let url = format!("http://{addr}/aoe");
                    let client = http_client.clone();
                    let position = self.position.clone();
                    tokio::task::spawn(async move {
                        let _ = client
                            .post(url)
                            .json(&crate::SubAoe {
                                uid,
                                position,
                                radius,
                                money,
                            })
                            .send()
                            .await;
                    });
                };
            }
        }

        self.position.move_once(&self.velocity);

        if logout.is_some() {
            Some(self.clone())
        } else {
            None
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub step: usize,
    pub next_step: crate::StepCompute,
    pub steps: Vec<crate::StepCompute>,
    pub position: crate::Vector2,
    pub velocity: crate::Vector2,
    pub money: u64,
}

impl User {
    pub fn update(&mut self, new: ComputeUser) {
        let ComputeUser {
            step,
            position,
            velocity,
            money,
        } = new;
        self.step = step;
        self.position = position;
        self.velocity = velocity;
        self.money = money;
    }

    pub fn add_compute_request(&mut self, request: crate::ComputeRequest) {
        self.next_step.add(request)
    }

    pub fn compute_next(
        &mut self,
        old_step: usize,
    ) -> (usize /* step num */, Vec<crate::StepCompute>) {
        self.steps.push(self.next_step.clear());

        (self.steps.len(), self.steps[old_step..].to_vec())
    }

    pub fn get_slot(&self) -> usize {
        crate::Slots::from_position(&self.position)
    }
}

impl Default for User {
    fn default() -> Self {
        use rand::Rng as _;
        let mut rng = rand::thread_rng();
        Self {
            next_step: Default::default(),
            steps: Default::default(),
            position: crate::Vector2 {
                x: rng.gen_range(-100000.0..=100000.0),
                y: rng.gen_range(-100000.0..=100000.0),
            },
            velocity: Default::default(),
            money: Default::default(),
            step: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        request::{Aoe, ComputeRequest, SetVelocity, StepCompute},
        vector::Vector2,
    };

    use super::User;

    #[test]
    fn next_step() {
        let mut user = User::default();
        user.add_compute_request(ComputeRequest::SetVelocity(SetVelocity {
            uid: 0,
            velocity: Vector2 { x: 0.0, y: 0.0 },
        }));
        user.add_compute_request(ComputeRequest::Aoe(Aoe {
            uid: 0,
            radius: 5.0,
            money: 100,
        }));
        user.add_compute_request(ComputeRequest::Aoe(Aoe {
            uid: 0,
            radius: 6.0,
            money: 100,
        }));

        let (a_step_num, a_step) = user.compute_next(0);
        let step = StepCompute {
            logout: None,
            set_velocity: Some((0, Vector2 { x: 0.0, y: 0.0 })),
            aoes: vec![(0, 5.0, 100), (0, 6.0, 100)],
        };
        let step = vec![step.clone()];

        assert_eq!(a_step_num, 1);
        assert_eq!(a_step, step);

        user.add_compute_request(ComputeRequest::SetVelocity(SetVelocity {
            uid: 0,
            velocity: Vector2 { x: 0.0, y: 0.0 },
        }));
        user.add_compute_request(ComputeRequest::Aoe(Aoe {
            uid: 0,
            radius: 5.0,
            money: 100,
        }));
        user.add_compute_request(ComputeRequest::Aoe(Aoe {
            uid: 0,
            radius: 6.0,
            money: 100,
        }));

        let (b_step_num, b_step) = user.compute_next(0);
        let step = StepCompute {
            logout: None,
            set_velocity: Some((0, Vector2 { x: 0.0, y: 0.0 })),
            aoes: vec![(0, 5.0, 100), (0, 6.0, 100)],
        };
        let step = vec![step.clone(), step];

        assert_eq!(b_step_num, 2);
        assert_eq!(b_step, step);

        let (c_step_num, c_step) = user.compute_next(2);
        assert_eq!(c_step_num, 3);
        assert_eq!(c_step.len(), 1);
    }
}
