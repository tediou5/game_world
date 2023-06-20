#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Node {
    User(String),
    Compute(String),
    #[default]
    Unknow,
}

impl Node {
    pub fn gen_id(&self) -> Result<u64, crate::socket_id::error::Error> {
        match self {
            Node::User(addr) | Node::Compute(addr) => {
                crate::socket_id::ipv4::into_u64(addr.as_str())
            }
            Node::Unknow => panic!("unknow node"),
        }
    }
}
