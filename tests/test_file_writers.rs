use netfog::{HtmlWriter, Writeable};

#[test]
#[ignore]
fn test_html() {
    let writer = HtmlWriter {};
    writer.write_file("output.html").expect("Erro ao criar arquivo");
}
