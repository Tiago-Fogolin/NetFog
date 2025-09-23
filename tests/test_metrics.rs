use netfog::*;


#[test]
fn test_total_weight() {
    let mut graph = _Graph::default();
    graph.add_node("node1".to_string());
    graph.add_node("node2".to_string());
    graph.add_node("node3".to_string());
    graph.add_node("node4".to_string());

    graph.create_connection("node1".to_string(), "node2".to_string(), 2., None);
    graph.create_connection("node3".to_string(), "node4".to_string(), 4., None);
    graph.create_connection("node4".to_string(), "node1".to_string(), 5.5, None);
    graph.create_connection("node3".to_string(), "node2".to_string(), 1.2, None);
    graph.create_connection("node2".to_string(), "node3".to_string(), 1.6, None);


    let total_weight = 2.0 + 4.0 + 5.5 + 1.2 + 1.6;
    assert_eq!(total_weight, graph.get_total_weight());
}

#[test]
fn test_mean_weigt() {
    let mut grafo = _Graph::default();
    grafo.add_node("node1".to_string());
    grafo.add_node("node2".to_string());
    grafo.add_node("node3".to_string());
    grafo.add_node("node4".to_string());

    grafo.create_connection("node1".to_string(), "node2".to_string(), 2., None);
    grafo.create_connection("node3".to_string(), "node4".to_string(), 4., None);
    grafo.create_connection("node4".to_string(), "node1".to_string(), 5.5, None);
    grafo.create_connection("node3".to_string(), "node2".to_string(), 1.2, None);
    grafo.create_connection("node2".to_string(), "node3".to_string(), 1.6, None);

    let mean = (2. + 4. + 5.5 + 1.2 + 1.6)/5.;

    assert_eq!(mean, grafo.get_mean_weight());
}