#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Request {
    MoveSlots {
        node_addr: String,
        slots: Vec<usize>,
    },
    UpdateWorldSnapshot {
        snapshot: std::collections::BTreeMap<u64, crate::User>,
    },
}

/**
 * Here you will defined what type of answer you expect from reading the data of a node.
 * In this  it will return a optional value from a given key in
 * the `Request.Set`.
 *
 * TODO: Should we explain how to create multiple `AppDataResponse`?
 *
 */
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Response {
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct Snapshot {
    pub meta: openraft::SnapshotMeta<crate::NodeId, crate::Node>,

    /// The data of the state machine at the time of this snapshot.
    pub data: Vec<u8>,
}

/**
 * Here defines a state machine of the raft, this state represents a copy of the data
 * between each node. Note that we are using `serde` to serialize the `data`, which has
 * a implementation to be serialized. Note that for this test we set both the key and
 * value as String, but you could set any type of value that has the serialization impl.
 */
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct StateMachine {
    pub last_applied_log: Option<openraft::LogId<crate::NodeId>>,

    pub last_membership: openraft::StoredMembership<crate::NodeId, crate::Node>,

    pub slots: crate::Slots,
    pub world_state_snapshot: std::collections::BTreeMap<u64, crate::User>,
}

#[derive(Debug, Default)]
pub struct Store {
    last_purged_log_id: tokio::sync::RwLock<Option<openraft::LogId<crate::NodeId>>>,

    /// The Raft log.
    log: tokio::sync::RwLock<std::collections::BTreeMap<u64, openraft::Entry<crate::TypeConfig>>>,

    /// The Raft state machine.
    pub state_machine: tokio::sync::RwLock<StateMachine>,

    /// The current granted vote.
    vote: tokio::sync::RwLock<Option<openraft::Vote<crate::NodeId>>>,

    snapshot_idx: std::sync::Arc<tokio::sync::Mutex<u64>>,

    current_snapshot: tokio::sync::RwLock<Option<Snapshot>>,
}

#[async_trait::async_trait]
impl openraft::RaftLogReader<crate::TypeConfig> for std::sync::Arc<Store> {
    async fn get_log_state(
        &mut self,
    ) -> Result<openraft::LogState<crate::TypeConfig>, openraft::StorageError<crate::NodeId>> {
        let log = self.log.read().await;
        let last = log.iter().next_back().map(|(_, ent)| ent.log_id);

        let last_purged = *self.last_purged_log_id.read().await;

        let last = match last {
            None => last_purged,
            Some(x) => Some(x),
        };

        Ok(openraft::LogState {
            last_purged_log_id: last_purged,
            last_log_id: last,
        })
    }

    async fn try_get_log_entries<
        RB: std::ops::RangeBounds<u64> + Clone + std::fmt::Debug + Send + Sync,
    >(
        &mut self,
        range: RB,
    ) -> Result<Vec<openraft::Entry<crate::TypeConfig>>, openraft::StorageError<crate::NodeId>>
    {
        let log = self.log.read().await;
        let response = log
            .range(range.clone())
            .map(|(_, val)| val.clone())
            .collect::<Vec<_>>();
        Ok(response)
    }
}

#[async_trait::async_trait]
impl openraft::RaftSnapshotBuilder<crate::TypeConfig, std::io::Cursor<Vec<u8>>>
    for std::sync::Arc<Store>
{
    #[tracing::instrument(level = "trace", skip(self))]
    async fn build_snapshot(
        &mut self,
    ) -> Result<
        openraft::Snapshot<crate::NodeId, crate::Node, std::io::Cursor<Vec<u8>>>,
        openraft::StorageError<crate::NodeId>,
    > {
        let data;
        let last_applied_log;
        let last_membership;

        {
            // Serialize the data of the state machine.
            let state_machine = self.state_machine.read().await;
            data = serde_json::to_vec(&*state_machine).map_err(|e| {
                openraft::StorageIOError::new(
                    openraft::ErrorSubject::StateMachine,
                    openraft::ErrorVerb::Read,
                    openraft::AnyError::new(&e),
                )
            })?;

            last_applied_log = state_machine.last_applied_log;
            last_membership = state_machine.last_membership.clone();
        }

        let snapshot_idx = {
            let mut l = self.snapshot_idx.lock().await;
            *l += 1;
            *l
        };

        let snapshot_id = if let Some(last) = last_applied_log {
            format!("{}-{}-{}", last.leader_id, last.index, snapshot_idx)
        } else {
            format!("--{}", snapshot_idx)
        };

        let meta = openraft::SnapshotMeta {
            last_log_id: last_applied_log,
            last_membership,
            snapshot_id,
        };

        let snapshot = Snapshot {
            meta: meta.clone(),
            data: data.clone(),
        };

        {
            let mut current_snapshot = self.current_snapshot.write().await;
            *current_snapshot = Some(snapshot);
        }

        Ok(openraft::Snapshot {
            meta,
            snapshot: Box::new(std::io::Cursor::new(data)),
        })
    }
}

#[async_trait::async_trait]
impl openraft::RaftStorage<crate::TypeConfig> for std::sync::Arc<Store> {
    type SnapshotData = std::io::Cursor<Vec<u8>>;
    type LogReader = Self;
    type SnapshotBuilder = Self;

    #[tracing::instrument(level = "trace", skip(self))]
    async fn save_vote(
        &mut self,
        vote: &openraft::Vote<crate::NodeId>,
    ) -> Result<(), openraft::StorageError<crate::NodeId>> {
        let mut v = self.vote.write().await;
        *v = Some(*vote);
        Ok(())
    }

    async fn read_vote(
        &mut self,
    ) -> Result<Option<openraft::Vote<crate::NodeId>>, openraft::StorageError<crate::NodeId>> {
        Ok(*self.vote.read().await)
    }

    #[tracing::instrument(level = "trace", skip(self, entries))]
    async fn append_to_log(
        &mut self,
        entries: &[&openraft::Entry<crate::TypeConfig>],
    ) -> Result<(), openraft::StorageError<crate::NodeId>> {
        let mut log = self.log.write().await;
        for entry in entries {
            log.insert(entry.log_id.index, (*entry).clone());
        }
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn delete_conflict_logs_since(
        &mut self,
        log_id: openraft::LogId<crate::NodeId>,
    ) -> Result<(), openraft::StorageError<crate::NodeId>> {
        tracing::debug!("delete_log: [{:?}, +oo)", log_id);

        let mut log = self.log.write().await;
        let keys = log
            .range(log_id.index..)
            .map(|(k, _v)| *k)
            .collect::<Vec<_>>();
        for key in keys {
            log.remove(&key);
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn purge_logs_upto(
        &mut self,
        log_id: openraft::LogId<crate::NodeId>,
    ) -> Result<(), openraft::StorageError<crate::NodeId>> {
        tracing::debug!("delete_log: [{:?}, +oo)", log_id);

        {
            let mut ld = self.last_purged_log_id.write().await;
            assert!(*ld <= Some(log_id));
            *ld = Some(log_id);
        }

        {
            let mut log = self.log.write().await;

            let keys = log
                .range(..=log_id.index)
                .map(|(k, _v)| *k)
                .collect::<Vec<_>>();
            for key in keys {
                log.remove(&key);
            }
        }

        Ok(())
    }

    async fn last_applied_state(
        &mut self,
    ) -> Result<
        (
            Option<openraft::LogId<crate::NodeId>>,
            openraft::StoredMembership<crate::NodeId, crate::Node>,
        ),
        openraft::StorageError<crate::NodeId>,
    > {
        let state_machine = self.state_machine.read().await;
        Ok((
            state_machine.last_applied_log,
            state_machine.last_membership.clone(),
        ))
    }

    #[tracing::instrument(level = "trace", skip(self, entries))]
    async fn apply_to_state_machine(
        &mut self,
        entries: &[&openraft::Entry<crate::TypeConfig>],
    ) -> Result<Vec<Response>, openraft::StorageError<crate::NodeId>> {
        let mut res = Vec::with_capacity(entries.len());

        let mut sm = self.state_machine.write().await;

        for entry in entries {
            tracing::debug!(%entry.log_id, "replicate to sm");

            sm.last_applied_log = Some(entry.log_id);

            match entry.payload {
                openraft::EntryPayload::Blank => res.push(Response { value: None }),
                openraft::EntryPayload::Normal(ref req) => match req {
                    Request::MoveSlots { node_addr, slots } => {
                        let node = sm
                            .last_membership
                            .nodes()
                            .find(|(_id, node)| {
                                if let crate::Node::Compute(addr) = node {
                                    addr.eq(node_addr)
                                } else {
                                    false
                                }
                            })
                            .map(|(_id, node)| node.clone());
                        if let Some(node) = node {
                            sm.slots.move_to(&node, slots);
                            res.push(Response {
                                value: Some("ok".to_string()),
                            });
                        } else {
                            res.push(Response {
                                value: Some(format!("no such compute node: {:?}", node_addr)),
                            })
                        }
                    }
                    Request::UpdateWorldSnapshot { snapshot: _ } => {
                        // TODO: update snapshot
                        todo!()
                    }
                },
                openraft::EntryPayload::Membership(ref mem) => {
                    sm.last_membership =
                        openraft::StoredMembership::new(Some(entry.log_id), mem.clone());
                    res.push(Response { value: None })
                }
            };
        }
        Ok(res)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn begin_receiving_snapshot(
        &mut self,
    ) -> Result<Box<Self::SnapshotData>, openraft::StorageError<crate::NodeId>> {
        Ok(Box::new(std::io::Cursor::new(Vec::new())))
    }

    #[tracing::instrument(level = "trace", skip(self, snapshot))]
    async fn install_snapshot(
        &mut self,
        meta: &openraft::SnapshotMeta<crate::NodeId, crate::Node>,
        snapshot: Box<Self::SnapshotData>,
    ) -> Result<(), openraft::StorageError<crate::NodeId>> {
        tracing::info!(
            { snapshot_size = snapshot.get_ref().len() },
            "decoding snapshot for installation"
        );

        let new_snapshot = Snapshot {
            meta: meta.clone(),
            data: snapshot.into_inner(),
        };

        // Update the state machine.
        {
            let updated_state_machine: StateMachine = serde_json::from_slice(&new_snapshot.data)
                .map_err(|e| {
                    openraft::StorageIOError::new(
                        openraft::ErrorSubject::Snapshot(new_snapshot.meta.signature()),
                        openraft::ErrorVerb::Read,
                        openraft::AnyError::new(&e),
                    )
                })?;
            let mut state_machine = self.state_machine.write().await;
            *state_machine = updated_state_machine;
        }

        // Update current snapshot.
        let mut current_snapshot = self.current_snapshot.write().await;
        *current_snapshot = Some(new_snapshot);
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_current_snapshot(
        &mut self,
    ) -> Result<
        Option<openraft::Snapshot<crate::NodeId, crate::Node, Self::SnapshotData>>,
        openraft::StorageError<crate::NodeId>,
    > {
        match &*self.current_snapshot.read().await {
            Some(snapshot) => {
                let data = snapshot.data.clone();
                Ok(Some(openraft::Snapshot {
                    meta: snapshot.meta.clone(),
                    snapshot: Box::new(std::io::Cursor::new(data)),
                }))
            }
            None => Ok(None),
        }
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.clone()
    }
}
