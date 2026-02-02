use crate::{_Node,_Graph};
use std::{collections::HashMap};
use std::fs::File;
use std::fs;
use std::io::{self, BufRead, BufReader, Error};
use crate::layout::layout::{denormalize_x, denormalize_y};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub fn read_net_file(file_path: &str) -> Result<_Graph, Error> {

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut graph = _Graph::default();

    let mut index_label_map: HashMap<String, String> = HashMap::new();

    let mut reading_nodes = false;
    let mut reading_edges = false;
    let mut reading_arcs  = false;
    for line in reader.lines(){
        let mut line = line?;
        line = line.replace("\n", "");
        line = line.trim().to_string();

        if line == "" {
            continue;
        }

        if line.contains("*vertices") || line.contains("*Vertices") {
            reading_edges = false;
            reading_arcs  = false;
            reading_nodes = true;
            continue
        }

        if line.contains("*edges") || line.contains("*Edges") {
            reading_edges = true;
            reading_arcs  = false;
            reading_nodes = false;
            continue
        }

        if line.contains("*arcs") || line.contains("*Arcs") {
            reading_edges = false;
            reading_arcs  = true;
            reading_nodes = false;
            continue
        }

        if reading_nodes {
            let start_node = line.find('"');
            let end_node = line.rfind('"');
            let node_label = line[start_node.unwrap()+1..end_node.unwrap()].to_string();

            let elements: Vec<&str> = line.split(' ').collect();
            let node_index = 0;


            if elements.len() == 2 {
                let node_index = elements[0];
                graph.add_node(node_label.to_string());
                index_label_map.insert(node_index.to_string(), node_label);
            }
            else if elements.len() >= 4 {
                let node_index = elements[0];
                let x_pos = elements[elements.len() - 2].parse::<f64>().expect("Invalid x pos");
                let y_pos = elements[elements.len() - 1].parse::<f64>().expect("Invalid y pos");

                graph.add_node_with_pos(
                    node_label.to_string(),
                    denormalize_x(x_pos),
                    denormalize_y(y_pos));

                index_label_map.insert(node_index.to_string(), node_label.clone());

            }
        }

        if reading_edges {
            let elements: Vec<&str> = line.split(' ').collect();
            let from_index = elements[0];
            let to_index = elements[1];
            let weight = elements[2];


            graph.create_connection(
                index_label_map[from_index].clone(),
                index_label_map[to_index].clone(),
                weight.parse::<f32>().expect("Invalid weight"),
                Some(false)
            );

        }

        if reading_arcs {
            let elements: Vec<&str> = line.split(' ').collect();
            let from_index = elements[0];
            let to_index = elements[1];
            let weight = elements[2];

            graph.create_connection(
                index_label_map[from_index].clone(),
                index_label_map[to_index].clone(),
                weight.parse::<f32>().expect("Invalid weight"),
                Some(true)
            );

        }
    }

    return Ok(graph);
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

pub fn read_json_file(file_path: &str) -> Result<_Graph, Error> {
    let path = Path::new(file_path);
    let content = fs::read_to_string(path)?;

    let json_graph: JsonGraph = serde_json::from_str(&content)?;

    let mut new_graph = _Graph::default();
    for node in json_graph.nodes {
        if !node.x.is_none() && !node.y.is_none() {
            new_graph.add_node_with_pos(node.label, node.x.unwrap(), node.y.unwrap());
        }
        else {
            new_graph.add_node(node.label);
        }
    }
    if !json_graph.edges.is_none() {
        for edge in json_graph.edges.unwrap() {
            new_graph.create_connection(edge.source, edge.target, edge.weight, Some(false));
        }
    }

    if !json_graph.arcs.is_none() {
        for arc in json_graph.arcs.unwrap() {
            new_graph.create_connection(arc.source, arc.target, arc.weight, Some(true));
        }
    }


    return Ok(new_graph);
}
