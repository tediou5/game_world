#[actix_web::put("/login")]
pub async fn login(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Uid>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(crate::request::Uid { uid }) = req;
    if let crate::AppData::User(users) = &app.users {
        let mut user = users.write().await;
        let user = user.entry(uid).or_default();
        let slot = user.get_slot();
        let node = app.store.state_machine.read().await;
        let node = node.slots.get_node(slot);
        if let Some(node) = node &&
        let Ok(addr) = node.get_addr() {
            let mut com_req: std::collections::HashMap<u64, crate::ComputeUser> = std::collections::HashMap::new();
            com_req.insert(uid, user.into());
            println!("merge [{uid}] into <{addr}>");
            let url = format!("http://{addr}/merge");
            // FIXME: remove this unwrap
            app.http_client.put(url).json(&com_req).send().await.unwrap();
        };
    };
    Ok(actix_web::web::Json("ok"))
}

#[actix_web::get("/users")]
pub async fn query_users_info(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::Query>,
) -> actix_web::Result<impl actix_web::Responder> {
    let mut res: std::collections::HashMap<u64, crate::ComputeUser> =
        std::collections::HashMap::new();

    let actix_web::web::Json(crate::request::Query { min, max }) = req;
    let slots = crate::Slots::from_area(&min, &max);
    let node = app.store.state_machine.read().await;
    let nodes = node.slots.get_nodes(slots);

    for (node, slots) in nodes {
        if let Ok(addr) = node.get_addr() {
            println!("query [{addr}] with {slots:?}");
            let url = format!("http://{addr}/users");
            // FIXME: handle error
            if let Ok(resp) = app.http_client.get(url).json(&slots).send().await &&
            let Ok(users) = resp.json::<std::collections::HashMap<u64, crate::ComputeUser>>().await {
                users.into_iter().filter(|(_, crate::ComputeUser { position, ..})| position.gte(&min) && position.lte(&max)).collect_into(&mut res);
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
    if let crate::AppData::User(users) = &app.users {
        if let Some(user) = users.write().await.get_mut(&uid) {
            user.add_compute_request(com_req)
        };
    }

    Ok(actix_web::web::Json("ok"))
}

#[actix_web::put("/velcoity")]
pub async fn set_velcoity(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<crate::request::SetVelocity>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(set_velcoity) = req;
    if set_velcoity.velocity.length(None) > 20.0 {
        return Ok(actix_web::web::Json("set velocity error: the velcoity len cannot bigger than 20m/s"));
    };
    let uid = set_velcoity.uid;
    let com_req = crate::ComputeRequest::SetVelocity(set_velcoity);
    if let crate::AppData::User(users) = &app.users {
        if let Some(user) = users.write().await.get_mut(&uid) {
            user.add_compute_request(com_req)
        };
    }

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
    if let crate::AppData::User(users) = &app.users {
        if let Some(user) = users.write().await.get_mut(&uid) {
            user.add_compute_request(com_req)
        };
    }

    Ok(actix_web::web::Json("ok"))
}

// update user info
#[actix_web::put("/users")]
pub async fn update_user(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<std::collections::HashMap<u64, crate::ComputeUser>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(users) = req;
    if let crate::AppData::User(local_users) = &app.users {
        let mut local_users = local_users.write().await;
        for (uid, new) in users {
            if let Some(user) = local_users.get_mut(&uid) {
                user.update(new);
            };
        }
    }
    Ok(actix_web::web::Json("ok"))
}

#[actix_web::get("/next_step")]
pub async fn next_step(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<std::collections::HashMap<u64 /* uid */, usize /* step */>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let mut res = std::collections::HashMap::new();

    let actix_web::web::Json(user_steps) = req;
    if let crate::AppData::User(users) = &app.users {
        let mut users = users.write().await;
        for (uid, step) in user_steps.iter() {
            if let Some(user) = users.get_mut(uid) {
                let (_step_num, step) = user.compute_next(*step);
                res.insert(*uid, step);
            }
        }
    }

    Ok(actix_web::web::Json(res))
}
