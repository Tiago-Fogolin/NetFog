use crate::_Graph;
use crate::graph_core::disk_graph::DiskGraph;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_edge_list_file_streaming<S: crate::graph_core::graph_structure_interface::IGraphStructure + Default>(
    file_path: &str,
    directed: bool,
) -> Result<_Graph<S>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = _Graph::default();

    let mut node_set: HashSet<String> = HashSet::new();
    let mut edges: Vec<(String, String, f32)> = Vec::new();

    for raw in reader.lines() {
        let line = raw?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        if parts.len() < 2 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid edge list line (expected at least 2 columns): '{}'", trimmed),
            ));
        }

        let source = parts[0].to_string();
        let target = parts[1].to_string();

        let weight: f32 = if parts.len() >= 3 {
            parts[2].parse().map_err(|_| {
                Error::new(ErrorKind::InvalidData, format!("Invalid weight value: '{}'", parts[2]))
            })?
        } else {
            1.0
        };

        node_set.insert(source.clone());
        node_set.insert(target.clone());
        edges.push((source, target, weight));
    }

    let mut sorted_nodes: Vec<String> = node_set.into_iter().collect();
    sorted_nodes.sort();

    for node in sorted_nodes {
        graph.add_node(node);
    }

    for (source, target, weight) in edges {
        graph.create_connection(source, target, weight, Some(directed));
    }

    return Ok(graph);
}

pub fn read_edge_list_file_streaming_disk(file_path: &str, directed: bool) -> Result<_Graph<DiskGraph>, Error> {
    let mut graph = _Graph::<DiskGraph>::default();

    {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut node_set: HashSet<String> = HashSet::new();

        for raw in reader.lines() {
            let line = raw?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            if parts.len() < 2 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid edge list line (expected at least 2 columns): '{}'", trimmed),
                ));
            }

            node_set.insert(parts[0].to_string());
            node_set.insert(parts[1].to_string());
        }

        let mut sorted_nodes: Vec<String> = node_set.into_iter().collect();
        sorted_nodes.sort();

        for node in sorted_nodes {
            graph.add_node(node);
        }
    }

    {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for raw in reader.lines() {
            let line = raw?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            if parts.len() < 2 {
                continue;
            }

            let source = parts[0].to_string();
            let target = parts[1].to_string();

            let weight: f32 = if parts.len() >= 3 {
                parts[2].parse().map_err(|_| {
                    Error::new(ErrorKind::InvalidData, format!("Invalid weight value: '{}'", parts[2]))
                })?
            } else {
                1.0
            };

            graph.create_connection(source, target, weight, Some(directed));
        }
    }

    graph.structure.flush_to_disk();

    return Ok(graph);
}

