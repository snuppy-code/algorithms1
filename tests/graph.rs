use algorithms::graph::graph_types::Graph;

#[test]
fn basic_usage() {
    let mut g = Graph::new();

    let n1_id = g.add_node("joanne");
    let n2_id = g.add_node("gloanne");
    let n3_id = g.add_node("bloanne");
    let n4_id = g.add_node("moanne");
    let n5_id = g.add_node("schloanne");
    let n6_id = g.add_node("gloglanne");

    g.add_bi_edge(n1_id, n2_id);
    g.add_bi_edge(n2_id, n3_id);
    g.add_edge(n1_id, n4_id);
    g.add_bi_edge(n5_id, n6_id);

    let x = g.remove_node(n6_id);

    g.add_edge(n1_id, x.unwrap().id());

    // println!("{}: '{}'", n3_id, g.node_ref(n3_id).unwrap().data);
    println!("{:?}", g.components_dfs_recursive());
    println!("{:?}", g.components_dfs_iterative());
    println!("{:?}", g.components_sizes());
    println!("{:?}", g.get_component_size(n1_id));
    println!("{:?}", g.get_component_size(n5_id));
    println!("{:?}", g.is_connected());
}
