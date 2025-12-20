use netfog::*;
use std::collections::HashMap;

fn make_conn(from: &str, to: &str, weight: f32, directed: bool) -> HashMap<String, ConnectionProperty> {
    let mut conn = HashMap::new();
    conn.insert("from".to_string(), ConnectionProperty::From(from.to_string()));
    conn.insert("to".to_string(), ConnectionProperty::To(to.to_string()));
    conn.insert("weight".to_string(), ConnectionProperty::Weight(weight));
    conn.insert("directed".to_string(), ConnectionProperty::Directed(directed));
    conn
}

#[test]
fn test_from_adjacency_matrix() {


    let adj_matrix = vec![
        vec![0., 1.],
        vec![1., 0.],
    ];

    let mut graph = _Graph::from_adjacency_matrix(
        adj_matrix,
        Some(false),
        Some(vec!["one".to_string(), "two".to_string()]),
    );

    let connections = vec![
        make_conn("one", "two", 1., false),
        make_conn("two", "one", 1., false),
    ];

    assert_eq!(connections, graph.get_connections());

    // Segundo grafo
    let adj_matrix2 = vec![
        vec![0., 2., 1.],
        vec![1., 0., 3.],
        vec![1., 2., 0.],
    ];

    let mut graph2 = _Graph::from_adjacency_matrix(
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

    assert_eq!(connections2, graph2.get_connections());
}

#[test]
fn test_generate_adjacency_matrix() {
    let adj_matrix = vec![
        vec![0., 2., 1.],
        vec![2., 0., 3.],
        vec![1., 3., 0.],
    ];

    let mut graph = _Graph::from_adjacency_matrix(adj_matrix.clone(), Some(false), None);

    let generated_adj_matrix = graph.generate_adjacency_matrix();


    assert_eq!(adj_matrix, generated_adj_matrix);
}
