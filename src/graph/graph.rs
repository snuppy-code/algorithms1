use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
    ops::Deref,
    usize,
};

use thiserror::Error;

use crate::graph::graph::MissingNodesError::{U, UV};

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
        if self.contains(u_id) {
            Err(InsertNodeError(u_id))
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
        self.ensure_uv(u_id, v_id)?;
        let u_entry = self.entry_mut_by_id(u_id).expect("just ensured u exists");
        u_entry.neighbors.insert(v_id);
        Ok(())
    }
    pub fn insert_bi_edge(&mut self, u_id: NodeID, v_id: NodeID) -> Result<(), InsertBiEdgeError> {
        let (u_entry, v_entry) = self.two_entries_mut_by_id(u_id, v_id)?;

        u_entry.neighbors.insert(v_id);
        v_entry.neighbors.insert(u_id);
        Ok(())
    }
    pub fn remove_node(&mut self, u_id: NodeID) -> Result<NodeEntry<T>, RemoveNodeError> {
        let u_entry = self
            .neighbor_sets
            .remove(&u_id)
            .ok_or(RemoveNodeError(MissingUError(u_id)))?;

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
        for u_id in self
            .neighbors_ref(s_id)
            .expect("the graph should not have been mutated since we pushed this node")
        {
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
            let u_neighbors = self
                .neighbors_ref(u_id)
                .expect("the graph should not have been mutated since we pushed this node");

            visited.insert(u_id);
            component.insert(u_id);

            stack.extend(u_neighbors.iter().filter(|v_id| !visited.contains(v_id)));
        }

        component
    }

    pub fn get_component_size(&self, s_id: NodeID) -> Result<usize, GetComponentSizeError> {
        self.ensure_u(s_id)?;
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

    pub fn arbitrary_member_id(&self) -> Option<NodeID> {
        Some(*self.neighbor_sets.iter().next()?.0)
    }

    pub fn is_connected(&self) -> bool {
        let Some(s_id) = self.arbitrary_member_id() else {
            return true;
        };
        let s_component_size = self
            .get_component_size(s_id)
            .expect("this part of the function can't run if s isn't in the graph");

        s_component_size == self.neighbor_sets.len()
    }

    pub fn shortest_path(
        &self,
        s_id: NodeID,
        e_id: NodeID,
    ) -> Result<Vec<NodeID>, GetShortestPathError> {
        let (s_entry, e_entry) = self.two_entries_by_id(s_id, e_id)?;

        todo!()
    }
    fn shortest_path_bfs_visit(
        &self,
        s_id: NodeID,
        e_id: NodeID,
    ) -> Result<Vec<NodeID>, GetShortestPathError> {
        todo!();
    }

    pub fn contains(&self, u_id: NodeID) -> bool {
        self.neighbor_sets.contains_key(&u_id)
    }

    pub fn ensure_u(&self, u_id: NodeID) -> Result<(), MissingUError> {
        match self.neighbor_sets.contains_key(&u_id) {
            true => Ok(()),
            false => Err(MissingUError(u_id)),
        }
    }
    pub fn ensure_v(&self, v_id: NodeID) -> Result<(), MissingVError> {
        match self.neighbor_sets.contains_key(&v_id) {
            true => Ok(()),
            false => Err(MissingVError(v_id)),
        }
    }
    pub fn ensure_uv(&self, u_id: NodeID, v_id: NodeID) -> Result<(), MissingNodesError> {
        match (
            self.neighbor_sets.contains_key(&u_id),
            self.neighbor_sets.contains_key(&v_id),
        ) {
            (true, true) => Ok(()),
            (false, true) => Err(MissingNodesError::U(MissingUError(u_id))),
            (true, false) => Err(MissingNodesError::V(MissingVError(v_id))),
            (false, false) => Err(MissingNodesError::UV(MissingUVError { u_id, v_id })),
        }
    }

    pub fn entry_by_id(&self, u_id: NodeID) -> Result<&NodeEntry<T>, MissingUError> {
        self.neighbor_sets.get(&u_id).ok_or(MissingUError(u_id))
    }
    pub fn entry_mut_by_id(&mut self, u_id: NodeID) -> Result<&mut NodeEntry<T>, MissingUError> {
        self.neighbor_sets.get_mut(&u_id).ok_or(MissingUError(u_id))
    }
    pub fn two_entries_by_id(
        &self,
        u_id: NodeID,
        v_id: NodeID,
    ) -> Result<(&NodeEntry<T>, &NodeEntry<T>), MissingNodesError> {
        let u_maybe = self.entry_by_id(u_id);
        let v_maybe: Result<&NodeEntry<T>, MissingVError> =
            self.entry_by_id(v_id).map_err(|e| e.into());

        require_uv_entries(u_maybe, v_maybe)
    }
    pub fn two_entries_mut_by_id(
        &mut self,
        u_id: NodeID,
        v_id: NodeID,
    ) -> Result<(&mut NodeEntry<T>, &mut NodeEntry<T>), MissingNodesError> {
        let [u_maybe, v_maybe] = self.neighbor_sets.get_disjoint_mut([&u_id, &v_id]);

        require_uv_entries_mut(
            u_maybe.ok_or(MissingUError(u_id)),
            v_maybe.ok_or(MissingVError(v_id)),
        )
    }
}

#[derive(Debug, Error)]
#[error("failed to insert node")]
pub struct InsertNodeError(#[from] MissingUError);

#[derive(Debug, Error)]
#[error("failed to remove node")]
pub struct RemoveNodeError(#[from] MissingUError);

#[derive(Debug, Error)]
#[error("failed to get size of the component that u is in")]
pub struct GetComponentSizeError(#[from] MissingUError);

#[derive(Debug, Error)]
#[error("failed to insert bidirectional edge between u and v")]
pub struct InsertBiEdgeError(#[from] MissingNodesError);

#[derive(Debug, Error)]
#[error("failed to insert edge from u to v")]
pub struct InsertEdgeError(#[from] MissingNodesError);

#[derive(Debug, Error)]
#[error("failed to get shortest path")]
pub struct GetShortestPathError(#[from] MissingNodesError);

#[derive(Debug, Error)]
#[error("missing node(s)")]
pub enum MissingNodesError {
    UV(#[from] MissingUVError),
    U(#[from] MissingUError),
    V(#[from] MissingVError),
}

#[derive(Debug, Error)]
#[error("u: {0} was not found in the graph")]
pub struct MissingUError(NodeID);

impl From<MissingVError> for MissingUError {
    fn from(value: MissingVError) -> Self {
        MissingUError(value.0)
    }
}

#[derive(Debug, Error)]
#[error("v: {0} was not found in the graph")]
pub struct MissingVError(NodeID);

impl From<MissingUError> for MissingVError {
    fn from(value: MissingUError) -> Self {
        MissingVError(value.0)
    }
}

#[derive(Debug, Error)]
#[error("u: {u_id} and v: {v_id} were not found in the graph")]
pub struct MissingUVError {
    u_id: NodeID,
    v_id: NodeID,
}

pub fn require_uv_entries<'a, 'b, T>(
    maybe_u: Result<&'a NodeEntry<T>, MissingUError>,
    maybe_v: Result<&'b NodeEntry<T>, MissingVError>,
) -> Result<(&'a NodeEntry<T>, &'b NodeEntry<T>), MissingNodesError> {
    match (maybe_u, maybe_v) {
        (Ok(u_entry), Ok(v_entry)) => Ok((u_entry, v_entry)),
        (Err(u_err), Ok(_)) => Err(MissingNodesError::U(u_err)),
        (Ok(_), Err(v_err)) => Err(MissingNodesError::V(v_err)),
        (Err(MissingUError(u_id)), Err(MissingVError(v_id))) => {
            Err(MissingNodesError::UV(MissingUVError { u_id, v_id }))
        }
    }
}
pub fn require_uv_entries_mut<'a, 'b, T>(
    maybe_u: Result<&'a mut NodeEntry<T>, MissingUError>,
    maybe_v: Result<&'b mut NodeEntry<T>, MissingVError>,
) -> Result<(&'a mut NodeEntry<T>, &'b mut NodeEntry<T>), MissingNodesError> {
    match (maybe_u, maybe_v) {
        (Ok(u_entry), Ok(v_entry)) => Ok((u_entry, v_entry)),
        (Err(u_err), Ok(_)) => Err(MissingNodesError::U(u_err)),
        (Ok(_), Err(v_err)) => Err(MissingNodesError::V(v_err)),
        (Err(MissingUError(u_id)), Err(MissingVError(v_id))) => {
            Err(MissingNodesError::UV(MissingUVError { u_id, v_id }))
        }
    }
}
