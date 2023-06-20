mod app;
pub mod network;
pub mod node;
mod slot;
mod socket_id;
pub mod store;
mod user;
mod vector;

use app::App;
use network::raft_network_impl::Network;
use node::Node;
use slot::Slots;
use store::Store;
use user::User;
use vector::Vector2;

pub const SLOT_NUMBER: usize = 10000;
pub type NodeId = u64;

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig: D = store::Request, R = store::Response, NodeId = NodeId, Node = node::Node
);

pub type Raft = openraft::Raft<TypeConfig, Network, std::sync::Arc<store::Store>>;

pub mod typ {
    use crate::Node;
    use crate::NodeId;
    use crate::TypeConfig;

    pub type RaftError<E = openraft::error::Infallible> = openraft::error::RaftError<NodeId, E>;
    pub type RPCError<E = openraft::error::Infallible> =
        openraft::error::RPCError<NodeId, Node, RaftError<E>>;

    pub type ClientWriteError = openraft::error::ClientWriteError<NodeId, Node>;
    pub type CheckIsLeaderError = openraft::error::CheckIsLeaderError<NodeId, Node>;
    pub type ForwardToLeader = openraft::error::ForwardToLeader<NodeId, Node>;
    pub type InitializeError = openraft::error::InitializeError<NodeId, Node>;

    pub type ClientWriteResponse = openraft::raft::ClientWriteResponse<TypeConfig>;
}

pub async fn start_raft_node(node: Node) -> std::io::Result<()> {
    // Create a configuration for the raft instance.
    let config = openraft::Config {
        heartbeat_interval: 500,
        election_timeout_min: 1500,
        election_timeout_max: 3000,
        ..Default::default()
    };

    let config = std::sync::Arc::new(config.validate().unwrap());

    // Create a instance of where the Raft data will be stored.
    let store = std::sync::Arc::new(Store::default());

    // Create the network layer that will connect and communicate the raft instances and
    // will be used in conjunction with the store created above.
    let network = Network {};

    let node_id = node.gen_id().unwrap();
    // Create a local raft instance.
    let raft = Raft::new(node_id, config.clone(), network, store.clone())
        .await
        .unwrap();

    // Create an application that will store all the instances created above, this will
    // be later used on the actix-web services.
    let app = actix_web::web::Data::new(App {
        typ: node,
        raft,
        store,
        config,
    });

    // Start the actix-web server.
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(actix_web::middleware::Compress::default())
            .app_data(app.clone())
            // raft internal RPC
            .service(network::raft::append)
            .service(network::raft::snapshot)
            .service(network::raft::vote)
            // admin API
            .service(network::management::init)
            .service(network::management::add_user)
            .service(network::management::add_computer)
            .service(network::management::change_membership)
            .service(network::management::metrics)
            // application API
            .service(network::api::write)
            .service(network::api::read)
            .service(network::api::consistent_read)
    });

    let x = server.bind(socket_id::ipv4::from_u64(node_id).unwrap())?;

    x.run().await
}
