use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
    usize,
};

use thiserror::Error;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeID(u64);

impl Display for NodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeID({})", self.0)
    }
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
    pub fn get_id(&self) -> NodeID {
        self.id
    }
}

pub struct Graph<T> {
    neighbor_sets: HashMap<NodeID, NodeEntry<T>>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        return Graph {
            neighbor_sets: HashMap::new(),
        };
    }
    pub fn insert_node(&mut self, u: Node<T>) -> Result<NodeID, InsertNodeError> {
        let u_id = u.id;
        if self.neighbor_sets.contains_key(&u_id) {
            Err(InsertNodeError::ExistingID(u_id))
        } else {
            debug_assert!(
                self.neighbor_sets
                    .insert(
                        u.id,
                        NodeEntry {
                            node: u,
                            neighbors: HashSet::new()
                        }
                    )
                    .is_none()
            );
            Ok(u_id)
        }
    }
    pub fn insert_edge(&mut self, u_id: NodeID, v_id: NodeID) -> Result<(), InsertEdgeError> {
        let contains_u = self.neighbor_sets.contains_key(&u_id);
        let contains_v = self.neighbor_sets.contains_key(&v_id);

        if !(contains_u && contains_v) {
            return Err(InsertEdgeError::MissingNodes {
                u: InGraph {
                    id: u_id,
                    was: contains_u,
                },
                v: InGraph {
                    id: v_id,
                    was: contains_v,
                },
            });
        }

        let NodeEntry {
            node: u,
            neighbors: u_neighbors,
        } = self.neighbor_sets.get_mut(&u_id).expect("we just checked");
        debug_assert_eq!(u.get_id(), u_id);
        u_neighbors.insert(v_id);

        let NodeEntry {
            node: v,
            neighbors: v_neighbor_set,
        } = self.neighbor_sets.get_mut(&v_id).expect("we just checked");
        debug_assert_eq!(v.get_id(), v_id);
        v_neighbor_set.insert(u_id);

        Ok(())
    }
    pub fn remove_node(&mut self, u_id: NodeID) -> Result<NodeEntry<T>, RemoveNodeError> {
        let u_entry = self
            .neighbor_sets
            .remove(&u_id)
            .ok_or(RemoveNodeError::NoSuchNode(u_id))?;

        for (_, v_entry) in self.neighbor_sets.iter_mut() {
            v_entry.neighbors.remove(&u_id);
        }

        Ok(u_entry)
    }
    pub fn node_ref(&self, u_id: NodeID) -> Option<&Node<T>> {
        self.neighbor_sets.get(&u_id).map(|entry| &entry.node)
    }
    pub fn neighbors_ref(&self, u_id: NodeID) -> Option<&HashSet<NodeID>> {
        self.neighbor_sets.get(&u_id).map(|entry| &entry.neighbors)
    }
    pub fn components_dfs_full(&self) -> HashSet<BTreeSet<NodeID>> {
        let mut components = HashSet::new();
        let mut visited = HashSet::new();

        for (u_id, _) in &self.neighbor_sets {
            if visited.contains(&u_id) {
                continue;
            }
            visited.insert(u_id);
            components.insert(self.component_dfs_visit(*u_id));
        }

        components
    }
    fn component_dfs_visit(&self, u_id: NodeID) -> BTreeSet<NodeID> {
        todo!();
    }
    pub fn components_overview(&self) -> usize {
        todo!();
    }
    pub fn is_connected(&self) -> bool {
        todo!();
    }
}

#[derive(Debug)]
pub struct NodeEntry<T> {
    node: Node<T>,
    neighbors: HashSet<NodeID>,
}

#[derive(Debug, Error)]
pub enum InsertNodeError {
    #[error(
        "failed to insert node with id: {0}, because a node with that ID is already in the graph"
    )]
    ExistingID(NodeID),
}
#[derive(Debug, Error)]
pub enum RemoveNodeError {
    #[error("failed to remove node with id: {0}, because no node with that ID exists in the graph")]
    NoSuchNode(NodeID),
}

#[derive(Debug, Error)]
pub enum InsertEdgeError {
    MissingNodes { u: InGraph, v: InGraph },
}
#[derive(Debug)]
pub struct InGraph {
    id: NodeID,
    was: bool,
}

impl Display for InsertEdgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingNodes { u, v } => match (u, v) {
                (InGraph { id: u, was: true }, InGraph { id: v, was: true }) => write!(
                    f,
                    "failed to insert edge u: {u} -> v: {v} because u: {u}, v: {v} were not in the graph"
                ),
                (InGraph { id: u, was: true }, InGraph { id: v, was: false }) => write!(
                    f,
                    "failed to insert edge u: {u} -> v: {v} because u: {u} was not in the graph"
                ),
                (InGraph { id: u, was: false }, InGraph { id: v, was: true }) => write!(
                    f,
                    "failed to insert edge u: {u} -> v: {v} because v: {v} was not in the graph",
                ),
                (InGraph { id: u, was: false }, InGraph { id: v, was: false }) => write!(
                    f,
                    "failed to insert edge u: {u} -> v: {v} because of.. an impossible error! report to library"
                ),
            },
        }
    }
}
