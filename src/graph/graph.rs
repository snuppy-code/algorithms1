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
    neighbor_sets: HashMap<NodeID, (Node<T>, HashSet<NodeID>)>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        return Graph {
            neighbor_sets: HashMap::new(),
        };
    }
    pub fn insert_node(&mut self, n: Node<T>) -> Result<NodeID, InsertNodeError> {
        let id = n.id;
        if self.neighbor_sets.contains_key(&id) {
            Err(InsertNodeError::ExistingID(id))
        } else {
            debug_assert!(
                self.neighbor_sets
                    .insert(n.id, (n, HashSet::new()))
                    .is_none()
            );
            Ok(id)
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

        let (u, u_neighbor_set) = self.neighbor_sets.get_mut(&u_id).expect("we just checked");
        debug_assert_eq!(u.get_id(), u_id);
        u_neighbor_set.insert(v_id);

        let (v, v_neighbor_set) = self.neighbor_sets.get_mut(&v_id).expect("we just checked");
        debug_assert_eq!(v.get_id(), v_id);
        v_neighbor_set.insert(u_id);
        Ok(())
    }
    pub fn node_ref_by_id(&self, u_id: NodeID) -> Option<&Node<T>> {
        self.neighbor_sets.get(&u_id).map(|(u, _)| u)
    }
    pub fn neighbors(&self, u_id: NodeID) -> Option<&HashSet<NodeID>> {
        self.neighbor_sets
            .get(&u_id)
            .map(|(_, neighbors)| neighbors)
    }
    pub fn components(&self) -> HashSet<HashSet<NodeID>> {
        todo!();
    }
    fn components_dfs_full(&self) -> HashSet<HashSet<NodeID>> {
        todo!();
    }
    fn components_dfs_visit(&self) -> HashSet<HashSet<NodeID>> {
        todo!();
    }
    pub fn components_overview(&self) -> usize {
        todo!();
    }
    pub fn is_connected(&self) -> bool {
        todo!();
    }
}

#[derive(Debug, Error)]
pub enum InsertNodeError {
    #[error(
        "failed to insert node with id: {0}, because a node with that ID is already in the graph"
    )]
    ExistingID(NodeID),
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
