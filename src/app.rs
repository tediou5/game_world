// Representation of an application state. This struct can be shared around to share
// instances of raft, store and more.
pub struct App {
    // pub addr: String,
    pub typ: crate::Node,
    pub raft: crate::Raft,
    pub store: std::sync::Arc<crate::Store>,
    pub config: std::sync::Arc<openraft::Config>,
}
