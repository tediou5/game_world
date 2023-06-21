#[actix_web::put("/login")]
pub async fn login(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Uid>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(crate::request::Uid { uid }) = req;
    let mut user = app.users.write().await;
    let user = user.entry(uid).or_default().clone();
    let slot = user.get_slot();
    let node = app.store.state_machine.read().await;
    let node = node.slots.get_node(slot);
    if let Some(node) = node &&
    let Ok(addr) = node.get_addr() {
        let mut com_req = std::collections::HashMap::new();
        com_req.insert(uid, user);
        let url = format!("http://{addr}/merge");
        // FIXME: remove this unwrap
        app.http_client.put(url).json(&com_req).send().await.unwrap();
    };
    Ok(actix_web::web::Json("ok"))
}

#[actix_web::get("/users")]
pub async fn query_users_info(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Query>,
) -> actix_web::Result<impl actix_web::Responder> {
    let mut res: std::collections::HashMap<u64, crate::User> = std::collections::HashMap::new();

    let actix_web::web::Json(crate::request::Query { min, max }) = req;
    let slots = crate::Slots::from_area(&min, &max);
    let node = app.store.state_machine.read().await;
    let nodes = node.slots.get_nodes(slots);

    for (node, slots) in nodes {
        if let Ok(addr) = node.get_addr() {
            let url = format!("http://{addr}/query");
            // FIXME: remove this unwrap
            if let Ok(resp) = app.http_client.put(url).json(&slots).send().await &&
            let Ok(users) = resp.json::<std::collections::HashMap<u64, crate::User>>().await {
                users.into_iter().collect_into(&mut res);
            };
        };
    }
    Ok(actix_web::web::Json(res))
}

#[actix_web::put("/logout")]
pub async fn logout(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Uid>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(crate::request::Uid { uid }) = req;
    let com_req = crate::ComputeRequest::Logout(uid);
    app.users.write().await.get_mut(&uid).map(|user| user.add_compute_request(com_req));

    Ok(actix_web::web::Json("ok"))
}

#[actix_web::put("/velcoity")]
pub async fn set_velcoity(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::SetVelocity>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(set_velcoity) = req;
    let uid = set_velcoity.uid;
    let com_req = crate::ComputeRequest::SetVelocity(set_velcoity);
    app.users.write().await.get_mut(&uid).map(|user| user.add_compute_request(com_req));

    Ok(actix_web::web::Json("ok"))
}

#[actix_web::post("/aoe")]
pub async fn aoe(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Aoe>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(aoe) = req;
    let uid = aoe.uid;
    let com_req = crate::ComputeRequest::Aoe(aoe);
    app.users.write().await.get_mut(&uid).map(|user| user.add_compute_request(com_req));

    Ok(actix_web::web::Json("ok"))
}

// update user info
#[actix_web::put("/users")]
pub async fn update_user(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<std::collections::HashMap<u64, crate::User>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(users) = req;
    let mut local_users = app.users.write().await;
    for (uid, user) in users {
        local_users.insert(uid, user);
    }
    Ok(actix_web::web::Json("ok"))
}
