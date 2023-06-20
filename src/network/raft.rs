#[actix_web::post("/raft-vote")]
pub async fn vote(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<openraft::raft::VoteRequest<crate::NodeId>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let res = app.raft.vote(req.0).await;
    Ok(actix_web::web::Json(res))
}

#[actix_web::post("/raft-append")]
pub async fn append(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<openraft::raft::AppendEntriesRequest<crate::TypeConfig>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let res = app.raft.append_entries(req.0).await;
    Ok(actix_web::web::Json(res))
}

#[actix_web::post("/raft-snapshot")]
pub async fn snapshot(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<openraft::raft::InstallSnapshotRequest<crate::TypeConfig>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let res = app.raft.install_snapshot(req.0).await;
    Ok(actix_web::web::Json(res))
}
