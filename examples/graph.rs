use algorithms::graph::graph::{Graph, Node, NodeID, NodeIDGenerator};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut nodeid_gen = NodeIDGenerator::new();
    let mut g = Graph::new();

    let n1 = nodeid_gen.create_id();
    g.insert_node(Node::new_node("joanne", n1))?;
    let n2 = g.insert_node(Node::new_node("bloanne", nodeid_gen.create_id()))?;
    let n3 = g.insert_node(Node::new_node("schmoanne", nodeid_gen.create_id()))?;

    g.insert_edge(n1, n2)?;
    g.insert_edge(n1, n3)?;
    g.insert_edge(n2, n3)?;

    let n_not_in = nodeid_gen.create_id(); // nodeid is not used by a node in g
    // g.insert_edge(n1, n_not_in)?;

    println!("{}: '{}'", n3, g.node_ref(n3).unwrap().data);

    Ok(())
}
