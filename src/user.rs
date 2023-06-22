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
        if let Some(step) = self.next_step.clear() {
            self.steps.push(step);
        }

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
            aoe: vec![(0, 5.0, 100), (0, 6.0, 100)],
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
            aoe: vec![(0, 5.0, 100), (0, 6.0, 100)],
        };
        let step = vec![step.clone(), step];

        assert_eq!(b_step_num, 2);
        assert_eq!(b_step, step);

        let (c_step_num, c_step) = user.compute_next(2);
        assert_eq!(c_step_num, 2);
        assert_eq!(c_step.len(), 0);
    }
}
