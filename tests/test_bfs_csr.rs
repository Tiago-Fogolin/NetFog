use netfog::graph_core::bfs_csr::BfsCsr;
use netfog::graph_core::graph_structure_interface::IGraphStructure;
use netfog::_Graph;

type BfsCsrGraph = _Graph<BfsCsr>;

#[test]
fn test_add_node_and_create_connection() {
    let mut graph = BfsCsrGraph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1.5, Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 2.5, Some(false));

    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");

    let mut neighbors_b = graph.structure.get_neighbors_ids(b);
    neighbors_b.sort();
    let mut expected_b = vec![a, c];
    expected_b.sort();

    assert_eq!(neighbors_b, expected_b);
    assert_eq!(graph.get_edge_count(), 2);
}

#[test]
fn test_get_neighbors_ids_is_sorted_and_deduped() {
    let mut structure = BfsCsr::default();
    for _ in 0..5 {
        structure.add_node();
    }

    structure.batch_create_connections(&[
        (0, 3, 1., false),
        (0, 1, 1., false),
        (0, 4, 1., false),
        (0, 2, 1., false),
    ]);

    assert_eq!(structure.get_neighbors_ids(0), vec![1, 2, 3, 4]);
}

#[test]
fn test_duplicate_edge_is_deduplicated() {
    let mut structure = BfsCsr::default();
    for _ in 0..2 {
        structure.add_node();
    }

    structure.create_connection(0, 1, 1., Some(false));
    structure.create_connection(0, 1, 2., Some(false));

    assert_eq!(structure.get_neighbors_ids(0), vec![1]);
    assert_eq!(structure.get_neighbors_ids(1), vec![0]);
}

#[test]
fn test_self_loop_does_not_duplicate() {
    let mut structure = BfsCsr::default();
    structure.add_node();

    structure.create_connection(0, 0, 1., Some(false));

    assert_eq!(structure.get_neighbors_ids(0), vec![0]);
}

#[test]
fn test_has_connection() {
    let mut structure = BfsCsr::default();
    for _ in 0..3 {
        structure.add_node();
    }

    structure.batch_create_connections(&[(0, 1, 1., false)]);

    assert!(structure.has_connection(0, 1));
    assert!(structure.has_connection(1, 0));
    assert!(!structure.has_connection(0, 2));
}

#[test]
fn test_remove_connection() {
    let mut structure = BfsCsr::default();
    for _ in 0..3 {
        structure.add_node();
    }

    structure.batch_create_connections(&[(0, 1, 1., false), (1, 2, 1., false)]);
    structure.remove_connection(0, 1);

    assert!(!structure.has_connection(0, 1));
    assert!(!structure.has_connection(1, 0));
    assert!(structure.has_connection(1, 2));
}

#[test]
fn test_get_all_edges_reports_each_edge_once() {
    let mut structure = BfsCsr::default();
    for _ in 0..3 {
        structure.add_node();
    }

    structure.batch_create_connections(&[(0, 1, 1., false), (1, 2, 2., false)]);

    let mut edges: Vec<(usize, usize, f32, bool)> = structure.get_all_edges().collect();
    edges.sort_by_key(|&(u, v, _, _)| (u, v));

    assert_eq!(edges, vec![(0, 1, 1., false), (1, 2, 2., false)]);
}

#[test]
#[should_panic(expected = "so suporta grafo nao-direcionado")]
fn test_directed_connection_panics() {
    let mut structure = BfsCsr::default();
    for _ in 0..2 {
        structure.add_node();
    }

    structure.create_connection(0, 1, 1., Some(true));
}

#[test]
fn test_bfs_works_through_graph_wrapper() {
    let mut graph = BfsCsrGraph::default();
    graph.add_node("0".to_string());
    graph.add_node("1".to_string());
    graph.add_node("2".to_string());
    graph.add_node("3".to_string());

    graph.create_connection("0".to_string(), "1".to_string(), 1., Some(false));
    graph.create_connection("1".to_string(), "2".to_string(), 1., Some(false));
    graph.create_connection("2".to_string(), "3".to_string(), 1., Some(false));

    let order = graph.bfs("0");
    assert_eq!(order, vec!["0", "1", "2", "3"]);
}

#[test]
fn test_connected_components_through_graph_wrapper() {
    let mut graph = BfsCsrGraph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "D".to_string(), 1., Some(false));

    let components = graph.connected_components();
    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");
    let d = graph.label_to_id("D").expect("D not found");

    assert_eq!(components[a], components[b]);
    assert_eq!(components[c], components[d]);
    assert_ne!(components[a], components[c]);
}

#[test]
fn test_triangle_count_through_graph_wrapper() {
    let mut graph = BfsCsrGraph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(false));
    graph.create_connection("C".to_string(), "A".to_string(), 1., Some(false));

    assert_eq!(graph.triangle_count(), 1);
}

#[test]
fn test_dijkstra_through_graph_wrapper() {
    let mut graph = BfsCsrGraph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(false));
    graph.create_connection("B".to_string(), "C".to_string(), 2., Some(false));

    let distances = graph.dijkstra("A");
    assert_eq!(distances["C"], 3.);
}
