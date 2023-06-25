// Representation of an application state. This struct can be shared around to share
// instances of raft, store and more.
pub struct App {
    // pub addr: String,
    pub typ: crate::Node,
    pub raft: crate::Raft,
    pub http_client: reqwest::Client,
    pub store: std::sync::Arc<crate::Store>,
    pub users: AppData,
    pub config: std::sync::Arc<openraft::Config>,
}

impl App {
    // Error if is user node
    pub async fn compute(&self) -> Result<(), ()> {
        let slots = match &self.users {
            AppData::User(_) => return Err(()),
            AppData::Compute(slots) => slots,
        };
        let mut req = std::collections::HashMap::new();
        let mut slots = slots.write().await;
        println!(">>>>>>>>>Compute slots:\n{slots:?}\n<<<<<<<<<end");
        for (_, slot_users) in slots.iter() {
            for (uid, crate::user::ComputeUser { step, .. }) in slot_users {
                req.insert(*uid, *step);
            }
        }

        let nodes = self.store.state_machine.read().await;
        if let Some((_, node)) = nodes
            .last_membership
            .nodes()
            .find(|(_, node)| node.is_user_node())
        {
            let mut step_computes = std::collections::HashMap::new();
            let mut moved_users: std::collections::HashMap<
                &str, /* addr str */
                std::collections::HashMap<u64, crate::ComputeUser>,
            > = std::collections::HashMap::new();
            if let Ok(addr) = node.get_addr() {
                let mut logout_users = std::collections::HashMap::new();

                let url = format!("http://{addr}/next_step");
                // FIXME: handle error
                if let Ok(resp) = self.http_client.get(url).json(&req).send().await &&
                let Ok(steps) = resp.json::<std::collections::HashMap<u64, Vec<crate::request::StepCompute>>>().await {
                    steps.into_iter().collect_into(&mut step_computes);
                };
                let node_slots = &nodes.slots;
                // TODO: compute user
                for (slot, slot_users) in slots.iter_mut() {
                    slot_users.retain(|uid, user| {
                        if let Some(steps) = step_computes.remove(uid) &&
                        let Some(logout) = user.compute(steps, node_slots, &self.http_client) {
                            logout_users.insert(*uid, logout);
                            return false;
                        };
                        let current_slot = user.get_slot();
                        if current_slot != *slot {
                            if let Some(node) = node_slots.get_node(*slot) &&
                            let Ok(addr) = node.get_addr() {
                                moved_users.entry(addr).or_default().insert(*uid, user.clone());
                            };
                            false
                        } else {
                            true
                        }
                    });
                }

                // move user to target slot
                for (addr, move_user) in moved_users {
                    let url = format!("http://{addr}/merge");
                    // FIXME: remove this unwrap
                    let _ = &self
                        .http_client
                        .put(url)
                        .json(&move_user)
                        .send()
                        .await
                        .unwrap();
                }

                // logout, callback to user node to update user info
                let url = format!("http://{addr}/users");
                let http_client = self.http_client.clone();
                tokio::task::spawn(async move {
                    let _ = http_client.put(url).json(&logout_users).send().await;
                });
            };
        };

        Ok(())
    }
}

pub enum AppData {
    User(tokio::sync::RwLock<std::collections::HashMap<u64 /* uid */, crate::User>>),
    Compute(
        tokio::sync::RwLock<
            std::collections::HashMap<
                usize, /* slot */
                std::collections::HashMap<u64 /* uid */, crate::ComputeUser>,
            >,
        >,
    ),
}
