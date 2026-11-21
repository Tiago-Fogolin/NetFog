use netfog::TestGraph as _Graph;

#[test]
fn test_connected_components_single_component() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "D".to_string(), 1., Some(false));

    let components = graph.connected_components();
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");
    let d = graph.label_to_id("D").expect("D not found");

    assert_eq!(components[a], components[b]);
    assert_eq!(components[a], components[c]);
    assert_eq!(components[a], components[d]);
}

#[test]
fn test_connected_components_multiple_components() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());
    graph.add_node("E".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "D".to_string(), 1., Some(false));

    let components = graph.connected_components();
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");
    let d = graph.label_to_id("D").expect("D not found");
    let e = graph.label_to_id("E").expect("E not found");

    assert_eq!(components[a], components[b]);
    assert_eq!(components[c], components[d]);
    assert_ne!(components[a], components[c]);
    assert_ne!(components[a], components[e]);
    assert_ne!(components[c], components[e]);

    let mut unique_components: Vec<usize> = components.clone();
    unique_components.sort();
    unique_components.dedup();
    assert_eq!(unique_components.len(), 3);
}

#[test]
fn test_connected_components_ignores_edge_direction() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(true));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(true));

    let components = graph.connected_components();
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");

    assert_eq!(components[a], components[b]);
    assert_eq!(components[b], components[c]);
}

#[test]
fn test_connected_components_isolated_nodes() {
    let mut graph = _Graph::default();
    graph.batch_add_anonymous_nodes(3);

    let components = graph.connected_components();

    let mut unique_components: Vec<usize> = components.clone();
    unique_components.sort();
    unique_components.dedup();
    assert_eq!(unique_components.len(), 3);
}

#[test]
fn test_connected_components_empty_graph() {
    let mut graph = _Graph::default();
    let components = graph.connected_components();
    assert_eq!(components.len(), 0);
}

#[test]
fn test_connected_components_grouped_matches_flat_result() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "D".to_string(), 1., Some(false));

    let components = graph.connected_components();
    let grouped = graph.connected_components_grouped();

    assert_eq!(grouped.len(), 2);

    for (component_id, members) in &grouped {
        for &member in members {
            assert_eq!(components[member], *component_id);
        }
    }

    let total_members: usize = grouped.values().map(|members| members.len()).sum();
    assert_eq!(total_members, graph.get_node_count());
}

fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    return (a - b).abs() < eps;
}

#[test]
fn test_pagerank_sums_to_one() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());
    graph.add_node("E".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(true));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(true));
    graph.create_connection("C".to_string(), "D".to_string(), 1., Some(true));
    graph.create_connection("D".to_string(), "E".to_string(), 1., Some(true));
    graph.create_connection("E".to_string(), "A".to_string(), 1., Some(true));

    let ranks = graph.pagerank(0.85, 100, 1e-6);
    let total: f32 = ranks.iter().sum();

    assert!(approx_eq(total, 1.0, 1e-3), "expected sum ~1.0, got {}", total);
}

#[test]
fn test_pagerank_symmetric_cycle_gives_equal_scores() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(true));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(true));
    graph.create_connection("C".to_string(), "D".to_string(), 1., Some(true));
    graph.create_connection("D".to_string(), "A".to_string(), 1., Some(true));

    let ranks = graph.pagerank(0.85, 100, 1e-8);
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");
    let d = graph.label_to_id("D").expect("D not found");

    assert!(approx_eq(ranks[a], 0.25, 1e-3));
    assert!(approx_eq(ranks[b], 0.25, 1e-3));
    assert!(approx_eq(ranks[c], 0.25, 1e-3));
    assert!(approx_eq(ranks[d], 0.25, 1e-3));
}

#[test]
fn test_pagerank_more_inbound_links_ranks_higher() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("Hub".to_string());

    graph.create_connection("A".to_string(), "Hub".to_string(), 1., Some(true));
    graph.create_connection("B".to_string(), "Hub".to_string(), 1., Some(true));
    graph.create_connection("C".to_string(), "Hub".to_string(), 1., Some(true));
    graph.create_connection("Hub".to_string(), "A".to_string(), 1., Some(true));

    let ranks = graph.pagerank(0.85, 100, 1e-8);
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");
    let hub = graph.label_to_id("Hub").expect("Hub not found");

    assert!(ranks[hub] > ranks[b]);
    assert!(ranks[hub] > ranks[c]);
}

#[test]
fn test_pagerank_empty_graph_returns_empty_map() {
    let mut graph = _Graph::default();
    let ranks = graph.pagerank(0.85, 100, 1e-6);
    assert_eq!(ranks.len(), 0);
}

#[test]
fn test_pagerank_dangling_node_mass_still_sums_to_one() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(true));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(true));
    graph.create_connection("C".to_string(), "D".to_string(), 1., Some(true));

    let ranks = graph.pagerank(0.85, 100, 1e-6);
    let total: f32 = ranks.iter().sum();

    assert!(approx_eq(total, 1.0, 1e-3), "expected sum ~1.0, got {}", total);
}

#[test]
fn test_triangle_count_simple_triangle() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "A".to_string(), 1., Some(false));

    assert_eq!(graph.triangle_count(), 1);

    let per_node = graph.triangle_count_by_node();
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");

    assert_eq!(per_node[a], 1);
    assert_eq!(per_node[b], 1);
    assert_eq!(per_node[c], 1);
}

#[test]
fn test_triangle_count_open_path_has_no_triangle() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));

    assert_eq!(graph.triangle_count(), 0);
}

#[test]
fn test_triangle_count_two_triangles_sharing_an_edge() {
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

    assert_eq!(graph.triangle_count(), 2);

    let per_node = graph.triangle_count_by_node();
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");
    let d = graph.label_to_id("D").expect("D not found");

    assert_eq!(per_node[a], 1);
    assert_eq!(per_node[d], 1);
    assert_eq!(per_node[b], 2);
    assert_eq!(per_node[c], 2);
}

#[test]
fn test_triangle_count_ignores_edge_direction() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(true));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(true));
    graph.create_connection("C".to_string(), "A".to_string(), 1., Some(true));

    assert_eq!(graph.triangle_count(), 1);
}

#[test]
fn test_triangle_count_empty_graph() {
    let mut graph = _Graph::default();
    assert_eq!(graph.triangle_count(), 0);
    assert_eq!(graph.triangle_count_by_node().len(), 0);
}

#[test]
fn test_betweenness_centrality_path_of_three() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));

    let scores = graph.betweenness_centrality();
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");

    assert!(approx_eq(scores[a], 0.0, 1e-6));
    assert!(approx_eq(scores[b], 1.0, 1e-6));
    assert!(approx_eq(scores[c], 0.0, 1e-6));
}

#[test]
fn test_betweenness_centrality_star_graph_center_is_maximal() {
    let mut graph = _Graph::default();
    graph.add_node("Center".to_string());
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("Center".to_string(), "A".to_string(), 1., Some(false));
    graph.create_connection("Center".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("Center".to_string(), "C".to_string(), 1., Some(false));

    let scores = graph.betweenness_centrality();
    let center = graph.label_to_id("Center").expect("Center not found");
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");

    assert!(approx_eq(scores[center], 1.0, 1e-6));
    assert!(approx_eq(scores[a], 0.0, 1e-6));
    assert!(approx_eq(scores[b], 0.0, 1e-6));
    assert!(approx_eq(scores[c], 0.0, 1e-6));
}

#[test]
fn test_betweenness_centrality_triangle_is_zero_everywhere() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "A".to_string(), 1., Some(false));

    let scores = graph.betweenness_centrality();

    for score in scores.iter() {
        assert!(approx_eq(*score, 0.0, 1e-6));
    }
}

#[test]
fn test_betweenness_centrality_empty_graph() {
    let mut graph = _Graph::default();
    assert_eq!(graph.betweenness_centrality().len(), 0);
}

#[test]
fn test_betweenness_centrality_two_nodes_does_not_panic() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));

    let scores = graph.betweenness_centrality();

    for score in scores.iter() {
        assert!(approx_eq(*score, 0.0, 1e-6));
    }
}
