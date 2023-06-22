#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Node {
    User(String),
    Compute(String),
    #[default]
    Unknow,
}

impl Node {
    pub fn get_id(&self) -> Result<u64, crate::socket_id::error::Error> {
        match self {
            Node::User(addr) | Node::Compute(addr) => {
                crate::socket_id::ipv4::into_u64(addr.as_str())
            }
            Node::Unknow => panic!("unknow node"),
        }
    }

    pub fn get_addr(&self) -> Result<&str, crate::socket_id::error::Error> {
        match self {
            Node::User(addr) | Node::Compute(addr) => Ok(addr),
            Node::Unknow => panic!("unknow node"),
        }
    }

    pub fn is_user_node(&self) -> bool {
        match self {
            Node::User(_) => true,
            _ => false,
        }
    }
}
