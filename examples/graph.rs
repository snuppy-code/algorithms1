use algorithms::graph::graph::{Graph, Node, NodeID, NodeIDGenerator};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut nodeid_gen = NodeIDGenerator::new();
    let mut g = Graph::new();

    let n1_id = g.insert_node(Node::new_node("joanne", nodeid_gen.create_id()))?;
    let n2_id = g.insert_node(Node::new_node("gloanne", nodeid_gen.create_id()))?;
    let n3_id = g.insert_node(Node::new_node("bloanne", nodeid_gen.create_id()))?;
    let n4_id = g.insert_node(Node::new_node("moanne", nodeid_gen.create_id()))?;
    let n5_id = g.insert_node(Node::new_node("schloanne", nodeid_gen.create_id()))?;
    let n6_id = g.insert_node(Node::new_node("gloglanne", nodeid_gen.create_id()))?;

    g.insert_bi_edge(n1_id, n2_id)?;
    g.insert_bi_edge(n2_id, n3_id)?;
    g.insert_edge(n1_id, n4_id)?;
    g.insert_bi_edge(n5_id, n6_id)?;

    // let n_not_in = nodeid_gen.create_id(); // nodeid is not used by a node in g
    // g.insert_edge(n1, n_not_in)?;

    // println!("{}: '{}'", n3_id, g.node_ref(n3_id).unwrap().data);
    println!("{:?}", g.components_dfs_recursive());
    println!("{:?}", g.components_dfs_iterative());
    println!("{:?}", g.components_sizes());
    println!("{:?}", g.get_component_size(n1_id)?);
    println!("{:?}", g.get_component_size(n5_id)?);
    println!("{:?}", g.is_connected());

    Ok(())
}
