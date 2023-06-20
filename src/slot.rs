use std::vec;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Slots {
    inner: Vec<crate::Node>,
}

impl Default for Slots {
    fn default() -> Self {
        Self {
            inner: vec![crate::Node::Unknow; 10000],
        }
    }
}

impl Slots {
    pub fn check(&self) -> bool {
        for node in self.inner.iter() {
            if let crate::Node::Compute(_) = node {
                continue;
            } else {
                return false;
            }
        }
        true
    }

    pub fn move_to(&mut self, node: &crate::Node, slots: &[usize]) {
        if self.inner.capacity() < 10000 {
            self.inner.truncate(10000)
        }
        for slot in slots.iter() {
            // FIXME: panic if out of index
            self.inner[*slot] = node.clone();
        }
    }

    pub fn with_node(&self) -> std::collections::HashMap<String, std::collections::HashSet<usize>> {
        let mut node_slots: std::collections::HashMap<
            crate::Node,
            std::collections::HashSet<usize>,
        > = std::collections::HashMap::new();

        for (slot, node) in self.inner.iter().enumerate() {
            node_slots.entry(node.clone()).or_default().insert(slot);
        }
        node_slots
            .into_iter()
            .map(|(node, slots)| (serde_json::to_string(&node).unwrap(), slots))
            .collect()
    }
}
