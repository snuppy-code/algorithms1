use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct NodeEntry<T> {
    pub(super) node: Node<T>,
    pub(super) neighbors: HashSet<NodeID>,
}

/// Directed, unweighted graph.
pub struct Graph<T> {
    pub(super) neighbor_sets: HashMap<NodeID, NodeEntry<T>>,
    pub(super) id_generator: NodeIDGenerator,
}

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    id: NodeID,
}
impl<T> Node<T> {
    pub fn new_node(data: T, id: NodeID) -> Node<T> {
        Node { data, id }
    }
    pub fn id(&self) -> NodeID {
        self.id
    }
}

use std::fmt::Display;

// invariant: next_id is monotonically increasing
pub struct NodeIDGenerator {
    next_id: u64,
}
impl NodeIDGenerator {
    pub fn new() -> Self {
        NodeIDGenerator { next_id: 0 }
    }
    pub fn create_id(&mut self) -> NodeID {
        self.next_id += 1;
        NodeID(self.next_id)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct NodeID(u64);

impl Display for NodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeID({})", self.0)
    }
}
