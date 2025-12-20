use netfog::*;

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
