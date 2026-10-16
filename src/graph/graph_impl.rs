use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use thiserror::Error;

use crate::graph::graph_types::{Graph, Node, NodeEntry, NodeID, NodeIDGenerator};

#[derive(PartialEq, Eq)]
struct ExplicitOrdNodeID {
    ord: usize,
    id: NodeID,
}

impl Ord for ExplicitOrdNodeID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ord.cmp(&other.ord)
    }
}
impl PartialOrd for ExplicitOrdNodeID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ord.cmp(&other.ord))
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        return Graph {
            neighbor_sets: HashMap::new(),
            id_generator: NodeIDGenerator::new(),
        };
    }
    pub fn add_node(&mut self, data: T) -> NodeID {
        let u = Node::new_node(data, self.id_generator.create_id());
        let u_id = u.id();
        self.neighbor_sets.insert(
            u.id(),
            NodeEntry {
                node: u,
                neighbors: HashSet::new(),
            },
        );
        u_id
    }
    pub fn add_edge(&mut self, u_id: NodeID, v_id: NodeID) -> bool {
        if !(self.contains(u_id) && self.contains(v_id)) {
            return false;
        }
        self.entry_mut_by_id(u_id)
            .expect("held borrow since we checked u exists")
            .neighbors
            .insert(v_id)
    }
    pub fn add_bi_edge(&mut self, u_id: NodeID, v_id: NodeID) -> bool {
        if !(self.contains(u_id) && self.contains(v_id)) {
            return false;
        }
        let [u_entry, v_entry] = self
            .neighbor_sets
            .get_disjoint_mut([&u_id, &v_id])
            .map(|o| o.expect("held borrow since we checked u,v exist"));

        let mut did_anything = false;

        did_anything |= u_entry.neighbors.insert(v_id);
        did_anything |= v_entry.neighbors.insert(u_id);

        did_anything
    }
    pub fn remove_node(&mut self, u_id: NodeID) -> Option<Node<T>> {
        let u_entry = self.neighbor_sets.remove(&u_id).map(|e| e.node);

        for (_, v_entry) in self.neighbor_sets.iter_mut() {
            v_entry.neighbors.remove(&u_id);
        }

        u_entry
    }
    pub fn node_ref(&self, u_id: NodeID) -> Option<&Node<T>> {
        self.neighbor_sets.get(&u_id).map(|entry| &entry.node)
    }
    pub fn neighbors_ref(&self, u_id: NodeID) -> Option<&HashSet<NodeID>> {
        self.neighbor_sets.get(&u_id).map(|entry| &entry.neighbors)
    }

    pub fn arbitrary_member_id(&self) -> Option<NodeID> {
        Some(*self.neighbor_sets.iter().next()?.0)
    }

    pub fn contains(&self, u_id: NodeID) -> bool {
        self.neighbor_sets.contains_key(&u_id)
    }

    pub fn contains_uv(&self, u_id: NodeID, v_id: NodeID) -> bool {
        self.neighbor_sets.contains_key(&u_id) && self.neighbor_sets.contains_key(&v_id)
    }

    pub fn entry_by_id(&self, u_id: NodeID) -> Option<&NodeEntry<T>> {
        self.neighbor_sets.get(&u_id)
    }
    pub fn entry_mut_by_id(&mut self, u_id: NodeID) -> Option<&mut NodeEntry<T>> {
        self.neighbor_sets.get_mut(&u_id)
    }
    pub fn two_entries_by_id(
        &self,
        u_id: NodeID,
        v_id: NodeID,
    ) -> Option<(&NodeEntry<T>, &NodeEntry<T>)> {
        let u_entry = self.entry_by_id(u_id)?;
        let v_entry = self.entry_by_id(v_id)?;

        Some((u_entry, v_entry))
    }
    pub fn two_entries_mut_by_id(
        &mut self,
        u_id: NodeID,
        v_id: NodeID,
    ) -> Option<(&mut NodeEntry<T>, &mut NodeEntry<T>)> {
        let [u_maybe, v_maybe] = self.neighbor_sets.get_disjoint_mut([&u_id, &v_id]);
        Some((u_maybe?, v_maybe?))
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

    pub fn get_component_size(&self, s_id: NodeID) -> Result<usize, NoSuchNodeError> {
        if !self.contains(s_id) {
            return Err(NoSuchNodeError);
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
        while let Some(u_id) = stack.pop() {
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

    pub fn is_connected(&self) -> bool {
        let Some(s_id) = self.arbitrary_member_id() else {
            return true;
        };
        let s_component_size = self
            .get_component_size(s_id)
            .expect("this part of the function can't run if s isn't in the graph");

        s_component_size == self.neighbor_sets.len()
    }

    /// bfs visit iterative
    pub fn shortest_path(
        &self,
        s_id: NodeID,
        e_id: NodeID,
    ) -> Result<Vec<NodeID>, NoSuchNodesError> {
        if !(self.contains(s_id) && self.contains(e_id)) {
            return Err(NoSuchNodesError);
        }
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push(ExplicitOrdNodeID { id: s_id, ord: 0 });
        while let Some(ExplicitOrdNodeID { id: u_id, ord }) = queue.pop() {
            visited.insert(u_id);
            for n in self.neighbors_ref(u_id).expect("should have neighbors") {
                todo!()
            }
        }
        todo!();
    }
}

#[derive(Debug, Error)]
#[error("no such node in the graph")]
pub struct NoSuchNodeError;

#[derive(Debug, Error)]
#[error("no such nodes in the graph")]
pub struct NoSuchNodesError;
