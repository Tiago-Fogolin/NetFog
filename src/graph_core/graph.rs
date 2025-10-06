use crate::graph_core::node::{self, _Node};
use std::hash::Hash;
use std::hint::unreachable_unchecked;
use std::{collections::HashMap};
use std::cell::RefCell;
use std::rc::{Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionProperty  {
    From(String),
    To(String),
    Weight(f32),
    Directed(bool)
}

pub type PositionMap = HashMap<String, i32>;
pub type ConnectionData = HashMap<String, ConnectionProperty>;
pub type ConnectionsList = Vec<ConnectionData>;

pub struct _Graph {
    pub nodes: Vec<Rc<RefCell<_Node>>>,
    normalized_positions: HashMap<String, PositionMap>
}


fn create_nodes_from_labels(size: usize, labels: Option<Vec<String>>) -> Vec<Rc<RefCell<_Node>>> {
    let labels = labels.unwrap_or_else(|| {
        (0..size).map(|x| x.to_string()).collect()
    });
    let mut node_list: Vec<Rc<RefCell<_Node>>> = Vec::new(); 
    for label in labels {
        let new_node = _Node {
                label,
                connections: Vec::new(),
            };
        
        node_list.push(Rc::new(RefCell::new(new_node)));
    }

    return node_list;
}

fn create_node_hashmap(nodes: &Vec<Rc<RefCell<_Node>>>, start_index: usize)  -> HashMap<usize, String> {
    let node_hash: HashMap<usize, String> = nodes
                .iter()
                .enumerate()
                .map(|(i, x)| (i + start_index, x.borrow().label.clone()))
                .collect();

    return node_hash;
}

fn invert_node_hashmap(node_hashmap: HashMap<usize, String>) -> HashMap<String, usize> {
    let mut inverted_node_hash: HashMap<String, usize> = HashMap::new();

    for (key, value) in node_hashmap.iter() {
        inverted_node_hash.insert(value.clone(), *key);
    }

    return inverted_node_hash;
}


impl _Graph {
    pub fn add_node(&mut self, label: String) {

        let exists = self.nodes.iter().any(|node_rc| {
            node_rc.borrow().label == label
        });

        if exists {
            println!("Node with label '{}' already exists!", label);
            return;
        }

        let mut new_node = _Node{
            label: label,
            connections: Vec::new()
        };

        self.nodes.push(Rc::new(RefCell::new(new_node)));
    }

    // O(n) bullshit, will be fixed in future updates, same thing in the add_node method
    pub fn create_connection(&mut self, from: String, to: String, weight: f32, directed: Option<bool>) {
        let directed = Some(directed.unwrap_or(false));

        let from_node = self.nodes.iter().find(|n| n.borrow().label == from)
                .expect("Node 'from' not found")
                .clone();

            let to_node = self.nodes.iter().find(|n| n.borrow().label == to)
                .expect("Node 'to' not found")
                .clone();

        let to_node_ref = Rc::clone(&to_node);

        from_node.borrow_mut().add_connection(to_node_ref, weight, directed);
    }

    pub fn get_connections(&mut self) -> ConnectionsList {
        let mut all_connections = ConnectionsList::new();

        for n in &mut self.nodes {
            let mut node = n.borrow();

            for conn in node.connections.iter() {
                let mut formatted_conn = ConnectionData::new();


                formatted_conn.insert("from".to_string(), ConnectionProperty::From(node.label.clone()));

                if let Some(to_node_rc) = conn.node.upgrade() {
                    
                    let to_label = to_node_rc.borrow().label.clone();
                    formatted_conn.insert("to".to_string(), ConnectionProperty::To(to_label));
                } else {
                    formatted_conn.insert("to".to_string(), ConnectionProperty::To("[Removed]".to_string()));
                };

                formatted_conn.insert("weight".to_string(), ConnectionProperty::Weight(conn.weight));
                formatted_conn.insert("directed".to_string(), ConnectionProperty::Directed(conn.directed));

                all_connections.push(formatted_conn);
            }
        }

        return all_connections;
    }

    pub fn generate_adjacency_matrix(&mut self) -> Vec<Vec<f32>> {
        let matrix_size = self.nodes.len();
        let mut adj_matrix: Vec<Vec<f32>> = vec![vec![0.; matrix_size]; matrix_size];
        let node_hash = invert_node_hashmap(create_node_hashmap(&self.nodes, 0));
        let connections = self.get_connections();

        for conn in connections {

            let from_label = match &conn["from"] {
                ConnectionProperty::From(s) => s,
                _ => unreachable!(),
            };

            let to_label = match &conn["to"] {
                ConnectionProperty::To(s) => s,
                _ => unreachable!(),
            };

            let weight = match &conn["weight"] {
                ConnectionProperty::Weight(w) => w,
                _ => unreachable!(),
            };

            let directed = match &conn["directed"] {
                ConnectionProperty::Directed(d) => d,
                _ => unreachable!(),
            };

            let i = node_hash[from_label];
            let j = node_hash[to_label];
            adj_matrix[i][j] = *weight;

            if !*directed {
                adj_matrix[j][i] = *weight;
            }

        }

        return adj_matrix;

    }

    pub fn get_total_weight(&mut self) -> f32 {
        let total_weight: f32 = self.get_connections()
            .iter()
            .map(|conn| {
                if let ConnectionProperty::Weight(w) = conn["weight"] {
                    w
                } else {
                    0.
                }
            })
            .sum();

        return total_weight;
    }

    pub fn get_node_count(&self) -> usize {
        return self.nodes.len();
    }

    pub fn get_edge_count(&mut self) -> usize {
        return self.get_connections().len();
    }

    pub fn get_density(&mut self, directed: Option<bool>) -> f32 {
        let edge_count = self.get_edge_count() as f32;
        let node_count = self.get_node_count() as f32;
        let directed = directed.unwrap_or(false);
        let multiply = if directed {
            2.
        } else {
            1.
        };

        let density: f32 = (multiply * edge_count) / (node_count * (node_count - 1.));

        return density;
    }

    pub fn get_mean_weight(&mut self) -> f32 {
        return self.get_total_weight() / self.get_connections().len() as f32;
    }

    pub fn compute_degrees(&mut self, node_label: &str) -> HashMap<String, i32> {
        let connections = self.get_connections();

        let mut degrees: HashMap<String, i32> = HashMap::new();
        degrees.insert("in_degree".to_string(), 0);
        degrees.insert("out_degree".to_string(), 0);
        degrees.insert("total_degree".to_string(), 0);
        degrees.insert("undirected_degree".to_string(), 0);

        for conn in  connections {
            let directed = match &conn["directed"] {
                ConnectionProperty::Directed(d) => d,
                _ => unreachable!()
            };

            let from = match &conn["from"] {
                ConnectionProperty::From(f) => f,
                _ => unreachable!()
            };

            let to = match &conn["to"] {
                ConnectionProperty::To(f) => f,
                _ => unreachable!()
            };

            if *directed {
                if from == node_label {
                    *degrees.entry("out_degree".to_string()).or_insert(0) += 1;
                    *degrees.entry("total_degree".to_string()).or_insert(0) += 1;
                }

                if to == node_label {
                    *degrees.entry("in_degree".to_string()).or_insert(0) += 1;
                    *degrees.entry("total_degree".to_string()).or_insert(0) += 1;
                }

                continue;
            }

            if to == node_label || from == node_label {
                *degrees.entry("undirected_degree".to_string()).or_insert(0) += 1;
            }
        }

        return degrees;
    }

    pub fn get_all_nodes_degrees(&mut self) -> HashMap<String, HashMap<String, i32>> {
        let mut degree_hash: HashMap<String, HashMap<String, i32>> = HashMap::new();

        let labels: Vec<String> = self.nodes
                .iter()
                .map(|n| n.borrow().label.clone())
                .collect();

        for label in labels {
            let degree = self.compute_degrees(&label);
            degree_hash.insert(label,degree);
        }

        return degree_hash;
    }

    pub fn get_average_degree(&mut self, directed: Option<bool>) -> f32 {
        let directed = directed.unwrap_or(false);
        let multiply = if directed {
            1
        } 
        else {
            2
        } as f32;

        let edge_count = self.get_edge_count() as f32;
        let node_count = self.get_node_count() as f32;

        let mean = (multiply * edge_count) / (node_count);

        return mean;
    }

    pub fn get_centrality_degrees(&mut self, node_label: &str) -> HashMap<&str, f32> {
        let mut centralities: HashMap<&str, f32> = HashMap::new();

        let degrees = self.compute_degrees(node_label);
        let node_count = self.get_node_count();

        if node_count <= 1 {
            return centralities;
        }

        centralities.insert("out_centrality", degrees["out_degree"] as f32 / (node_count - 1) as f32); 
        centralities.insert("in_centrality", degrees["in_degree"] as f32 / (node_count - 1) as f32); 
        centralities.insert("total_centrality", degrees["total_degree"] as f32 / (node_count - 1) as f32); 
        centralities.insert("undirected_centrality", degrees["undirected_degree"] as f32 / (node_count - 1) as f32); 

        return centralities;
    }

}


impl _Graph {
    pub fn default() -> Self {
        return _Graph {
            nodes: Vec::new(),
            normalized_positions: HashMap::new()
        };
    }

    pub fn from_adjacency_matrix(adj_matrix: Vec<Vec<f32>>, directed: Option<bool>, custom_labels: Option<Vec<String>>) -> Self {
        let mut adj_matrix_graph = _Graph::default();

        adj_matrix_graph.nodes = create_nodes_from_labels(adj_matrix.len(), custom_labels);
        let node_hash = create_node_hashmap(&adj_matrix_graph.nodes, 0);
        
        for i in 0..adj_matrix.len() {
            for j in 0..adj_matrix.len() {
                let weight = adj_matrix[i][j]; 

                if weight != 0. {
                    adj_matrix_graph.create_connection(
                        node_hash.get(&i).expect("Node not found").clone(), 
                        node_hash.get(&j).expect("Node not found").clone(), 
                        weight,
                        directed
                    );
                }

            }
        }

        return adj_matrix_graph;
    }
}