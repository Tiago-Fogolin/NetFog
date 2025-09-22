use std::fs;
use std::io::Error;


pub trait Writeable {
    fn write_file(&self, path: &str) -> Result<(), Error>;
}

pub struct HtmlWriter {}

impl Writeable for HtmlWriter {
    fn write_file(&self, path: &str) -> Result<(), Error> {
        let content = "Hello, Rust!";
    
        fs::write(path, content)?;

        println!("Arquivo criado!");
        return Ok(());
    }
}