use netfog::file_reader_core::file_reader::read_json_file;
use netfog::{file_reader_core::file_reader::read_net_file, *};
use netfog::TestGraph as _Graph;
use netfog::layout::layout::Layout;
use netfog::layout::style::GraphStyle;


#[test]
fn test_read_net_file() {
    let net_content = "*Vertices\n1 \"node1\"\n2 \"node2\"\n*Edges\n1 2 1.0\n";
    std::fs::write("mock_read.net", net_content).unwrap();

    let mut graph: _Graph = read_net_file("mock_read.net").expect("Falha ao ler o arquivo .net");
    assert_eq!(graph.get_node_count(), 2);
    
    std::fs::remove_file("mock_read.net").ok();
}

#[test]
fn test_read_json_file() {
    let json_content = r#"{
        "nodes": [{"label": "node1", "x": 10.0, "y": 20.0}, {"label": "node2"}],
        "edges": [{"source": "node1", "target": "node2", "weight": 5.0}],
        "arcs": []
    }"#;
    std::fs::write("mock_read.json", json_content).unwrap();

    let mut graph: _Graph = read_json_file("mock_read.json").expect("Falha ao ler o json");
    assert_eq!(graph.get_node_count(), 2);
    
    std::fs::remove_file("mock_read.json").ok();
}
