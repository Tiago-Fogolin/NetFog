use crate::_Graph;
use crate::graph_core::disk_graph::DiskGraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use redb::{Database, TableDefinition};
use crate::layout::layout::{denormalize_x, denormalize_y};

// Staging table: node file index (as string) → label string
const NET_INDEX_TO_LABEL: TableDefinition<&str, &str> = TableDefinition::new("net_index_to_label");

fn create_net_staging_db() -> (Database, PathBuf) {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let c = COUNTER.fetch_add(1, Ordering::SeqCst);
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir()
        .join(format!("netfog_net_staging_{}_{}.redb", t, c));
    let db = Database::create(&path).unwrap();
    {
        let write_tx = db.begin_write().unwrap();
        let _ = write_tx.open_table(NET_INDEX_TO_LABEL).unwrap();
        write_tx.commit().unwrap();
    }
    return (db, path);
}

fn commit_node_buffer(staging_db: &Database, buffer: &mut Vec<(String, String)>) {
    if buffer.is_empty() {
        return;
    }
    let write_tx = staging_db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(NET_INDEX_TO_LABEL).unwrap();
        for (idx, label) in buffer.iter() {
            table.insert(idx.as_str(), label.as_str()).unwrap();
        }
    }
    write_tx.commit().unwrap();
    buffer.clear();
}

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
                graph.add_node_with_pos(node_label.clone(), denormalize_x(x_pos), denormalize_y(y_pos));
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

pub fn read_net_file_streaming_disk(file_path: &str) -> Result<_Graph<DiskGraph>, Error> {
    let (staging_db, staging_path) = create_net_staging_db();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = _Graph::<DiskGraph>::default();

    let mut reading_nodes = false;
    let mut reading_edges = false;
    let mut reading_arcs  = false;
    let mut nodes_committed = false;

    // Buffer node entries and bulk-commit to staging redb when nodes section ends.
    // This frees the in-memory buffer before edge processing begins.
    let mut node_buffer: Vec<(String, String)> = Vec::new();

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
            if !nodes_committed {
                commit_node_buffer(&staging_db, &mut node_buffer);
                nodes_committed = true;
            }
            reading_nodes = false;
            reading_edges = true;
            reading_arcs  = false;
            continue;
        }

        if line.contains("*arcs") || line.contains("*Arcs") {
            if !nodes_committed {
                commit_node_buffer(&staging_db, &mut node_buffer);
                nodes_committed = true;
            }
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
                node_buffer.push((node_index.to_string(), node_label));
            } else if elements.len() >= 4 {
                let node_index = elements[0];
                let x_pos = elements[elements.len() - 2].parse::<f64>().expect("Invalid x pos");
                let y_pos = elements[elements.len() - 1].parse::<f64>().expect("Invalid y pos");
                graph.add_node_with_pos(node_label.clone(), denormalize_x(x_pos), denormalize_y(y_pos));
                node_buffer.push((node_index.to_string(), node_label));
            }
        }

        if reading_edges || reading_arcs {
            let elements: Vec<&str> = line.split(' ').collect();
            let from_index = elements[0];
            let to_index   = elements[1];
            let weight     = elements[2].parse::<f32>().expect("Invalid weight");
            let directed   = reading_arcs;

            let read_tx = staging_db.begin_read().unwrap();
            let table = read_tx.open_table(NET_INDEX_TO_LABEL).unwrap();

            let from_label = table.get(from_index).unwrap()
                .expect("Node index not found in staging db")
                .value()
                .to_string();
            let to_label = table.get(to_index).unwrap()
                .expect("Node index not found in staging db")
                .value()
                .to_string();

            drop(table);
            drop(read_tx);

            graph.create_connection(from_label, to_label, weight, Some(directed));
        }
    }

    drop(staging_db);
    let _ = std::fs::remove_file(&staging_path);

    graph.structure.flush_to_disk();

    return Ok(graph);
}
