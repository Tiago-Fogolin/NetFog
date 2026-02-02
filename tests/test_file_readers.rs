use netfog::file_reader_core::file_reader::read_json_file;
use netfog::{file_reader_core::file_reader::read_net_file, *};
use netfog::layout::layout::Layout;
use netfog::layout::style::GraphStyle;


#[test]
#[ignore]
fn test_read_net_file() {
    let style = GraphStyle::default();
    let mut graph = read_net_file("data.net").expect("Falha ao ler o arquivo .net");
    graph.output_html("output.html", Layout::Random, false, style);
}

#[test]
#[ignore]
fn test_read_json_file() {
    let style = GraphStyle::default();
    let mut graph = read_json_file("arquivo_json.json").expect("Falha ao ler o arquivo .json");
    graph.output_html("output.html", Layout::Random, false, style);
}
