#[actix_web::post("/aoe")]
pub async fn aoe(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Uid>,
) -> actix_web::Result<impl actix_web::Responder> {
    // let actix_web::web::Json(Uid { uid }) = req;
    // let req = crate::store::Request::MoveSlots { node_addr, slots };
    // let response = app.raft.client_write(req).await;
    Ok(actix_web::web::Json(""))
}

#[actix_web::post("/sub-aoe")]
pub async fn sub_aoe(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Uid>,
) -> actix_web::Result<impl actix_web::Responder> {
    // let actix_web::web::Json(Uid { uid }) = req;
    // let req = crate::store::Request::MoveSlots { node_addr, slots };
    // let response = app.raft.client_write(req).await;
    Ok(actix_web::web::Json(""))
}

#[actix_web::post("/query")]
pub async fn query(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Uid>,
) -> actix_web::Result<impl actix_web::Responder> {
    // let actix_web::web::Json(Uid { uid }) = req;
    // let req = crate::store::Request::MoveSlots { node_addr, slots };
    // let response = app.raft.client_write(req).await;
    Ok(actix_web::web::Json(""))
}

#[actix_web::post("/merge")]
pub async fn merge(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Uid>,
) -> actix_web::Result<impl actix_web::Responder> {
    // let actix_web::web::Json(Uid { uid }) = req;
    // let req = crate::store::Request::MoveSlots { node_addr, slots };
    // let response = app.raft.client_write(req).await;
    Ok(actix_web::web::Json("ok"))
}

#[actix_web::post("/logout")]
pub async fn logout(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Uid>,
) -> actix_web::Result<impl actix_web::Responder> {
    // let actix_web::web::Json(Uid { uid }) = req;
    // let req = crate::store::Request::MoveSlots { node_addr, slots };
    // let response = app.raft.client_write(req).await;

    // TODO: return user info to user node

    Ok(actix_web::web::Json(""))
}
