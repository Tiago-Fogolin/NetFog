use netfog::*;
use netfog::TestGraph as _Graph;
use netfog::graph_core::graph_structure_interface::IGraphStructure;

#[test]
fn test_dfs() {
    let mut graph = _Graph::default();
    graph.add_node("1".to_string());
    graph.add_node("2".to_string());
    graph.add_node("3".to_string());
    graph.add_node("4".to_string());
    graph.add_node("5".to_string());

    graph.create_connection("1".to_string(), "2".to_string(), 1., Some(false));
    graph.create_connection("1".to_string(), "3".to_string(), 1., Some(false));
    graph.create_connection("2".to_string(), "4".to_string(), 1., Some(false));
    graph.create_connection("2".to_string(), "5".to_string(), 1., Some(false));

    let expected_order: Vec<String> = vec![
        "1".to_string(),
        "2".to_string(),
        "4".to_string(),
        "5".to_string(),
        "3".to_string()
    ];

    assert_eq!(expected_order, graph.dfs("1"));
}

#[test]
fn test_bfs() {
    let mut graph = _Graph::default();
    graph.add_node("0".to_string());
    graph.add_node("1".to_string());
    graph.add_node("2".to_string());
    graph.add_node("3".to_string());
    graph.add_node("4".to_string());
    graph.add_node("5".to_string());
    graph.add_node("6".to_string());
    graph.add_node("7".to_string());

    graph.create_connection("0".to_string(), "1".to_string(), 1., Some(false));
    graph.create_connection("0".to_string(), "2".to_string(), 1., Some(false));
    graph.create_connection("0".to_string(), "3".to_string(), 1., Some(false));
    graph.create_connection("1".to_string(), "4".to_string(), 1., Some(false));
    graph.create_connection("1".to_string(), "5".to_string(), 1., Some(false));
    graph.create_connection("2".to_string(), "6".to_string(), 1., Some(false));
    graph.create_connection("3".to_string(), "7".to_string(), 1., Some(false));


    let expected_order: Vec<String> = vec![
        "0".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string()
    ];

    assert_eq!(expected_order, graph.bfs("0"));
}

#[test]
fn test_dijkstra() {
    let mut graph = _Graph::default();
    graph.add_node("1".to_string());
    graph.add_node("2".to_string());
    graph.add_node("3".to_string());
    graph.add_node("4".to_string());
    graph.add_node("5".to_string());
    graph.add_node("6".to_string());

    graph.create_connection("1".to_string(), "2".to_string(), 9., Some(false));
    graph.create_connection("1".to_string(), "3".to_string(), 4., Some(false));
    graph.create_connection("2".to_string(), "3".to_string(), 2., Some(false));
    graph.create_connection("2".to_string(), "5".to_string(), 3., Some(false));
    graph.create_connection("2".to_string(), "4".to_string(), 7., Some(false));
    graph.create_connection("3".to_string(), "4".to_string(), 1., Some(false));
    graph.create_connection("3".to_string(), "5".to_string(), 6., Some(false));
    graph.create_connection("4".to_string(), "5".to_string(), 4., Some(false));
    graph.create_connection("4".to_string(), "6".to_string(), 8., Some(false));
    graph.create_connection("5".to_string(), "6".to_string(), 2., Some(false));

    let dijkstra_dists = graph.dijkstra("1");

    assert_eq!(dijkstra_dists["6"], 11.);
}

#[test]
fn test_get_neighbors_ids_directed() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());

    graph.create_connection("A".to_string(), "B".to_string(), 1., Some(true));
    graph.create_connection("B".to_string(), "C".to_string(), 1., Some(true));

    let a_id = graph.label_to_id("A").expect("A not found");
    let b_id = graph.label_to_id("B").expect("B not found");
    let c_id = graph.label_to_id("C").expect("C not found");

    let mut neighbors_a = graph.structure.get_neighbors_ids(a_id);
    let mut neighbors_b = graph.structure.get_neighbors_ids(b_id);
    let mut neighbors_c = graph.structure.get_neighbors_ids(c_id);

    neighbors_a.sort();
    neighbors_b.sort();
    neighbors_c.sort();

    let mut expected_b = vec![a_id, c_id];
    expected_b.sort();

    assert_eq!(neighbors_a, vec![b_id]);
    assert_eq!(neighbors_b, expected_b);
    assert_eq!(neighbors_c, vec![b_id]);
}

/// Ordem de entrada fora de ordem de `from`, misturando direcionada,
/// não-direcionada e self-loop -- exercita o counting sort do
/// batch_create_connections (item 0.5 do TODO). Cada par (from,to) aparece
/// uma única vez: matriz e PCSR não suportam arestas paralelas.
#[test]
fn test_batch_create_connections_from_ids_mixed_order() {
    let mut graph = _Graph::default();
    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_node("D".to_string());

    let a = graph.label_to_id("A").expect("A not found");
    let b = graph.label_to_id("B").expect("B not found");
    let c = graph.label_to_id("C").expect("C not found");
    let d = graph.label_to_id("D").expect("D not found");

    let edges = vec![
        (d, a, 1.0, false),
        (a, b, 2.0, true),
        (c, c, 3.0, false),
        (b, c, 4.0, false),
    ];

    graph.batch_create_connections_from_ids(&edges);

    assert_eq!(graph.get_edge_count(), 4);

    let mut neighbors_a = graph.structure.get_neighbors_ids(a);
    neighbors_a.sort();
    let mut expected_a = vec![b, d];
    expected_a.sort();
    assert_eq!(neighbors_a, expected_a);

    let mut neighbors_b = graph.structure.get_neighbors_ids(b);
    neighbors_b.sort();
    let mut expected_b = vec![a, c];
    expected_b.sort();
    assert_eq!(neighbors_b, expected_b);
}

#[test]
fn test_incremental_add_node_crossing_capacity_growth() {
    let mut graph = _Graph::default();
    let n: usize = 20;

    for i in 0..n {
        graph.add_node(i.to_string());
    }

    for i in 0..n {
        let next = (i + 1) % n;
        graph.create_connection(i.to_string(), next.to_string(), 1.0, Some(false));
    }

    assert_eq!(graph.get_node_count(), n);
    assert_eq!(graph.get_edge_count(), n);

    for i in 0..n {
        let prev = (i + n - 1) % n;
        let next = (i + 1) % n;
        let mut expected = vec![prev, next];
        expected.sort();
        expected.dedup();

        let mut actual = graph.structure.get_neighbors_ids(i);
        actual.sort();

        assert_eq!(actual, expected);
    }
}
