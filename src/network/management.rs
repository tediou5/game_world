// --- Cluster management

/// Add a node as **Learner**.
///
/// A Learner receives log replication from the leader but does not vote.
/// This should be done before adding a node as a member into the cluster
/// (by calling `change-membership`)

#[actix_web::post("/add-user-node")]
pub async fn add_user(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<String>,
) -> actix_web::Result<impl actix_web::Responder> {
    let addr = req.0;
    let node_id = crate::socket_id::ipv4::into_u64(addr.as_str())?;
    let node = crate::Node::User(addr);
    let res = app.raft.add_learner(node_id, node, true).await;
    Ok(actix_web::web::Json(res))
}

#[actix_web::post("/add-compute-node")]
pub async fn add_computer(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<String>,
) -> actix_web::Result<impl actix_web::Responder> {
    let addr = req.0;
    let node_id = crate::socket_id::ipv4::into_u64(addr.as_str())?;
    let node = crate::Node::Compute(addr);
    let res = app.raft.add_learner(node_id, node, true).await;
    Ok(actix_web::web::Json(res))
}

/// Changes specified learners to members, or remove members.
#[actix_web::post("/change-membership")]
pub async fn change_membership(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<std::collections::BTreeSet<crate::NodeId>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let res = app.raft.change_membership(req.0, false).await;
    Ok(actix_web::web::Json(res))
}

/// Initialize a single-node cluster.
#[actix_web::post("/init")]
pub async fn init(
    app: actix_web::web::Data<crate::App>,
) -> actix_web::Result<impl actix_web::Responder> {
    let mut nodes = std::collections::BTreeMap::new();
    let node = &app.typ;

    nodes.insert(node.gen_id()?, node.clone());
    let res = app.raft.initialize(nodes).await;
    Ok(actix_web::web::Json(res))
}

/// Get the latest metrics of the cluster
#[actix_web::get("/metrics")]
pub async fn metrics(
    app: actix_web::web::Data<crate::App>,
) -> actix_web::Result<impl actix_web::Responder> {
    let metrics = app.raft.metrics().borrow().clone();

    let res: Result<
        openraft::RaftMetrics<crate::NodeId, crate::Node>,
        openraft::error::Infallible,
    > = Ok(metrics);
    Ok(actix_web::web::Json(res))
}
