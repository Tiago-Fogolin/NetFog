use std::{fs, io};
use std::io::Error;


fn read_template(path: &str) -> io::Result<String> {
    return fs::read_to_string(path);
}

pub trait Writeable {
    fn write_file(&self, path: &str, content: &str) -> Result<(), Error>;
}

pub struct HtmlWriter {}

impl Writeable for HtmlWriter {
    fn write_file(&self, path: &str, content: &str) -> Result<(), Error> {
        let html_string = read_template("src/file_writer_core/template.html")?;
        let js_string = read_template("src/file_writer_core/script.js")?;

        let html_with_svg = html_string.replace("ESCAPE_SVG", content);
        let final_string = html_with_svg.replace("ESCAPE_SCRIPT", &js_string);

        fs::write(path, final_string)?;

        return Ok(());
    }
}
