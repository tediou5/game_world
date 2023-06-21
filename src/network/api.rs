#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MoveSlots {
    node_addr: String,
    slots: Vec<usize>,
}

#[actix_web::post("/slots")]
pub async fn write(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<MoveSlots>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(MoveSlots { node_addr, slots }) = req;
    let req = crate::store::Request::MoveSlots { node_addr, slots };
    let response = app.raft.client_write(req).await;
    Ok(actix_web::web::Json(response))
}

#[actix_web::get("/slots")]
pub async fn read(
    app: actix_web::web::Data<crate::App>,
) -> actix_web::Result<impl actix_web::Responder> {
    let state_machine = app.store.state_machine.read().await;
    let slots = state_machine.slots.with_node();

    let res: Result<
        std::collections::HashMap<u64, std::collections::HashSet<usize>>,
        openraft::error::Infallible,
    > = Ok(slots);
    Ok(actix_web::web::Json(res))
}

#[actix_web::post("/consistent_read")]
pub async fn consistent_read(
    app: actix_web::web::Data<crate::App>,
) -> actix_web::Result<impl actix_web::Responder> {
    let ret = app.raft.is_leader().await;

    match ret {
        Ok(_) => {
            let state_machine = app.store.state_machine.read().await;
            let slots = state_machine.slots.clone();

            let res: Result<
                crate::slot::Slots,
                openraft::error::RaftError<
                    crate::NodeId,
                    openraft::error::CheckIsLeaderError<crate::NodeId, crate::Node>,
                >,
            > = Ok(slots);
            Ok(actix_web::web::Json(res))
        }
        Err(e) => Ok(actix_web::web::Json(Err(e))),
    }
}
