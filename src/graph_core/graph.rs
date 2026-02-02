use crate::layout::style::GraphStyle;
use crate::{HtmlWriter, Node, Writeable};
use crate::graph_core::node::{self, _Node};
use crate::file_writer_core::file_writer::{write_json_file, write_net_file};
use std::f64;
use std::hash::Hash;
use std::{collections::HashMap, collections::HashSet, collections::VecDeque};
use std::cell::RefCell;
use std::rc::{Rc};
use crate::svg_creation::svg_creation::Svg;
use crate::layout::layout::Layout;
use crate::file_reader_core::file_reader::{read_json_file, read_net_file};

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
    pub positions_set: bool,
    _current_index: usize
}


fn create_nodes_from_labels(size: usize, labels: Option<Vec<String>>) -> Vec<Rc<RefCell<_Node>>> {
    let labels = labels.unwrap_or_else(|| {
        (0..size).map(|x| x.to_string()).collect()
    });
    let mut node_list: Vec<Rc<RefCell<_Node>>> = Vec::new();
    let mut current_index: usize = 0;
    for label in labels {
        let new_node = _Node {
                label,
                connections: Vec::new(),
                index: Some(current_index),
                x: None,
                y: None,

            };

        current_index += 1;

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

        let new_node = _Node{
            label: label,
            connections: Vec::new(),
            index: Some(self._current_index),
            x: None,
            y: None,

        };

        self._current_index += 1;

        self.nodes.push(Rc::new(RefCell::new(new_node)));
    }


    pub fn add_node_with_pos(&mut self, label: String, x:f64, y:f64) {

        let exists = self.nodes.iter().any(|node_rc| {
            node_rc.borrow().label == label
        });

        if exists {
            println!("Node with label '{}' already exists!", label);
            return;
        }

        let new_node = _Node{
            label: label,
            connections: Vec::new(),
            index: Some(self._current_index),
            x: Some(x),
            y: Some(y),

        };

        self._current_index += 1;
        self.positions_set = true;

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

    pub fn node_by_label(&self, label: &str) -> Option<Rc<RefCell<_Node>>> {
        for n in &self.nodes {
            if n.borrow().label == label {
                return Some(Rc::clone(n));
            }
        }
        return None;
    }

    pub fn get_connections(&mut self) -> ConnectionsList {
        let mut all_connections = ConnectionsList::new();

        for n in &mut self.nodes {
            let node = n.borrow();

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

    pub fn get_node_strength(&mut self, node_label: &str) -> HashMap<&str, f32> {
        let connections = self.get_connections();

        let mut strengths: HashMap<&str, f32> = HashMap::new();

        strengths.insert("out_strength", 0.);
        strengths.insert("in_strength", 0.);
        strengths.insert("total_strength", 0.);

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

            if from_label == node_label {
                *strengths.get_mut("out_strength").unwrap() += *weight;

                if !*directed {
                    *strengths.get_mut("in_strength").unwrap() += *weight;
                }
            }

            if to_label == node_label {
                *strengths.get_mut("in_strength").unwrap() += *weight;

                if !*directed {
                    *strengths.get_mut("out_strength").unwrap() += *weight;
                }
            }

        }

        *strengths.get_mut("total_strength").unwrap() = strengths["out_strength"] + strengths["in_strength"];


        return strengths;
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

    pub fn get_degree_distribution(&mut self) -> HashMap<&str, HashMap<i32 ,f32>>{
        let mut computed_degrees: Vec<HashMap<String, i32>> = Vec::new();

        let mut nodes = self.nodes.clone();

        for n in &mut nodes {
            let mut node = n.borrow();

            computed_degrees.push(self.compute_degrees(&node.label));
        }

        let node_count = self.get_node_count();
        let mut count_in_degree: HashMap<i32, i32> = HashMap::new();
        let mut count_out_degree: HashMap<i32, i32> = HashMap::new();
        let mut count_undirected_degree: HashMap<i32, i32> = HashMap::new();



        for mut computed_degree in computed_degrees {
            let in_degree = *computed_degree.get_mut("in_degree").unwrap();
            let out_degree = *computed_degree.get_mut("out_degree").unwrap();
            let undirected_degree = *computed_degree.get_mut("undirected_degree").unwrap();

            *count_in_degree.entry(in_degree).or_insert(0) += 1;
            *count_out_degree.entry(out_degree).or_insert(0) += 1;
            *count_undirected_degree.entry(undirected_degree).or_insert(0) += 1;
        }

        let undirected_distribution: HashMap<i32, f32> = count_undirected_degree
            .iter()
            .map(|(&k, &v)| (k, v as f32 / node_count as f32))
            .collect();

        let in_distribution: HashMap<i32, f32> = count_in_degree
            .iter()
            .map(|(&k, &v)| (k, v as f32 / node_count as f32))
            .collect();

        let out_distribution: HashMap<i32, f32> = count_out_degree
            .iter()
            .map(|(&k, &v)| (k, v as f32 / node_count as f32))
            .collect();

        let mut distribution: HashMap<&str, HashMap<i32 ,f32>> = HashMap::new();

        distribution.insert("undirected_distribution", undirected_distribution);
        distribution.insert("in_distribution", in_distribution);
        distribution.insert("out_distribution", out_distribution);

        return distribution;
    }

    pub fn compute_entropy(&mut self) -> HashMap<&str, f32> {
        let mut result: HashMap<&str, f32> = HashMap::new();

        let dist: HashMap<&str, HashMap<i32 ,f32>> = self.get_degree_distribution();
        let mut in_entropy: f32 = 0.;
        let mut out_entropy: f32 = 0.;
        let mut undirected_entropy: f32 = 0.;

        for (degree, dist_value) in &dist["in_distribution"] {
            in_entropy += dist_value * dist_value.ln();
        }

        for (degree, dist_value) in &dist["out_distribution"] {
            out_entropy += dist_value * dist_value.ln();
        }

        for (degree, dist_value) in &dist["undirected_distribution"] {
            undirected_entropy += dist_value * dist_value.ln();
        }

        result.insert("in_entropy", -in_entropy);
        result.insert("out_entropy", -out_entropy);
        result.insert("undirected_entropy", -undirected_entropy);

        return result;
    }

    pub fn get_max_possible_entropy(&mut self) -> f64 {
        let nodes_minus_1 = (self.get_node_count() - 1) as f64;
        return nodes_minus_1.ln();
    }

    pub fn get_skewness(&mut self) -> HashMap<&str, f32> {

        fn _rank_degree_for_skewness(degree_collection: HashMap<String, i32>) -> Vec<(usize, i32)> {
            let mut sorted_degrees: Vec<(String, i32)> = degree_collection.into_iter().collect();
            sorted_degrees.sort_by(|a, b| b.1.cmp(&a.1));

            let mut ranked: Vec<(usize, i32)> = Vec::new();


            for i in 1..=sorted_degrees.len() {
                let (node_label, degree) = sorted_degrees[i - 1].clone();
                let rank = (i, degree);
                ranked.push(rank);
            }

            return ranked;
        }

        fn _get_ranked_degrees(all_nodes_degrees: HashMap<String, HashMap<String, i32>>) -> Vec<Vec<(usize, i32)>> {
            let mut ranked_degrees: Vec<Vec<(usize, i32)>>  = Vec::new();

            for degree_type in ["in_degree", "out_degree", "undirected_degree"] {
                let current_degree_map: HashMap<String, i32> = all_nodes_degrees
                    .iter()
                    .map(|(node_id, metrics)| {
                        let val = *metrics.get(degree_type).unwrap_or(&0);
                        (node_id.clone(), val)
                    })
                    .collect();

                let ranked: Vec<(usize, i32)> = _rank_degree_for_skewness(current_degree_map);
                ranked_degrees.push(ranked);
            }

            return ranked_degrees;

        }

        fn _compute_sknums(ranked_degrees: &Vec<Vec<(usize, i32)>>) -> Vec<f32> {
            let mut sknums: Vec<f32> = Vec::new();

            for ranked_degree in ranked_degrees {
                let mut result: f32 = 0.;
                for (rank, degree) in ranked_degree {
                    result += ((*rank) as i32 * degree) as f32;
                }
                sknums.push(result);
            }

            return sknums;
        }

        fn _compute_sku(ranked_degrees: &Vec<Vec<(usize, i32)>>, node_count: usize) -> Vec<f32> {
            let mut mean_degrees: Vec<f32> = Vec::new();
            for ranked in ranked_degrees {
                if !ranked.is_empty() {
                    let degrees: Vec<i32> = ranked.iter().map(|(rank, degree)| *degree).collect();
                    let total = degrees.iter().sum::<i32>() as f32;
                    let count = degrees.len() as f32;

                    mean_degrees.push(total / count);
                }
                else {
                    mean_degrees.push(0.);
                }
            }

            let mut skus: Vec<f32> = Vec::new();

            for degree in mean_degrees {
                let node_count_f = node_count as f32;
                let sku = degree * (node_count_f * (node_count_f + 1.) / 2.);
                skus.push(sku);
            }

            return skus;
        }

        let mut result: HashMap<&str, f32> = HashMap::new();

        let degrees = self.get_all_nodes_degrees();

        let ranked_degrees = _get_ranked_degrees(degrees);

        let sknums = _compute_sknums(&ranked_degrees);

        let skus = _compute_sku(&ranked_degrees, self.get_node_count());

        result.insert("in_skewness", sknums[0] / skus[0]);
        result.insert("out_skewness", sknums[1] / skus[1]);
        result.insert("undirected_skewness", sknums[2] / skus[2]);

        return result;
    }

    /*
     * This method has almost no purpouse right now, but the idea
     * is that it'll be able to get a function that will be applied to each
     * node in the future (same thing to bfs)
     */
    pub fn dfs(&mut self, start_node_label: &str) -> Vec<String> {
        let mut final_order: Vec<String> = Vec::new();

        let mut visited: HashSet<String> = HashSet::new();

        let mut stack: Vec<Rc<RefCell<_Node>>> = Vec::new();
        let starting_node: Rc<RefCell<_Node>> = self.node_by_label(start_node_label).expect("Error: Initial node not found");

        stack.push(starting_node);

        while let Some(n) = stack.pop() {
            let node = n.borrow();

            if !visited.insert(node.label.clone()) {
                continue;
            }

            final_order.push(node.label.clone());

            for conn in node.connections.iter().rev() {
                if let Some(rc_node) = conn.node.upgrade() {
                    let conn_node = rc_node.borrow();

                    if !visited.contains(&conn_node.label) {
                        stack.push(Rc::clone(&rc_node));
                    }
                }
            }
        }

        return final_order;

    }

    pub fn bfs(&mut self, start_node_label: &str) -> Vec<String> {
        let mut final_order: Vec<String> = Vec::new();

        let mut q: VecDeque<Rc<RefCell<_Node>>> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();

        let starting_node: Rc<RefCell<_Node>> = self.node_by_label(start_node_label).expect("Error: Initial node not found");


        visited.insert(start_node_label.to_string());
        q.push_back(starting_node);

        while let Some(n) = q.pop_front() {
            let node = n.borrow();

            final_order.push(node.label.clone());

            for conn in &node.connections {
                if let Some(rc_node) = conn.node.upgrade() {
                    let conn_node = rc_node.borrow();

                    if visited.insert(conn_node.label.clone()) {
                        q.push_back(Rc::clone(&rc_node));
                    }
                }
            }
        }


        return final_order;
    }

    pub fn dijkstra(&mut self, start_node_label: &str) -> HashMap<String, f64>{
        let size = self.nodes.len();
        let node_ref = self.node_by_label(start_node_label).expect("Node not found");
        let mut distances: HashMap<String, f64> = HashMap::new();

        for i in 0..size {
            let lbl = self.nodes[i].borrow().label.clone();
            distances.insert(lbl, f64::INFINITY);
        }

        distances.insert(node_ref.borrow().label.clone(), 0.);

        let mut visited = vec![false; size];
        let adj_matrix = self.generate_adjacency_matrix();

        for _ in 0..size {
            let mut min_distance = f64::INFINITY;
            let mut u: Option<usize> = None;

            for i in 0..size {
                if !visited[i] && distances[&self.nodes[i].borrow().label] < min_distance {
                    min_distance = distances[&self.nodes[i].borrow().label];
                    u = Some(i);
                }
            }

            if u == None {
                break;
            }
            let u: usize = u.unwrap();
            visited[u] = true;


            for v in 0..size {
                if adj_matrix[u][v] != 0. && !visited[v] {
                    let alt = distances[&self.nodes[u].borrow().label] as f32 + adj_matrix[u][v];
                    if alt < distances[&self.nodes[v].borrow().label] as f32{
                        distances.insert(self.nodes[v].borrow().label.clone(), alt as f64);
                    }
                }
            }
        }

        return distances;

    }

    pub fn output_svg(&mut self, layout: Layout, override_positions: bool, style: GraphStyle) -> String {
        let mut svg: Svg = Svg::new();
        let connections = self.get_connections();
        let svg_string = svg.get_svg(&self.nodes, &connections, layout, self.positions_set, override_positions, style);
        return svg_string;
    }

    pub fn output_html(&mut self, file_name: &str, layout: Layout, override_positions: bool, style: GraphStyle) {
        let svg_string = self.output_svg(layout, override_positions, style);
        let html_writer = HtmlWriter{};
        html_writer.write_file(file_name, &svg_string).expect("Error while creating the file");
    }

    pub fn output_net_file(&mut self, path: &str) {
        write_net_file(path, self.nodes.clone()).expect("Error while creating the file");
    }

    pub fn output_json_file(&mut self, path: &str) {
        write_json_file(path, self.nodes.clone()).expect("Error while creating the file");
    }
}


impl _Graph {
    pub fn default() -> Self {
        return _Graph {
            nodes: Vec::new(),
            positions_set: false,
            _current_index: 0
        };
    }

    pub fn from_net_file(path: &str) -> Self {
        let new_graph = read_net_file(path).expect("Failed to read .net file");
        return new_graph;
    }

    pub fn from_json_file(path: &str) -> Self {
        let new_graph = read_json_file(path).expect("Failed to read .json file");
        return new_graph;
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
