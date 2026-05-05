use netfog::graph_core::adjacency_list::AdjacencyList;
use netfog::graph_core::adjacency_matrix::AdjacencyMatrix;
use netfog::{_Graph, *};
use netfog::TestGraph;
use netfog::file_reader_core::file_reader::{read_edge_list_file, read_mtx_file};
use std::collections::HashMap;
use crate::external_apis::core::{OpenAlexGraphType};
use netfog::layout::style::GraphStyle;
use netfog::layout::layout::Layout;

fn make_conn(from: &str, to: &str, weight: f32, directed: bool) -> HashMap<String, ConnectionProperty> {
    let mut conn = HashMap::new();
    conn.insert("from".to_string(), ConnectionProperty::From(from.to_string()));
    conn.insert("to".to_string(), ConnectionProperty::To(to.to_string()));
    conn.insert("weight".to_string(), ConnectionProperty::Weight(weight));
    conn.insert("directed".to_string(), ConnectionProperty::Directed(directed));
    conn
}

#[test]
#[ignore]
fn test_from_openalex_api() {
    let style = GraphStyle::default();
    let mut g = TestGraph::from_openalex(None,None,None,None, Some("TEST"), OpenAlexGraphType::Coauthorship, "YOUR_API_KEY", Some(10), Some(1.), None);
    g.output_html("teste_open_alex_mock.html", Layout::Spring, true, style);
    std::fs::remove_file("teste_open_alex_mock.html").ok();
}

#[test]
#[ignore]
fn test_from_overpass_api() {
    let style = GraphStyle::default();
    let mut g = TestGraph::from_overpass_address("Test".to_string(), 20.0);
    g.output_html("teste_overpass_mock.html", Layout::Random, false, style);
    std::fs::remove_file("teste_overpass_mock.html").ok();
}

#[test]
fn test_from_adjacency_matrix() {


    let adj_matrix = vec![
        vec![0., 1.],
        vec![1., 0.],
    ];

    let mut graph = TestGraph::from_adjacency_matrix(
        adj_matrix,
        Some(false),
        Some(vec!["one".to_string(), "two".to_string()]),
    );

    let connections = vec![
        make_conn("one", "two", 1., false),
    ];

    assert_eq!(connections, graph.get_connections(None, None, false));


    let adj_matrix2 = vec![
        vec![0., 2., 1.],
        vec![1., 0., 3.],
        vec![1., 2., 0.],
    ];

    let mut graph2 = TestGraph::from_adjacency_matrix(
        adj_matrix2,
        Some(true),
        Some(vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
        ]),
    );

    let connections2 = vec![
        make_conn("one", "two", 2., true),
        make_conn("one", "three", 1., true),
        make_conn("two", "one", 1., true),
        make_conn("two", "three", 3., true),
        make_conn("three", "one", 1., true),
        make_conn("three", "two", 2., true),
    ];

    assert_eq!(connections2, graph2.get_connections(None, None, false));
}

#[test]
fn test_generate_adjacency_matrix() {
    let adj_matrix = vec![
        vec![0., 2., 1.],
        vec![2., 0., 3.],
        vec![1., 3., 0.],
    ];

    let mut graph = TestGraph::from_adjacency_matrix(adj_matrix.clone(), Some(false), None);

    let generated_adj_matrix = graph.generate_adjacency_matrix();


    assert_eq!(adj_matrix, generated_adj_matrix);
}



#[test]
fn test_read_mtx_undirected_weighted() {
    let content = "\
%%MatrixMarket matrix coordinate real symmetric
% comment line
3 3 2
1 2 1.5
2 3 2.5
";
    std::fs::write("test_undirected_weighted.mtx", content).unwrap();

    let mut graph: TestGraph = read_mtx_file("test_undirected_weighted.mtx")
        .expect("Failed to read .mtx file");

    assert_eq!(graph.get_node_count(), 3);
    assert_eq!(graph.get_edge_count(), 2);

    let connections = graph.get_connections(None, None, false);
    let has_12 = connections.iter().any(|c| {
        matches!(c.get("from"), Some(ConnectionProperty::From(f)) if f == "1")
            && matches!(c.get("to"), Some(ConnectionProperty::To(t)) if t == "2")
            && matches!(c.get("weight"), Some(ConnectionProperty::Weight(w)) if (w - 1.5).abs() < 1e-5)
            && matches!(c.get("directed"), Some(ConnectionProperty::Directed(false)))
    });
    let has_23 = connections.iter().any(|c| {
        matches!(c.get("from"), Some(ConnectionProperty::From(f)) if f == "2")
            && matches!(c.get("to"), Some(ConnectionProperty::To(t)) if t == "3")
            && matches!(c.get("weight"), Some(ConnectionProperty::Weight(w)) if (w - 2.5).abs() < 1e-5)
            && matches!(c.get("directed"), Some(ConnectionProperty::Directed(false)))
    });

    assert!(has_12, "Expected undirected edge 1-2 with weight 1.5");
    assert!(has_23, "Expected undirected edge 2-3 with weight 2.5");

    std::fs::remove_file("test_undirected_weighted.mtx").ok();
}

#[test]
fn test_read_mtx_directed_weighted() {
    let content = "\
%%MatrixMarket matrix coordinate real general
3 3 3
1 2 1.0
2 3 2.0
3 1 3.0
";
    std::fs::write("test_directed_weighted.mtx", content).unwrap();

    let mut graph: TestGraph = read_mtx_file("test_directed_weighted.mtx")
        .expect("Failed to read .mtx file");

    assert_eq!(graph.get_node_count(), 3);
    assert_eq!(graph.get_edge_count(), 3);

    let connections = graph.get_connections(None, None, false);
    let all_directed = connections.iter().all(|c| {
        matches!(c.get("directed"), Some(ConnectionProperty::Directed(true)))
    });
    assert!(all_directed, "All edges should be directed for general symmetry");

    std::fs::remove_file("test_directed_weighted.mtx").ok();
}

#[test]
fn test_read_mtx_pattern_unweighted() {
    let content = "\
%%MatrixMarket matrix coordinate pattern symmetric
4 4 3
1 2
1 3
3 4
";
    std::fs::write("test_pattern.mtx", content).unwrap();

    let mut graph: TestGraph = read_mtx_file("test_pattern.mtx")
        .expect("Failed to read .mtx file");

    assert_eq!(graph.get_node_count(), 4);
    assert_eq!(graph.get_edge_count(), 3);

    let connections = graph.get_connections(None, None, false);
    let all_weight_one = connections.iter().all(|c| {
        matches!(c.get("weight"), Some(ConnectionProperty::Weight(w)) if (w - 1.0).abs() < 1e-5)
    });
    assert!(all_weight_one, "Pattern edges should all have weight 1.0");

    std::fs::remove_file("test_pattern.mtx").ok();
}

#[test]
fn test_read_mtx_integer_type() {
    let content = "\
%%MatrixMarket matrix coordinate integer symmetric
2 2 1
1 2 7
";
    std::fs::write("test_integer.mtx", content).unwrap();

    let mut graph: TestGraph = read_mtx_file("test_integer.mtx")
        .expect("Failed to read .mtx file");

    assert_eq!(graph.get_node_count(), 2);
    assert_eq!(graph.get_edge_count(), 1);

    let connections = graph.get_connections(None, None, false);
    let has_edge = connections.iter().any(|c| {
        matches!(c.get("weight"), Some(ConnectionProperty::Weight(w)) if (w - 7.0).abs() < 1e-5)
    });
    assert!(has_edge, "Edge weight should be 7.0 for integer type");

    std::fs::remove_file("test_integer.mtx").ok();
}

#[test]
fn test_read_mtx_invalid_header() {
    let content = "NOT A MATRIX MARKET FILE\n1 2 1\n";
    std::fs::write("test_invalid_header.mtx", content).unwrap();

    let result: Result<TestGraph, _> = read_mtx_file("test_invalid_header.mtx");
    assert!(result.is_err(), "Should return an error for invalid header");

    std::fs::remove_file("test_invalid_header.mtx").ok();
}

#[test]
fn test_from_mtx_file_method() {
    let content = "\
%%MatrixMarket matrix coordinate real symmetric
2 2 1
1 2 4.0
";
    std::fs::write("test_from_mtx.mtx", content).unwrap();

    let graph = TestGraph::from_mtx_file("test_from_mtx.mtx");

    assert_eq!(graph.get_node_count(), 2);
    assert!(graph.build_time_ms.is_some(), "build_time_ms should be set");

    std::fs::remove_file("test_from_mtx.mtx").ok();
}

#[test]
fn test_read_edge_list_undirected_weighted() {
    let content = "\
# undirected weighted edge list
A B 1.5
B C 2.5
";
    std::fs::write("test_el_undirected_weighted.txt", content).unwrap();

    let mut graph: TestGraph = read_edge_list_file("test_el_undirected_weighted.txt", false)
        .expect("Failed to read edge list file");

    assert_eq!(graph.get_node_count(), 3);
    assert_eq!(graph.get_edge_count(), 2);

    let connections = graph.get_connections(None, None, false);
    let has_ab = connections.iter().any(|c| {
        matches!(c.get("from"), Some(ConnectionProperty::From(f)) if f == "A")
            && matches!(c.get("to"), Some(ConnectionProperty::To(t)) if t == "B")
            && matches!(c.get("weight"), Some(ConnectionProperty::Weight(w)) if (w - 1.5).abs() < 1e-5)
            && matches!(c.get("directed"), Some(ConnectionProperty::Directed(false)))
    });
    let has_bc = connections.iter().any(|c| {
        matches!(c.get("from"), Some(ConnectionProperty::From(f)) if f == "B")
            && matches!(c.get("to"), Some(ConnectionProperty::To(t)) if t == "C")
            && matches!(c.get("weight"), Some(ConnectionProperty::Weight(w)) if (w - 2.5).abs() < 1e-5)
            && matches!(c.get("directed"), Some(ConnectionProperty::Directed(false)))
    });

    assert!(has_ab, "Expected undirected edge A-B with weight 1.5");
    assert!(has_bc, "Expected undirected edge B-C with weight 2.5");

    std::fs::remove_file("test_el_undirected_weighted.txt").ok();
}

#[test]
fn test_read_edge_list_directed_unweighted() {
    let content = "\
1 2
2 3
3 1
";
    std::fs::write("test_el_directed_unweighted.txt", content).unwrap();

    let mut graph: TestGraph = read_edge_list_file("test_el_directed_unweighted.txt", true)
        .expect("Failed to read edge list file");

    assert_eq!(graph.get_node_count(), 3);
    assert_eq!(graph.get_edge_count(), 3);

    let connections = graph.get_connections(None, None, false);
    let all_directed = connections.iter().all(|c| {
        matches!(c.get("directed"), Some(ConnectionProperty::Directed(true)))
    });
    assert!(all_directed, "All edges should be directed");

    let all_weight_one = connections.iter().all(|c| {
        matches!(c.get("weight"), Some(ConnectionProperty::Weight(w)) if (w - 1.0).abs() < 1e-5)
    });
    assert!(all_weight_one, "All unweighted edges should have weight 1.0");

    std::fs::remove_file("test_el_directed_unweighted.txt").ok();
}

#[test]
fn test_read_edge_list_comments_skipped() {
    let content = "\
# this is a comment
# another comment
X Y 3.0
# inline ignored
Y Z 4.0
";
    std::fs::write("test_el_comments.txt", content).unwrap();

    let mut graph: TestGraph = read_edge_list_file("test_el_comments.txt", false)
        .expect("Failed to read edge list file");

    assert_eq!(graph.get_node_count(), 3);
    assert_eq!(graph.get_edge_count(), 2);

    std::fs::remove_file("test_el_comments.txt").ok();
}

#[test]
fn test_read_edge_list_invalid_weight() {
    let content = "A B notanumber\n";
    std::fs::write("test_el_bad_weight.txt", content).unwrap();

    let result: Result<TestGraph, _> = read_edge_list_file("test_el_bad_weight.txt", false);
    assert!(result.is_err(), "Should return error for invalid weight");

    std::fs::remove_file("test_el_bad_weight.txt").ok();
}

#[test]
fn test_from_edge_list_file_method() {
    let content = "\
A B 1.0
B C 2.0
";
    std::fs::write("test_from_edge_list.txt", content).unwrap();

    let mut graph = TestGraph::from_edge_list_file("test_from_edge_list.txt", false);

    assert_eq!(graph.get_node_count(), 3);
    assert_eq!(graph.get_edge_count(), 2);
    assert!(graph.build_time_ms.is_some(), "build_time_ms should be set");

    std::fs::remove_file("test_from_edge_list.txt").ok();
}

