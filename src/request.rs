#[derive(serde::Serialize, serde::Deserialize, PartialEq, Default, Debug, Clone)]
pub struct StepCompute {
    pub logout: Option<u64 /* uid */>,
    pub set_velocity: Option<(u64 /* uid */, crate::Vector2 /* velocity */)>,
    pub aoe: Vec<(
        u64, /* uid */
        f32, /* radius */
        u64, /* money */
    )>,
}

impl StepCompute {
    pub fn add(&mut self, request: ComputeRequest) {
        match request {
            ComputeRequest::Logout(uid) => self.logout = Some(uid),
            ComputeRequest::SetVelocity(SetVelocity { uid, velocity }) => {
                self.set_velocity = Some((uid, velocity))
            }
            ComputeRequest::Aoe(Aoe { uid, radius, money }) => self.aoe.push((uid, radius, money)),
        }
    }

    pub fn clear(&mut self) -> Option<Self> {
        if self.aoe.is_empty() && self.logout.is_none() && self.set_velocity.is_none() {
            return None;
        }

        let step = self.clone();

        self.aoe.clear();
        self.logout = None;
        self.set_velocity = None;

        Some(step)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ComputeRequest {
    Logout(u64),
    SetVelocity(SetVelocity),
    Aoe(Aoe),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Uid {
    pub uid: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SetVelocity {
    pub uid: u64,
    pub velocity: crate::Vector2,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Aoe {
    pub uid: u64,
    pub radius: f32,
    pub money: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Query {
    pub min: crate::Vector2,
    pub max: crate::Vector2,
}
