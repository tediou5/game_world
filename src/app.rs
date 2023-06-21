// Representation of an application state. This struct can be shared around to share
// instances of raft, store and more.
pub struct App {
    // pub addr: String,
    pub typ: crate::Node,
    pub raft: crate::Raft,
    pub http_client: reqwest::Client,
    pub store: std::sync::Arc<crate::Store>,
    pub users: tokio::sync::RwLock<std::collections::HashMap<u64, crate::User>>,
    pub config: std::sync::Arc<openraft::Config>,
}
