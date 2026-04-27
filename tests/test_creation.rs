use netfog::graph_core::adjacency_list::AdjacencyList;
use netfog::graph_core::adjacency_matrix::AdjacencyMatrix;
use netfog::{_Graph, *};
use netfog::TestGraph;
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
    let mut g = TestGraph::from_overpass_address("Rua Paulo da Cunha Mattos".to_string(), 20.0);
    g.output_html("teste_overpass_mock.html", Layout::Random, false, style);
    // std::fs::remove_file("teste_overpass_mock.html").ok();
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
