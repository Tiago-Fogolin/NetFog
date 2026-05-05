use netfog::file_reader_core::file_reader::{read_edge_list_file, read_json_file, read_mtx_file};
use netfog::{HtmlWriter, Writeable};
use netfog::TestGraph as _Graph;
use netfog::layout::layout::Layout;
use netfog::{file_reader_core::file_reader::read_net_file, *};
use netfog::layout::style::GraphStyle;

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
fn test_html() {
    let writer = HtmlWriter {};
    writer.write_file("output_mock_test.html", "test").expect("Erro ao criar arquivo");
    std::fs::remove_file("output_mock_test.html").ok();
}

#[test]
fn test_svg() {
    let mut graph = create_simple_graph();
    let style = GraphStyle::default();
    let conteudo_svg = graph.output_svg(Layout::Random, true, style);

    std::fs::write("test_output_mock.svg", &conteudo_svg).expect("Erro ao salvar");
    std::fs::remove_file("test_output_mock.svg").ok();
}

#[test]
fn test_html_with_svg() {
    let mut graph = create_simple_graph();
    let mut style = GraphStyle::default();
    style.dynamic_line_size = false;
    graph.output_html("output_svg_mock.html", Layout::Spring, true, style);
    std::fs::remove_file("output_svg_mock.html").ok();
}

#[test]
fn test_output_net_file() {
    let mut graph = create_simple_graph();
    graph.output_net_file("mock_output.net");
    let mut graph2: _Graph = read_net_file("mock_output.net").expect("Falha ao ler o arquivo net");
    assert_eq!(graph2.get_node_count(), 4);
    
    std::fs::remove_file("mock_output.net").ok();
}

#[test]
fn test_output_json_file() {
    let mut graph = create_simple_graph();
    graph.output_json_file("mock_output.json");
    let mut graph2: _Graph = read_json_file("mock_output.json").expect("Falha ao ler o arquivo .json");
    assert_eq!(graph2.get_node_count(), 4);

    std::fs::remove_file("mock_output.json").ok();
}

#[test]
fn test_output_mtx_file() {
    let mut graph = create_simple_graph();
    graph.output_mtx_file("mock_output.mtx");
    let mut graph2: _Graph = read_mtx_file("mock_output.mtx").expect("Failed to read .mtx file");
    assert_eq!(graph2.get_node_count(), 4);
    assert_eq!(graph2.get_edge_count(), graph.get_edge_count());

    std::fs::remove_file("mock_output.mtx").ok();
}

#[test]
fn test_output_edge_list_file() {
    let mut graph = create_simple_graph();
    graph.output_edge_list_file("mock_output_edges.txt");
    let mut graph2: _Graph = read_edge_list_file("mock_output_edges.txt", false).expect("Failed to read edge list file");
    assert_eq!(graph2.get_node_count(), graph.get_node_count());
    assert_eq!(graph2.get_edge_count(), graph.get_edge_count());

    std::fs::remove_file("mock_output_edges.txt").ok();
}
