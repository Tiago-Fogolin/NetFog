use netfog::*;
use approx::assert_abs_diff_eq;


fn create_simple_graph() -> _Graph {
    let mut graph = _Graph::default();
    graph.add_node("node1".to_string());
    graph.add_node("node2".to_string());
    graph.add_node("node3".to_string());
    graph.add_node("node4".to_string());

    
    graph.create_connection("node1".to_string(), "node2".to_string(), 2., Some(false));
    graph.create_connection("node3".to_string(), "node4".to_string(), 4., Some(true));
    graph.create_connection("node4".to_string(), "node1".to_string(), 5.5, Some(false));
    graph.create_connection("node3".to_string(), "node2".to_string(), 1.2, Some(true));
    graph.create_connection("node2".to_string(), "node3".to_string(), 1.6, Some(false));

    return graph;
}

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
    let mut grafo = create_simple_graph();
    let expected_density = (1. * 5.) / (4. * (4. - 1.));
    assert_eq!(expected_density, grafo.get_density(Some(false)));

    let expected_density_directed = (2. * 5.) / (4. * (4. - 1.));
    assert_eq!(expected_density_directed, grafo.get_density(Some(true)));
}

#[test]
fn test_compute_degrees() {
    let mut grafo = create_simple_graph();
    let degrees_node1 = grafo.compute_degrees("node1");
    assert_eq!(degrees_node1["in_degree"], 0);
    assert_eq!(degrees_node1["out_degree"], 0);
    assert_eq!(degrees_node1["undirected_degree"], 2);
    assert_eq!(degrees_node1["total_degree"], 0);

    let degrees_node2 = grafo.compute_degrees("node2");
    assert_eq!(degrees_node2["in_degree"], 1);
    assert_eq!(degrees_node2["out_degree"], 0);
    assert_eq!(degrees_node2["undirected_degree"], 2);
    assert_eq!(degrees_node2["total_degree"], 1);

    let degrees_node3 = grafo.compute_degrees("node3");
    assert_eq!(degrees_node3["in_degree"], 0);
    assert_eq!(degrees_node3["out_degree"], 2);
    assert_eq!(degrees_node3["undirected_degree"], 1);
    assert_eq!(degrees_node3["total_degree"], 2);

    let degrees_node4 = grafo.compute_degrees("node4");
    assert_eq!(degrees_node4["in_degree"], 1);
    assert_eq!(degrees_node4["out_degree"], 0);
    assert_eq!(degrees_node4["undirected_degree"], 1);
    assert_eq!(degrees_node4["total_degree"], 1);
}

#[test]
fn test_all_nodes_degrees() {
    let mut grafo = create_simple_graph();
    let degrees_dict = grafo.get_all_nodes_degrees();

    let mut expected_dict = std::collections::HashMap::new();

    expected_dict.insert("node1".to_string(), {
        let mut m = std::collections::HashMap::new();
        m.insert("in_degree".to_string(), 0);
        m.insert("out_degree".to_string(), 0);
        m.insert("undirected_degree".to_string(), 2);
        m.insert("total_degree".to_string(), 0);
        m
    });

    expected_dict.insert("node2".to_string(), {
        let mut m = std::collections::HashMap::new();
        m.insert("in_degree".to_string(), 1);
        m.insert("out_degree".to_string(), 0);
        m.insert("undirected_degree".to_string(), 2);
        m.insert("total_degree".to_string(), 1);
        m
    });

    expected_dict.insert("node3".to_string(), {
        let mut m = std::collections::HashMap::new();
        m.insert("in_degree".to_string(), 0);
        m.insert("out_degree".to_string(), 2);
        m.insert("undirected_degree".to_string(), 1);
        m.insert("total_degree".to_string(), 2);
        m
    });

    expected_dict.insert("node4".to_string(), {
        let mut m = std::collections::HashMap::new();
        m.insert("in_degree".to_string(), 1);
        m.insert("out_degree".to_string(), 0);
        m.insert("undirected_degree".to_string(), 1);
        m.insert("total_degree".to_string(), 1);
        m
    });

    assert_eq!(degrees_dict, expected_dict);
}

#[test]
fn test_get_centrality_degree() {
    let mut grafo = create_simple_graph();

    let centralities = grafo.get_centrality_degrees("node1");
    assert_eq!(centralities["in_centrality"], 0.);
    assert_eq!(centralities["out_centrality"], 0.);
    assert_eq!(centralities["total_centrality"], 0.);
    assert_abs_diff_eq!(centralities["undirected_centrality"], 2./3., epsilon = 1e-10);
    
    let centralities = grafo.get_centrality_degrees("node2");
    assert_abs_diff_eq!(centralities["in_centrality"], 1./3., epsilon = 1e-10);
    assert_eq!(centralities["out_centrality"], 0.);
    assert_abs_diff_eq!(centralities["total_centrality"], 1./3., epsilon = 1e-10);
    assert_abs_diff_eq!(centralities["undirected_centrality"], 2./3., epsilon = 1e-10);

    let centralities = grafo.get_centrality_degrees("node3");
    assert_eq!(centralities["in_centrality"], 0.);
    assert_abs_diff_eq!(centralities["out_centrality"], 2./3., epsilon = 1e-10);
    assert_abs_diff_eq!(centralities["total_centrality"], 2./3., epsilon = 1e-10);
    assert_abs_diff_eq!(centralities["undirected_centrality"], 1./3., epsilon = 1e-10);

    let centralities = grafo.get_centrality_degrees("node4");
    assert_abs_diff_eq!(centralities["in_centrality"], 1./3., epsilon = 1e-10);
    assert_eq!(centralities["out_centrality"], 0.);
    assert_abs_diff_eq!(centralities["total_centrality"], 1./3., epsilon = 1e-10);
    assert_abs_diff_eq!(centralities["undirected_centrality"], 1./3., epsilon = 1e-10);
}

#[test]
fn test_get_average_degree() {
    let mut grafo = create_simple_graph();

    let expected_mean_degree = (2. * 5.) / 4.;
    assert_eq!(grafo.get_average_degree(Some(false)), expected_mean_degree);

    let expected_mean_degree_directed = (1. * 5.) / 4.;
    assert_eq!(grafo.get_average_degree(Some(true)), expected_mean_degree_directed);

}

#[test]
fn test_get_node_strength_directed() {
    let mut grafo = _Graph::default();
    grafo.add_node("1".to_string());
    grafo.add_node("2".to_string());
    grafo.add_node("3".to_string());

    grafo.create_connection("1".to_string(), "2".to_string(), 2.0, Some(true));
    grafo.create_connection("2".to_string(), "3".to_string(), 3.0, Some(true));

    let strengths = grafo.get_node_strength("1");

    assert_eq!(strengths["in_strength"], 0.0);
    assert_eq!(strengths["out_strength"], 2.0);
    assert_eq!(strengths["total_strength"], 2.0);
}

#[test]
fn test_get_node_strength_undirected() {
    let mut grafo = _Graph::default();
    grafo.add_node("1".to_string());
    grafo.add_node("2".to_string());
    grafo.add_node("3".to_string());

    grafo.create_connection("1".to_string(), "2".to_string(), 2.0, Some(false));
    grafo.create_connection("2".to_string(), "3".to_string(), 3.0, Some(false));
    grafo.create_connection("1".to_string(), "3".to_string(), 5.0, Some(false));

    let strengths = grafo.get_node_strength("3");

    assert_eq!(strengths["in_strength"], 8.0);
    assert_eq!(strengths["out_strength"], 8.0);
    assert_eq!(strengths["total_strength"], 16.0);
}

#[test]
fn test_get_node_strength_mixed() {
    let mut grafo = _Graph::default();
    grafo.add_node("1".to_string());
    grafo.add_node("2".to_string());
    grafo.add_node("3".to_string());

    grafo.create_connection("1".to_string(), "2".to_string(), 2.0, Some(false));
    grafo.create_connection("2".to_string(), "3".to_string(), 3.0, Some(true));
    grafo.create_connection("1".to_string(), "3".to_string(), 5.0, Some(false));

    let strengths = grafo.get_node_strength("2");

    assert_eq!(strengths["in_strength"], 2.0);
    assert_eq!(strengths["out_strength"], 5.0);
    assert_eq!(strengths["total_strength"], 7.0);
}