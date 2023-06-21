#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub next_step: crate::StepCompute,
    pub steps: Vec<crate::StepCompute>,
    pub position: crate::Vector2,
    pub velocity: crate::Vector2,
    pub monet: u64,
}

impl User {
    pub fn add_compute_request(&mut self, request: crate::ComputeRequest) {
        self.next_step.add(request)
    }

    pub fn compute_next(&mut self) -> (usize /* step num */, crate::StepCompute) {
        let step = self.next_step.clear();
        self.steps.push(step.clone());

        (self.steps.len() - 1, step)
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
            monet: Default::default(),
        }
    }
}
