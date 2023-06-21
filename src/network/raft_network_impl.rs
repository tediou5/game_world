#[derive(Clone, Debug)]
pub struct Network {}

impl Network {
    pub async fn send_rpc<Req, Resp, Err>(
        target: crate::NodeId,
        target_node: &crate::Node,
        uri: &str,
        req: Req,
    ) -> Result<Resp, openraft::error::RPCError<crate::NodeId, crate::Node, Err>>
    where
        Req: serde::Serialize,
        Err: std::error::Error + serde::de::DeserializeOwned,
        Resp: serde::de::DeserializeOwned,
    {
        let addr = &match target_node {
            crate::Node::User(addr) | crate::Node::Compute(addr) => addr,
            crate::Node::Unknow => {
                return Err(openraft::error::RPCError::Network(
                    openraft::error::NetworkError::new(&error::Error::UnknowNode),
                ))
            }
        };

        let url = format!("http://{}/{}", addr, uri);

        tracing::debug!("send_rpc to url: {}", url);

        let client = reqwest::Client::new();

        tracing::debug!("client is created for: {}", url);

        let resp = client.post(url).json(&req).send().await.map_err(|e| {
            openraft::error::RPCError::Network(openraft::error::NetworkError::new(&e))
        })?;

        tracing::debug!("client.post() is sent");

        let res: Result<Resp, Err> = resp.json().await.map_err(|e| {
            openraft::error::RPCError::Network(openraft::error::NetworkError::new(&e))
        })?;

        res.map_err(|e| {
            openraft::error::RPCError::RemoteError(openraft::error::RemoteError::new(target, e))
        })
    }
}

// NOTE: This could be implemented also on `Arc<Network>`, but since it's empty, implemented
// directly.
#[async_trait::async_trait]
impl openraft::RaftNetworkFactory<crate::TypeConfig> for Network {
    type Network = NetworkConnection;

    async fn new_client(&mut self, _target: crate::NodeId, node: &crate::Node) -> Self::Network {
        NetworkConnection {
            // owner: Network {},
            retry_times: 0,
            target_node: node.clone(),
        }
    }
}

pub struct NetworkConnection {
    // owner: Network,
    retry_times: u8,
    target_node: crate::Node,
}

#[async_trait::async_trait]
impl openraft::RaftNetwork<crate::TypeConfig> for NetworkConnection {
    async fn send_append_entries(
        &mut self,
        req: openraft::raft::AppendEntriesRequest<crate::TypeConfig>,
    ) -> Result<
        openraft::raft::AppendEntriesResponse<crate::NodeId>,
        openraft::error::RPCError<
            crate::NodeId,
            crate::Node,
            openraft::error::RaftError<crate::NodeId>,
        >,
    > {
        let res = Network::send_rpc(
            self.target_node.get_id().unwrap(),
            &self.target_node,
            "raft-append",
            req,
        )
        .await;

        match &res {
            Ok(_) => self.retry_times = 0,
            Err(_) => {
                self.retry_times += 1;
                if self.retry_times > 5 {
                    // TODO: remove node: Raft::change_membership()
                    if let Some(raft) = crate::RAFT_CLIENT.get() &&
                    let Some(current_leader) = raft.current_leader().await {
                        let membership_config = &raft.metrics();
                        let membership_config = &membership_config.borrow().membership_config;
                        let leader = membership_config.membership().get_node(&current_leader).unwrap().clone();
                        let mut members: std::collections::BTreeSet<crate::NodeId>= membership_config
                            .nodes()
                            .map(|(id, _)| *id)
                            .collect();
                        members.remove(&self.target_node.get_id().unwrap());

                        tokio::task::spawn(async move {
                            let _: Result<crate::typ::ClientWriteResponse, openraft::error::RPCError<u64, crate::node::Node, openraft::error::ClientWriteError<crate::NodeId, crate::Node>>>
                                = Network::send_rpc(current_leader, &leader, "change-membership", members).await;
                        });
                    }
                }
            }
        }

        res
    }

    async fn send_install_snapshot(
        &mut self,
        req: openraft::raft::InstallSnapshotRequest<crate::TypeConfig>,
    ) -> Result<
        openraft::raft::InstallSnapshotResponse<crate::NodeId>,
        openraft::error::RPCError<
            crate::NodeId,
            crate::Node,
            openraft::error::RaftError<crate::NodeId, openraft::error::InstallSnapshotError>,
        >,
    > {
        Network::send_rpc(
            self.target_node.get_id().unwrap(),
            &self.target_node,
            "raft-snapshot",
            req,
        )
        .await
    }

    async fn send_vote(
        &mut self,
        req: openraft::raft::VoteRequest<crate::NodeId>,
    ) -> Result<
        openraft::raft::VoteResponse<crate::NodeId>,
        openraft::error::RPCError<
            crate::NodeId,
            crate::Node,
            openraft::error::RaftError<crate::NodeId>,
        >,
    > {
        Network::send_rpc(
            self.target_node.get_id().unwrap(),
            &self.target_node,
            "raft-vote",
            req,
        )
        .await
    }
}

mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("unknow node")]
        UnknowNode,
    }
}
