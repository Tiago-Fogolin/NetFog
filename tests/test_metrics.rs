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

#[test]
fn test_node_count() {
    let mut g = _Graph::default();
    g.add_node("A".to_string());
    g.add_node("B".to_string());
    g.add_node("C".to_string());
    g.add_node("D".to_string());
    g.add_node("D".to_string()); // shouldnt be able to place 2 nodes with same label

    assert_eq!(4, g.get_node_count());
}

#[test]
fn test_edge_count() {
    let mut g = _Graph::default();
    g.add_node("A".to_string());
    g.add_node("B".to_string());
    g.add_node("C".to_string());
    g.add_node("D".to_string());

    g.create_connection("A".to_string(), "B".to_string(), 2., Some(false));
    g.create_connection("B".to_string(), "C".to_string(), 1., Some(false));
    g.create_connection("D".to_string(), "A".to_string(), 1., Some(false));

    assert_eq!(3, g.get_edge_count());
}

#[test]
fn test_density() {
    let mut grafo = _Graph::default();
    grafo.add_node("node1".to_string());
    grafo.add_node("node2".to_string());
    grafo.add_node("node3".to_string());
    grafo.add_node("node4".to_string());

    
    grafo.create_connection("node1".to_string(), "node2".to_string(), 2., Some(false));
    grafo.create_connection("node3".to_string(), "node4".to_string(), 4., Some(true));
    grafo.create_connection("node4".to_string(), "node1".to_string(), 5.5, Some(false));
    grafo.create_connection("node3".to_string(), "node2".to_string(), 1.2, Some(true));
    grafo.create_connection("node2".to_string(), "node3".to_string(), 1.6, Some(false));

    let expected_density = (1. * 5.) / (4. * (4. - 1.));
    assert_eq!(expected_density, grafo.get_density(Some(false)));

    let expected_density_directed = (2. * 5.) / (4. * (4. - 1.));
    assert_eq!(expected_density_directed, grafo.get_density(Some(true)));
}