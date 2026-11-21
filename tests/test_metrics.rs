use std::hash::Hash;
use std::{collections::HashMap};
use netfog::*;
use netfog::TestGraph as _Graph;
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
    graph.create_connection("node2".to_string(), "node3".to_string(), 1.6, Some(false));

    return graph;
}

#[test]
fn test_total_weight() {
    let epsilon = 1e-5;

    let mut graph = _Graph::default();
    graph.add_node("node1".to_string());
    graph.add_node("node2".to_string());
    graph.add_node("node3".to_string());
    graph.add_node("node4".to_string());

    graph.create_connection("node1".to_string(), "node2".to_string(), 2., None);
    graph.create_connection("node3".to_string(), "node4".to_string(), 4., None);
    graph.create_connection("node4".to_string(), "node1".to_string(), 5.5, None);
    graph.create_connection("node2".to_string(), "node3".to_string(), 1.6, None);


    let total_weight = 2.0 + 4.0 + 5.5 + 1.6;
    assert!(
        (total_weight - graph.get_total_weight()).abs() < epsilon
    );
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
    grafo.create_connection("node2".to_string(), "node3".to_string(), 1.6, None);

    let mean = (2. + 4. + 5.5 + 1.6)/4.;

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
    let expected_density = 4. / 6.;
    assert_eq!(expected_density, grafo.get_density(Some(false)));

    let expected_density_directed = 4. / 12.;
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
    assert_eq!(degrees_node2["in_degree"], 0);
    assert_eq!(degrees_node2["out_degree"], 0);
    assert_eq!(degrees_node2["undirected_degree"], 2);
    assert_eq!(degrees_node2["total_degree"], 0);

    let degrees_node3 = grafo.compute_degrees("node3");
    assert_eq!(degrees_node3["in_degree"], 0);
    assert_eq!(degrees_node3["out_degree"], 1);
    assert_eq!(degrees_node3["undirected_degree"], 1);
    assert_eq!(degrees_node3["total_degree"], 1);

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
        m.insert("in_degree".to_string(), 0);
        m.insert("out_degree".to_string(), 0);
        m.insert("undirected_degree".to_string(), 2);
        m.insert("total_degree".to_string(), 0);
        m
    });

    expected_dict.insert("node3".to_string(), {
        let mut m = std::collections::HashMap::new();
        m.insert("in_degree".to_string(), 0);
        m.insert("out_degree".to_string(), 1);
        m.insert("undirected_degree".to_string(), 1);
        m.insert("total_degree".to_string(), 1);
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
    assert_abs_diff_eq!(centralities["undirected_centrality"], 2./3., epsilon = 1e-5);

    let centralities = grafo.get_centrality_degrees("node2");
    assert_eq!(centralities["in_centrality"], 0.);
    assert_eq!(centralities["out_centrality"], 0.);
    assert_eq!(centralities["total_centrality"], 0.);
    assert_abs_diff_eq!(centralities["undirected_centrality"], 2./3., epsilon = 1e-5);

    let centralities = grafo.get_centrality_degrees("node3");
    assert_eq!(centralities["in_centrality"], 0.);
    assert_abs_diff_eq!(centralities["out_centrality"], 1./3., epsilon = 1e-5);
    assert_abs_diff_eq!(centralities["total_centrality"], 1./3., epsilon = 1e-5);
    assert_abs_diff_eq!(centralities["undirected_centrality"], 1./3., epsilon = 1e-5);

    let centralities = grafo.get_centrality_degrees("node4");
    assert_abs_diff_eq!(centralities["in_centrality"], 1./3., epsilon = 1e-5);
    assert_eq!(centralities["out_centrality"], 0.);
    assert_abs_diff_eq!(centralities["total_centrality"], 1./3., epsilon = 1e-5);
    assert_abs_diff_eq!(centralities["undirected_centrality"], 1./3., epsilon = 1e-5);
}

#[test]
fn test_get_average_degree() {
    let mut grafo = create_simple_graph();

    let expected_mean_degree = (2. * 4.) / 4.;
    assert_eq!(grafo.get_average_degree(Some(false)), expected_mean_degree);

    let expected_mean_degree_directed = (1. * 4.) / 4.;
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

#[test]
fn test_degree_distribution() {
    let mut grafo = create_simple_graph();
    let distribution = grafo.get_degree_distribution();

    let expected_distribution: HashMap<&str, HashMap<i32, f32>> = HashMap::from([
        ("in_distribution", HashMap::from([
            (0, 0.75),
            (1, 0.25),
        ])),
        ("out_distribution", HashMap::from([
            (1, 0.25),
            (0, 0.75),
        ])),
        ("undirected_distribution", HashMap::from([
            (2, 0.5),
            (1, 0.5),
        ])),
    ]);

    assert_eq!(distribution["in_distribution"], expected_distribution["in_distribution"]);
    assert_eq!(distribution["out_distribution"], expected_distribution["out_distribution"]);
    assert_eq!(distribution["undirected_distribution"], expected_distribution["undirected_distribution"]);
}

#[test]
fn test_compute_entropy() {
    let mut grafo = create_simple_graph();

    let entropy = grafo.compute_entropy();

    assert_abs_diff_eq!(entropy["in_entropy"], 0.5623, epsilon = 1e-4);
    assert_abs_diff_eq!(entropy["out_entropy"], 0.5623, epsilon = 1e-4);
    assert_abs_diff_eq!(entropy["undirected_entropy"], 0.6931, epsilon = 1e-4);

}

#[test]
fn test_get_max_possible_entropy() {
    let mut grafo = create_simple_graph();
    let max_entropy = grafo.get_max_possible_entropy();

    assert_abs_diff_eq!(max_entropy, 1.0986, epsilon = 1e-4);

}

#[test]
fn test_get_skewness() {
    let mut grafo = create_simple_graph();
    let skewness = grafo.get_skewness();

    assert_abs_diff_eq!(skewness["in_skewness"], 0.4, epsilon = 1e-4);
    assert_abs_diff_eq!(skewness["out_skewness"], 0.4, epsilon = 1e-4);
    assert_abs_diff_eq!(skewness["undirected_skewness"], 0.8666, epsilon = 1e-4);

}

fn create_path_of_three() -> _Graph {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));

    return graph;
}

fn create_triangle() -> _Graph {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "A".to_string(), 1., Some(false));

    return graph;
}

fn create_two_triangles_sharing_edge() -> _Graph {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "A".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "D".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "D".to_string(), 1., Some(false));

    return graph;
}

fn create_star_of_four() -> _Graph {
    let mut graph = _Graph::default();
    graph.add_node("Center".to_string());
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("Center".to_string(), "A".to_string(), 1., Some(false));
    graph.create_connection("Center".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("Center".to_string(), "C".to_string(), 1., Some(false));

    return graph;
}

#[test]
fn test_closeness_centrality_path_of_three() {
    let mut graph = create_path_of_three();

    assert_abs_diff_eq!(graph.closeness_centrality("B"), 0.5, epsilon = 1e-5);
    assert_abs_diff_eq!(graph.closeness_centrality("A"), 1. / 3., epsilon = 1e-5);
    assert_abs_diff_eq!(graph.closeness_centrality("C"), 1. / 3., epsilon = 1e-5);
}

#[test]
fn test_closeness_centrality_triangle_is_equal_everywhere() {
    let mut graph = create_triangle();

    assert_abs_diff_eq!(graph.closeness_centrality("A"), 0.5, epsilon = 1e-5);
    assert_abs_diff_eq!(graph.closeness_centrality("B"), 0.5, epsilon = 1e-5);
    assert_abs_diff_eq!(graph.closeness_centrality("C"), 0.5, epsilon = 1e-5);
}

#[test]
fn test_closeness_centrality_unreachable_node_is_zero() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));

    assert_eq!(graph.closeness_centrality("A"), 0.);
}

#[test]
fn test_closeness_centrality_all_matches_per_node_path_of_three() {
    let mut graph = create_path_of_three();

    let scores = graph.closeness_centrality_all();

    assert_abs_diff_eq!(scores[0], 1. / 3., epsilon = 1e-5);
    assert_abs_diff_eq!(scores[1], 0.5, epsilon = 1e-5);
    assert_abs_diff_eq!(scores[2], 1. / 3., epsilon = 1e-5);
}

#[test]
fn test_closeness_centrality_all_matches_per_node_triangle() {
    let mut graph = create_triangle();

    let scores = graph.closeness_centrality_all();

    assert_abs_diff_eq!(scores[0], 0.5, epsilon = 1e-5);
    assert_abs_diff_eq!(scores[1], 0.5, epsilon = 1e-5);
    assert_abs_diff_eq!(scores[2], 0.5, epsilon = 1e-5);
}

#[test]
fn test_closeness_centrality_all_unreachable_node_is_zero() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));

    let scores = graph.closeness_centrality_all();

    assert_eq!(scores[0], 0.);
    assert_eq!(scores[1], 0.);
    assert_eq!(scores[2], 0.);
}

#[test]
fn test_closeness_centrality_all_empty_graph() {
    let mut graph = _Graph::default();

    assert_eq!(graph.closeness_centrality_all().len(), 0);
}

#[test]
fn test_clustering_coefficient_triangle_is_one() {
    let mut graph = create_triangle();

    assert_abs_diff_eq!(graph.clustering_coefficient("A"), 1., epsilon = 1e-5);
    assert_abs_diff_eq!(graph.clustering_coefficient("B"), 1., epsilon = 1e-5);
    assert_abs_diff_eq!(graph.clustering_coefficient("C"), 1., epsilon = 1e-5);
}

#[test]
fn test_clustering_coefficient_path_is_zero() {
    let mut graph = create_path_of_three();

    assert_abs_diff_eq!(graph.clustering_coefficient("A"), 0., epsilon = 1e-5);
    assert_abs_diff_eq!(graph.clustering_coefficient("B"), 0., epsilon = 1e-5);
    assert_abs_diff_eq!(graph.clustering_coefficient("C"), 0., epsilon = 1e-5);
}

#[test]
fn test_clustering_coefficient_two_triangles_sharing_an_edge() {
    let mut graph = create_two_triangles_sharing_edge();

    assert_abs_diff_eq!(graph.clustering_coefficient("A"), 1., epsilon = 1e-5);
    assert_abs_diff_eq!(graph.clustering_coefficient("D"), 1., epsilon = 1e-5);
    assert_abs_diff_eq!(graph.clustering_coefficient("B"), 2. / 3., epsilon = 1e-5);
    assert_abs_diff_eq!(graph.clustering_coefficient("C"), 2. / 3., epsilon = 1e-5);
}

#[test]
fn test_transitivity_triangle_is_one() {
    let mut graph = create_triangle();

    assert_abs_diff_eq!(graph.transitivity(), 1., epsilon = 1e-5);
}

#[test]
fn test_transitivity_path_is_zero() {
    let mut graph = create_path_of_three();

    assert_abs_diff_eq!(graph.transitivity(), 0., epsilon = 1e-5);
}

#[test]
fn test_transitivity_two_triangles_sharing_an_edge() {
    let mut graph = create_two_triangles_sharing_edge();

    assert_abs_diff_eq!(graph.transitivity(), 0.75, epsilon = 1e-5);
}

#[test]
fn test_transitivity_empty_graph_is_zero() {
    let mut graph = _Graph::default();

    assert_eq!(graph.transitivity(), 0.);
}

#[test]
fn test_k_core_decomposition_triangle() {
    let mut graph = create_triangle();
    let core = graph.k_core_decomposition();

    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");

    assert_eq!(core[&a], 2);
    assert_eq!(core[&b], 2);
    assert_eq!(core[&c], 2);
}

#[test]
fn test_k_core_decomposition_path_of_three() {
    let mut graph = create_path_of_three();
    let core = graph.k_core_decomposition();

    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");

    assert_eq!(core[&a], 1);
    assert_eq!(core[&b], 1);
    assert_eq!(core[&c], 1);
}

#[test]
fn test_k_core_decomposition_star_of_four() {
    let mut graph = create_star_of_four();
    let core = graph.k_core_decomposition();

    for value in core.values() {
        assert_eq!(*value, 1);
    }
}

#[test]
fn test_k_core_decomposition_two_triangles_sharing_an_edge() {
    let mut graph = create_two_triangles_sharing_edge();
    let core = graph.k_core_decomposition();

    for value in core.values() {
        assert_eq!(*value, 2);
    }
}

#[test]
fn test_k_core_decomposition_isolated_nodes() {
    let mut graph = _Graph::default();
    graph.batch_add_anonymous_nodes(3);

    let core = graph.k_core_decomposition();

    for value in core.values() {
        assert_eq!(*value, 0);
    }
}

#[test]
fn test_k_core_decomposition_empty_graph() {
    let mut graph = _Graph::default();
    let core = graph.k_core_decomposition();

    assert_eq!(core.len(), 0);
}
