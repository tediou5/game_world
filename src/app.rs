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
        let slots = slots.write().await;
        for (_, slot_users) in slots.iter() {
            for (uid, crate::user::ComputeUser { step, .. }) in slot_users {
                req.insert(uid, step);
            }
        }

        let nodes = self.store.state_machine.read().await;
        if let Some((_, node)) = nodes
            .last_membership
            .nodes()
            .find(|(_, node)| node.is_user_node())
        {
            let mut step_computes = std::collections::HashMap::new();
            if let Ok(addr) = node.get_addr() {
                let url = format!("http://{addr}/next_step");
                // FIXME: handle error
                if let Ok(resp) = self.http_client.get(url).json(&req).send().await &&
                let Ok(steps) = resp.json::<std::collections::HashMap<usize, Vec<crate::request::StepCompute>>>().await {
                    steps.clone_into(&mut step_computes);
                };
            };

            // TODO: compute user
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
