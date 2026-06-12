use crate::_Graph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_net_file_streaming<S: crate::graph_core::graph_structure_interface::IGraphStructure + Default>(file_path: &str) -> Result<_Graph<S>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = _Graph::default();

    let mut index_label_map: HashMap<String, String> = HashMap::new();

    let mut reading_nodes = false;
    let mut reading_edges = false;
    let mut reading_arcs  = false;

    for line in reader.lines() {
        let line = line?.trim().to_string();

        if line.is_empty() {
            continue;
        }

        if line.contains("*vertices") || line.contains("*Vertices") {
            reading_nodes = true;
            reading_edges = false;
            reading_arcs  = false;
            continue;
        }

        if line.contains("*edges") || line.contains("*Edges") {
            reading_nodes = false;
            reading_edges = true;
            reading_arcs  = false;
            continue;
        }

        if line.contains("*arcs") || line.contains("*Arcs") {
            reading_nodes = false;
            reading_edges = false;
            reading_arcs  = true;
            continue;
        }

        if reading_nodes {
            let start_node = line.find('"');
            let end_node = line.rfind('"');
            let node_label = line[start_node.unwrap() + 1..end_node.unwrap()].to_string();
            let elements: Vec<&str> = line.split(' ').collect();

            if elements.len() == 2 {
                let node_index = elements[0];
                graph.add_node(node_label.clone());
                index_label_map.insert(node_index.to_string(), node_label);
            } else if elements.len() >= 4 {
                let node_index = elements[0];
                let x_pos = elements[elements.len() - 2].parse::<f64>().expect("Invalid x pos");
                let y_pos = elements[elements.len() - 1].parse::<f64>().expect("Invalid y pos");
                graph.add_node_with_pos(node_label.clone(), x_pos, y_pos);
                index_label_map.insert(node_index.to_string(), node_label);
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
                Some(false),
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
                Some(true),
            );
        }
    }

    return Ok(graph);
}
