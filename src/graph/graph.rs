use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
    ops::Deref,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

#[derive(Debug)]
pub struct NodeEntry<T> {
    node: Node<T>,
    neighbors: HashSet<NodeID>,
}

/// Directed, unweighted graph.
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
    fn prep_insert_edge(
        &mut self,
        u_id: NodeID,
        v_id: NodeID,
    ) -> Result<(&mut NodeEntry<T>, &mut NodeEntry<T>), InsertEdgeError> {
        let contains_u = self.neighbor_sets.contains_key(&u_id);
        let contains_v = self.neighbor_sets.contains_key(&v_id);

        MissingNodesError::if_missing(contains_u, u_id, contains_v, v_id)?;

        let [u_entry, v_entry] = self
            .neighbor_sets
            .get_disjoint_mut([&u_id, &v_id])
            .map(|v| {
                v.expect(
                    "we have held a mutable borrow on self since we checked that these nodes exist",
                )
            });

        Ok((u_entry, v_entry))
    }
    pub fn insert_edge(&mut self, u_id: NodeID, v_id: NodeID) -> Result<(), InsertEdgeError> {
        let (u, _) = self.prep_insert_edge(u_id, v_id)?;
        u.neighbors.insert(v_id);
        Ok(())
    }
    pub fn insert_bi_edge(&mut self, u_id: NodeID, v_id: NodeID) -> Result<(), InsertEdgeError> {
        let (u, v) = self.prep_insert_edge(u_id, v_id)?;
        u.neighbors.insert(v_id);
        v.neighbors.insert(u_id);
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

    pub fn components_dfs_recursive(&self) -> HashSet<BTreeSet<NodeID>> {
        let mut components = HashSet::new();
        let mut visited = HashSet::new();

        for (u_id, _) in &self.neighbor_sets {
            if visited.contains(u_id) {
                continue;
            }
            let mut component = BTreeSet::new();
            self.component_dfs_visit_recursive(*u_id, &mut visited, &mut component);
            components.insert(component);
        }

        components
    }
    fn component_dfs_visit_recursive(
        &self,
        s_id: NodeID,
        visited: &mut HashSet<NodeID>,
        component: &mut BTreeSet<NodeID>,
    ) {
        visited.insert(s_id);
        component.insert(s_id);
        for u_id in self.neighbors_ref(s_id).expect("u should be in the graph") {
            if visited.contains(u_id) {
                continue;
            }
            self.component_dfs_visit_recursive(*u_id, visited, component)
        }
    }

    pub fn components_dfs_iterative(&self) -> HashSet<BTreeSet<NodeID>> {
        let mut components = HashSet::new();
        let mut visited = HashSet::new();

        for (u_id, _) in &self.neighbor_sets {
            if visited.contains(u_id) {
                continue;
            }
            let component = self.component_dfs_visit_iterative(*u_id, &mut visited);
            components.insert(component);
        }

        components
    }
    fn component_dfs_visit_iterative(
        &self,
        s_id: NodeID,
        visited: &mut HashSet<NodeID>,
    ) -> BTreeSet<NodeID> {
        let mut stack = vec![s_id];
        let mut component = BTreeSet::new();

        while !stack.is_empty() {
            let u_id = stack.pop().expect("just checked not empty");
            let u_neighbors = self.neighbors_ref(u_id).expect("u should be in the graph");

            visited.insert(u_id);
            component.insert(u_id);

            stack.extend(u_neighbors.iter().filter(|v_id| !visited.contains(v_id)));
        }

        component
    }

    pub fn get_component_size(&self, s_id: NodeID) -> Result<usize, GetComponentSizeError> {
        if !self.neighbor_sets.contains_key(&s_id) {
            return Err(GetComponentSizeError(s_id));
        }
        let mut visited = HashSet::new();
        Ok(self.component_size_dfs_visit_iterative(s_id, &mut visited))
    }
    /// Returns a map from sizes of components to amount of such-sized components
    pub fn components_sizes(&self) -> HashMap<usize, usize> {
        let mut visited = HashSet::new();
        let mut components = HashMap::new();
        for (u_id, _) in &self.neighbor_sets {
            if visited.contains(u_id) {
                continue;
            }
            let component_size = self.component_size_dfs_visit_iterative(*u_id, &mut visited);
            components
                .entry(component_size)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
        components
    }
    // precondition: node s exists in graph
    fn component_size_dfs_visit_iterative(
        &self,
        s: NodeID,
        visited: &mut HashSet<NodeID>,
    ) -> usize {
        let mut stack = vec![s];
        let mut component_size = 0;
        while !stack.is_empty() {
            let u_id = stack.pop().expect("just checked not empty");
            let u_neighbors = self.neighbors_ref(u_id).expect("u should be in the graph");

            component_size += 1;
            visited.insert(u_id);

            for v_id in u_neighbors {
                if visited.contains(v_id) {
                    continue;
                }
                stack.push(*v_id);
            }
        }
        component_size
    }

    pub fn get_arbitrary_id(&self) -> Option<NodeID> {
        Some(*self.neighbor_sets.iter().next()?.0)
    }

    pub fn is_connected(&self) -> bool {
        let Some(s_id) = self.get_arbitrary_id() else {
            return false;
        };
        let s_component_size = self
            .get_component_size(s_id)
            .expect("s should be in the graph");

        s_component_size == self.neighbor_sets.len()
    }
}

#[derive(Debug, Error)]
#[error("failed to insert {0}, because a node with that ID is already in the graph")]
pub struct InsertNodeError(NodeID);

#[derive(Debug, Error)]
#[error("failed to remove {0}, because no node with that ID exists in the graph")]
pub struct RemoveNodeError(NodeID);

#[derive(Debug, Error)]
#[error("failed to get size of {0}'s component, because no node with that ID exists in the graph")]
pub struct GetComponentSizeError(NodeID);

#[derive(Debug, Error)]
#[error("failed to insert edge due to missing nodes")]
pub struct InsertEdgeError(#[from] MissingNodesError);

#[derive(Debug, Error)]
pub enum MissingNodesError {
    NeededUAndV(NodeID, NodeID),
    NeededU(NodeID),
    NeededV(NodeID),
}
impl Display for MissingNodesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NeededUAndV(u, v) => write!(f, "u: {u} and v: {v} were not in the graph"),
            Self::NeededU(u) => write!(f, "u: {u} was not in the graph"),
            Self::NeededV(v) => write!(f, "v: {v} was not in the graph"),
        }
    }
}
impl MissingNodesError {
    pub fn if_missing(
        u_exists: bool,
        u_id: NodeID,
        v_exists: bool,
        v_id: NodeID,
    ) -> Result<(), MissingNodesError> {
        match (u_exists, v_exists) {
            (true, true) => Ok(()),
            (true, false) => Err(MissingNodesError::NeededV(v_id)),
            (false, true) => Err(MissingNodesError::NeededU(u_id)),
            (false, false) => Err(MissingNodesError::NeededUAndV(u_id, v_id)),
        }
    }
}
