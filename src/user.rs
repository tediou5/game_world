#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct User {
    pub step: u64,
    pub position: crate::Vector2,
    pub velocity: crate::Vector2,
    pub monet: u64,
}
