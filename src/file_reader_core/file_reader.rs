use crate::_Graph;
use std::io::Error;
use serde::{Deserialize, Serialize};
use crate::file_reader_core::json_streaming::read_json_file_streaming;
use crate::file_reader_core::net_file_streaming::read_net_file_streaming;

pub fn read_net_file<S: crate::graph_core::graph_structure_interface::IGraphStructure + Default>(file_path: &str) -> Result<_Graph<S>, Error> {
    return read_net_file_streaming(file_path);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonNode {
    pub label: String,
    pub x: Option<f64>,
    pub y: Option<f64>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonConnection {
    pub source: String,
    pub target: String,
    pub weight: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonGraph {
    pub nodes: Vec<JsonNode>,
    pub edges: Option<Vec<JsonConnection>>,
    pub arcs: Option<Vec<JsonConnection>>,
}

pub fn read_json_file<S: crate::graph_core::graph_structure_interface::IGraphStructure + Default>(file_path: &str) -> Result<_Graph<S>, Error> {
    return read_json_file_streaming(file_path);
}
