use std::{fs, io};
use std::io::Error;
use std::rc::Rc;
use std::cell::RefCell;
use crate::{_Node};
use crate::layout::layout::{normalize_x, normalize_y};
use crate::file_reader_core::file_reader::{JsonConnection, JsonGraph, JsonNode};
use std::fs::File;
use std::io::BufWriter;

fn read_template(path: &str) -> io::Result<String> {
    return fs::read_to_string(path);
}

pub trait Writeable {
    fn write_file(&self, path: &str, content: &str) -> Result<(), Error>;
}

pub struct HtmlWriter {}

impl Writeable for HtmlWriter {
    fn write_file(&self, path: &str, content: &str) -> Result<(), Error> {
        let html_string = include_str!("../file_writer_core/template.html");
        let js_string = include_str!("../file_writer_core/script.js");

        let html_with_svg = html_string.replace("ESCAPE_SVG", content);
        let final_string = html_with_svg.replace("ESCAPE_SCRIPT", &js_string);

        fs::write(path, final_string)?;

        return Ok(());
    }
}


pub fn write_net_file(path: &str, nodes: Vec<Rc<RefCell<_Node>>>) -> Result<(), Error>{
    let mut content_string: String = String::new();

    let mut edges: Vec<(usize,usize,f32)> = Vec::new();
    let mut arcs: Vec<(usize,usize,f32)> = Vec::new();

    content_string += "*Vertices\n";
    for n in &nodes {
        let node = n.borrow();
        content_string += &format!("{} \"{}\"", node.index.unwrap(), node.label);
        if !node.x.is_none() && !node.y.is_none() {
            content_string += &format!(" {} {}", normalize_x(node.x.unwrap()), normalize_y(node.y.unwrap()));
        }
        content_string += "\n";

        for conn in node.connections.iter() {
            if let Some(rc_node) = conn.node.upgrade() {
                let connected_node = rc_node.borrow();

                if conn.directed {
                    arcs.push(
                        (
                            node.index.unwrap(),
                            connected_node.index.unwrap(),
                            conn.weight
                        )
                    );
                }
                else {
                    edges.push(
                        (
                            node.index.unwrap(),
                            connected_node.index.unwrap(),
                            conn.weight
                        )
                    );
                }
            }
        }
    }

    if !edges.is_empty() {
        content_string += "*Edges\n";
        for (from_index ,to_index, weight) in edges {
            content_string += &format!("{} {} {}\n", from_index, to_index, weight);
        }
    }

    if !arcs.is_empty() {
        content_string += "*Arcs\n";
        for (from_index ,to_index, weight) in arcs {
            content_string += &format!("{} {} {}\n", from_index, to_index, weight);
        }
    }

    fs::write(path, content_string)?;

    return Ok(());
}

pub fn write_json_file(path: &str, nodes: Vec<Rc<RefCell<_Node>>>) -> Result<(), Error>{
    let mut json_nodes: Vec<JsonNode> = Vec::new();
    let mut json_edges: Vec<JsonConnection> = Vec::new();
    let mut json_arcs: Vec<JsonConnection> = Vec::new();

    for n in &nodes {
        let node = n.borrow();

        let json_node = JsonNode {
            label: node.label.clone(),
            x: node.x,
            y: node.y
        };
        json_nodes.push(json_node);

        for conn in node.connections.iter() {
            if let Some(rc_node) = conn.node.upgrade() {
                let connected_node = rc_node.borrow();
                let json_conn = JsonConnection {
                    source: node.label.clone(),
                    target: connected_node.label.clone(),
                    weight: conn.weight
                };

                if conn.directed {
                    json_arcs.push(json_conn);
                }
                else {
                    json_edges.push(json_conn);
                }
            }
        }
    }

    let json_graph = JsonGraph {
      nodes: json_nodes,
      edges: Some(json_edges),
      arcs: Some(json_arcs)
    };

    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &json_graph)?;

    return Ok(());
}
