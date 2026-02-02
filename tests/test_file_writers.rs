use netfog::file_reader_core::file_reader::read_json_file;
use netfog::{_Graph, HtmlWriter, Writeable};
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
#[ignore]
fn test_html() {
    let writer = HtmlWriter {};
    writer.write_file("output.html", "test").expect("Erro ao criar arquivo");
}

#[test]
#[ignore]
fn test_svg() {
    let mut graph = create_simple_graph();
    let style = GraphStyle::default();
    let conteudo_svg = graph.output_svg(Layout::Random, true, style);

    std::fs::write("test_output.svg", &conteudo_svg).expect("Erro ao salvar");
}

#[test]
#[ignore]
fn test_html_with_svg() {
    let mut graph = create_simple_graph();
    let mut style = GraphStyle::default();
    style.dynamic_line_size = false;
    graph.output_html("output.html", Layout::Spring, true, style);
}

#[test]
#[ignore]
fn test_output_net_file() {

    let style = GraphStyle::default();
    let mut graph = read_net_file("data.net").expect("Falha ao ler o arquivo .net");
    graph.output_net_file("output.net");
    let mut graph2 = read_net_file("output.net").expect("Falha ao ler o arquivo .net");
    graph2.output_html("output2.html", Layout::Random, false,style);
}

#[test]
#[ignore]
fn test_output_json_file() {
    let style = GraphStyle::default();
    let mut graph = read_json_file("arquivo_json.json").expect("Falha ao ler o arquivo .json");
    graph.output_json_file("output.json");
    let mut graph2 = read_json_file("output.json").expect("Falha ao ler o arquivo .json");
    graph2.output_html("output2.html", Layout::Random, false,style);
}
