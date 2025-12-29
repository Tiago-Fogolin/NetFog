use netfog::{_Graph, HtmlWriter, Writeable};
use netfog::layout::layout::Layout;

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
    let conteudo_svg = graph.output_svg(Layout::Random, true);

    std::fs::write("test_output.svg", &conteudo_svg).expect("Erro ao salvar");
}

#[test]
#[ignore]
fn test_html_with_svg() {
    let mut graph = create_simple_graph();
    graph.output_html("output.html", Layout::Random, true);
}
