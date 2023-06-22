#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SubAoe {
    position: crate::Vector2,
    radius: f32,
    money: u64,
}

#[actix_web::post("/aoe")]
pub async fn aoe(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<SubAoe>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(SubAoe {
        position,
        radius,
        money,
    }) = req;
    if let crate::AppData::Compute(users) = &app.users {
        let mut users = users.write().await;
        for (_, slot_users) in users.iter_mut() {
            slot_users
                .iter_mut()
                .filter(
                    |(
                        _,
                        crate::ComputeUser {
                            position: user_position,
                            ..
                        },
                    )| { position.length(user_position) < radius },
                )
                .for_each(|(_, user)| {
                    user.money += money;
                });
        }
    }
    Ok(actix_web::web::Json("ok"))
}

#[actix_web::get("/users")]
pub async fn query(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<Vec<usize>>, // slots
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(slots) = req;
    let mut res: std::collections::HashMap<u64, crate::ComputeUser> =
        std::collections::HashMap::new();
    if let crate::AppData::Compute(users) = &app.users {
        let user = users.read().await;
        for slot in slots.iter() {
            if let Some(slot_users) = user.get(slot) {
                slot_users.clone_into(&mut res);
            }
        }
    }
    Ok(actix_web::web::Json(res))
}

#[actix_web::put("/merge")]
pub async fn merge(
    app: actix_web::web::Data<crate::App>,
    req: actix_web::web::Json<std::collections::HashMap<u64, crate::ComputeUser>>,
) -> actix_web::Result<impl actix_web::Responder> {
    let actix_web::web::Json(merge_users) = req;
    if let crate::AppData::Compute(users) = &app.users {
        let mut users = users.write().await;
        for (uid, user) in merge_users {
            users.entry(user.get_slot()).or_default().insert(uid, user);
        }
    };
    Ok(actix_web::web::Json("ok"))
}
