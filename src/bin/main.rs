use clap::Parser;
use game_world::network::raft_network_impl::Network;
use game_world::start_raft_node;
use game_world::store::Store;
use game_world::TypeConfig;
use openraft::Raft;
use tracing_subscriber::EnvFilter;

pub type ExampleRaft = Raft<TypeConfig, Network, Store>;

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    #[clap(long)]
    pub typ: String,

    #[clap(long)]
    pub addr: String,
}

impl From<Opt> for game_world::node::Node {
    fn from(opt: Opt) -> Self {
        match opt.typ.as_str() {
            "user" => game_world::node::Node::User(opt.addr),
            "compute" => game_world::node::Node::Compute(opt.addr),
            _ => panic!("type must be 'user' or 'compute'"),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_ansi(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Parse the parameters passed by arguments.
    let opt = Opt::parse();

    start_raft_node(opt.into()).await
}
