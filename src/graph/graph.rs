use std::collections::HashMap;

struct Node<T> {
    data: T,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data }
    }
}

// invariant: id is monotonically increasing
struct Graph<T> {
    neighborlist: HashMap<u64, Vec<Node<T>>>,
    next_id: u64,
}

impl<T> Graph<T> {
    fn new() -> Self {
        return Graph {
            neighborlist: HashMap::new(),
            next_id: 0,
        };
    }
    fn insert_node(&mut self, n: Node<T>) {
        self.neighborlist
            .entry(self.next_id)
            .or_insert(Vec::new())
            .push(n);
        self.next_id += 1;
    }
    fn pop_node(&mut self, id: u64) -> Option<T> {
        self.neighborlist.remove(&id)
    }
}
