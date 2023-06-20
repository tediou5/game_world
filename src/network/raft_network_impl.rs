pub struct Network {}

impl Network {
    pub async fn send_rpc<Req, Resp, Err>(
        &self,
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

    async fn new_client(&mut self, target: crate::NodeId, node: &crate::Node) -> Self::Network {
        NetworkConnection {
            owner: Network {},
            target,
            target_node: node.clone(),
        }
    }
}

pub struct NetworkConnection {
    owner: Network,
    target: crate::NodeId,
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
        self.owner
            .send_rpc(self.target, &self.target_node, "raft-append", req)
            .await
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
        self.owner
            .send_rpc(self.target, &self.target_node, "raft-snapshot", req)
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
        self.owner
            .send_rpc(self.target, &self.target_node, "raft-vote", req)
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
