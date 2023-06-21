use std::vec;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Slots {
    inner: Vec<crate::Node>,
}

impl Default for Slots {
    fn default() -> Self {
        Self {
            inner: vec![crate::Node::Unknow; 100],
        }
    }
}

impl Slots {
    pub fn get_node(&self, slot: usize) -> Option<&crate::Node> {
        self.inner.get(slot)
    }

    pub fn get_nodes(
        &self,
        slots: Vec<usize>,
    ) -> std::collections::HashMap<&crate::Node, Vec<usize>> {
        let mut nodes: std::collections::HashMap<&crate::node::Node, Vec<usize>> =
            std::collections::HashMap::new();
        slots
            .iter()
            .filter_map(|slot| self.get_node(*slot))
            .enumerate()
            .for_each(|(slot, node)| {
                nodes.entry(node).or_default().push(slot);
            });
        nodes
    }

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

    pub fn with_node(&self) -> std::collections::HashMap<u64, std::collections::HashSet<usize>> {
        let mut node_slots: std::collections::HashMap<
            crate::Node,
            std::collections::HashSet<usize>,
        > = std::collections::HashMap::new();

        for (slot, node) in self.inner.iter().enumerate() {
            node_slots.entry(node.clone()).or_default().insert(slot);
        }
        node_slots
            .into_iter()
            .map(|(node, slots)| (node.get_id().unwrap(), slots))
            .collect()
    }

    pub fn from_position(position: &crate::Vector2) -> usize {
        // Int((x + 100000) / 2000) + (100 * Int((y + 100000) / 2000)).
        let crate::Vector2 { x, y } = position;

        let mut x = x + 100000.0;
        if x < 0.0 {
            x = 0.0;
        } else if x > 200000.0 {
            x = 200000.0;
        }

        let x = (x / 20000.0) as usize;

        let mut y = y + 100000.0;
        if y == 200000.0 {
            y = 190000.0;
        } else if y < 0.0 {
            y = 0.0;
        } else if y > 200000.0 {
            y = 190000.0;
        }
        let y = (y / 20000.0) as usize;
        let y = 10 * y;
        x + y
    }

    // (12 34)
    // 32 33 34
    // 22 23 24
    // 12 13 14
    pub fn from_area(min: &crate::Vector2, max: &crate::Vector2) -> Vec<usize> {
        let mut res = Vec::new();
        let mut min = Slots::from_position(&min);
        let mut max = Slots::from_position(&max);

        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        let min_x = min % 10;
        let min_y = min / 10;

        let max_x = max % 10;
        let max_y = max / 10;

        for i in 0..=(max_y - min_y) {
            let start = min + i * 10;
            res.extend(start..=(start + (max_x - min_x)));
        }

        res
    }

    pub fn from_radius(position: &crate::Vector2, radius: f32) -> Vec<usize> {
        let max = crate::Vector2 {
            x: position.x + radius,
            y: position.y + radius,
        };
        let min = crate::Vector2 {
            x: position.x - radius,
            y: position.y - radius,
        };
        Self::from_area(&min, &max)
    }
}

#[cfg(test)]
mod test {
    use super::Slots;
    use crate::Vector2;

    #[test]
    fn from_position() {
        let position = Vector2 { x: 0.0, y: 0.0 };
        assert_eq!(55, Slots::from_position(&position));
        let position = Vector2 {
            x: -100000.0,
            y: -100000.0,
        };
        assert_eq!(0, Slots::from_position(&position));
        let position = Vector2 {
            x: -200000.0,
            y: -200000.0,
        };
        assert_eq!(0, Slots::from_position(&position));
        let position = Vector2 {
            x: 100000.0,
            y: -100000.0,
        };
        assert_eq!(10, Slots::from_position(&position));
        let position = Vector2 {
            x: 100000.0,
            y: 100000.0,
        };
        assert_eq!(100, Slots::from_position(&position));
        let position = Vector2 {
            x: 200000.0,
            y: 200000.0,
        };
        assert_eq!(100, Slots::from_position(&position));
        let position = Vector2 {
            x: 100000.0,
            y: 99999.0,
        };
        assert_eq!(100, Slots::from_position(&position));
        let position = Vector2 {
            x: 100000.0,
            y: 70000.0,
        };
        assert_eq!(90, Slots::from_position(&position));
        let position = Vector2 {
            x: 19000.0,
            y: 19000.0,
        };
        assert_eq!(55, Slots::from_position(&position));
    }

    #[test]
    fn from_area() {
        let min = Vector2 {
            x: -68000.0,
            y: -78000.0,
        };
        let max = Vector2 {
            x: -19000.0,
            y: -36000.0,
        };
        let res = Slots::from_area(&min, &max);
        assert_eq!(vec![11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34], res);
    }
}
