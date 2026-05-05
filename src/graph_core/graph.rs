use crate::layout::style::GraphStyle;
use crate::{HtmlWriter, Writeable};
use crate::graph_core::node::_Node;
use crate::file_writer_core::file_writer::{write_edge_list_file, write_json_file, write_mtx_file, write_net_file};
use std::f64;
use std::time::Instant;
use std::{collections::HashMap, collections::HashSet, collections::VecDeque};
use crate::graph_core::graph_metadata::GraphMetadata;
use crate::graph_core::graph_structure_interface::IGraphStructure;
use crate::graph_core::adjacency_list::AdjacencyList;
use crate::synthetic_graphs::core::SyntheticGraphType;
use crate::synthetic_graphs::{erdos_renyi, barabasi_albert, watts_strogatz};
use crate::svg_creation::svg_creation::Svg;
use crate::layout::layout::{Layout, get_layout_function};
use crate::file_reader_core::file_reader::{read_edge_list_file, read_json_file, read_mtx_file, read_net_file};
use crate::external_apis::core::{OpenAlexGraphType, NominatimResponse};
use crate::external_apis::openalex::dispatch_openalex_graph_creation;
use crate::external_apis::nominatin::get_point_from_address;
use crate::external_apis::overpass::make_overpass_graph;
use crate::external_apis::overpass::make_overpass_disk_graph;
use crate::graph_core::disk_graph::DiskGraph;


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

pub struct _Graph<S: IGraphStructure = AdjacencyList> {
    pub metadata: GraphMetadata,
    pub structure: S,
    pub positions_set: bool,
    pub build_time_ms: Option<f64>,
}





impl<S: IGraphStructure> _Graph<S> {
    fn label_to_id(&self, label: &str) -> Option<usize> {
        if self.structure.manages_labels() {
            self.structure.get_id_by_label(label)
        } else {
            self.metadata.label_id_map.get(label).copied()
        }
    }

    pub fn add_node(&mut self, label: String) {
        let exists = if self.structure.manages_labels() {
            self.structure.get_id_by_label(&label).is_some()
        } else {
            self.metadata.label_id_map.contains_key(&label)
        };

        if exists {
            println!("Node with label '{}' already exists!", label);
            return;
        }

        let index = self.structure.add_node();

        if self.structure.manages_labels() {
            self.structure.set_node_label(index, &label);
        } else {
            let new_node = _Node {
                label: label.clone(),
                index: Some(index),
                x: None,
                y: None,
            };
            self.metadata.label_id_map.insert(label.clone(), index);
            self.metadata.node_info.push(new_node);
        }
    }


    pub fn add_node_with_pos(&mut self, label: String, x: f64, y: f64) {
        let exists = if self.structure.manages_labels() {
            self.structure.get_id_by_label(&label).is_some()
        } else {
            self.metadata.label_id_map.contains_key(&label)
        };

        if exists {
            println!("Node with label '{}' already exists!", label);
            return;
        }

        let index = self.structure.add_node();
        self.positions_set = true;

        if self.structure.manages_labels() {
            self.structure.set_node_label(index, &label);
            self.structure.set_node_position(index, x, y);
        } else {
            let new_node = _Node {
                label: label.clone(),
                index: Some(index),
                x: Some(x),
                y: Some(y),
            };
            self.metadata.label_id_map.insert(label.clone(), index);
            self.metadata.node_info.push(new_node);
        }
    }

    pub fn create_connection(&mut self, from: String, to: String, weight: f32, directed: Option<bool>) {
        let from_idx = self.label_to_id(&from).expect("Node 'from' not found");
        let to_idx = self.label_to_id(&to).expect("Node 'to' not found");

        self.structure.create_connection(from_idx, to_idx, weight, directed);
    }

    pub fn node_by_label(&self, label: &str) -> Option<&_Node> {
        self.metadata.label_id_map.get(label).map(|&idx| &self.metadata.node_info[idx])
    }

    pub fn node_by_label_mut(&mut self, label: &str) -> Option<&mut _Node> {
        self.metadata.label_id_map.get(label).copied().map(move |idx| &mut self.metadata.node_info[idx])
    }

    pub fn resolve_label(&self, id: usize) -> String {
        if let Some(label) = self.structure.resolve_label(id) {
            return label;
        }

        if id < self.metadata.node_info.len() {
            return self.metadata.node_info[id].label.clone();
        }

        return format!("{}", id);
    }

    pub fn get_connections(
        &mut self,
        from_name: Option<&str>,
        to_name: Option<&str>,
        use_id: bool
    ) -> ConnectionsList {
        let mut all_connections = ConnectionsList::new();

        let from_str = from_name.unwrap_or("from");
        let to_str = to_name.unwrap_or("to");

        let edges = self.structure.get_all_edges();

        for (from_idx, to_idx, weight, directed) in edges {
            let mut formatted_conn = ConnectionData::new();

            let from_property = if use_id {
                ConnectionProperty::From(format!("{}", from_idx))
            } else {
                ConnectionProperty::From(self.resolve_label(from_idx))
            };

            let to_property = if use_id {
                ConnectionProperty::To(format!("{}", to_idx))
            } else {
                ConnectionProperty::To(self.resolve_label(to_idx))
            };

            formatted_conn.insert(from_str.to_string(), from_property);
            formatted_conn.insert(to_str.to_string(), to_property);
            formatted_conn.insert("weight".to_string(), ConnectionProperty::Weight(weight));
            formatted_conn.insert("directed".to_string(), ConnectionProperty::Directed(directed));

            all_connections.push(formatted_conn);
        }

        return all_connections;
    }

    pub fn generate_adjacency_matrix(&mut self) -> Vec<Vec<f32>> {
        let matrix_size = self.get_node_count();
        let mut adj_matrix: Vec<Vec<f32>> = vec![vec![0.; matrix_size]; matrix_size];
        let edges = self.structure.get_all_edges();

        for (from_idx, to_idx, weight, directed) in edges {
            adj_matrix[from_idx][to_idx] = weight;
            if !directed {
                adj_matrix[to_idx][from_idx] = weight;
            }
        }

        return adj_matrix;
    }

    pub fn get_total_weight(&mut self) -> f64 {
        let edges = self.structure.get_all_edges();
        edges.map(|(_, _, w, _)| w as f64).sum()
    }

    pub fn get_node_count(&self) -> usize {
        return self.structure.node_count();
    }

    pub fn get_edge_count(&mut self) -> usize {
        return self.structure.get_all_edges().count();
    }

    pub fn get_density(&mut self, directed: Option<bool>) -> f32 {
        let edge_count = self.get_edge_count() as f32;
        let node_count = self.get_node_count() as f32;
        let directed = directed.unwrap_or(false);
        let multiply = if directed {
            1.
        } else {
            2.
        };

        let density: f32 = (multiply * edge_count) / (node_count * (node_count - 1.));

        return density;
    }

    pub fn get_mean_weight(&mut self) -> f32 {
        return self.get_total_weight() as f32 / self.get_edge_count() as f32;
    }

    pub fn compute_degrees(&mut self, node_label: &str) -> HashMap<String, i32> {
        let mut degrees: HashMap<String, i32> = HashMap::new();
        degrees.insert("in_degree".to_string(), 0);
        degrees.insert("out_degree".to_string(), 0);
        degrees.insert("total_degree".to_string(), 0);
        degrees.insert("undirected_degree".to_string(), 0);

        let target_idx = match self.label_to_id(node_label) {
            Some(idx) => idx,
            None => return degrees,
        };

        let edges = self.structure.get_all_edges();

        for (from_idx, to_idx, _weight, directed) in edges {
            if directed {
                if from_idx == target_idx {
                    *degrees.entry("out_degree".to_string()).or_insert(0) += 1;
                    *degrees.entry("total_degree".to_string()).or_insert(0) += 1;
                }
                if to_idx == target_idx {
                    *degrees.entry("in_degree".to_string()).or_insert(0) += 1;
                    *degrees.entry("total_degree".to_string()).or_insert(0) += 1;
                }
                continue;
            }

            if to_idx == target_idx || from_idx == target_idx {
                *degrees.entry("undirected_degree".to_string()).or_insert(0) += 1;
            }
        }

        return degrees;
    }

    pub fn get_all_nodes_degrees(&mut self) -> HashMap<String, HashMap<String, i32>> {
        let mut degree_hash: HashMap<String, HashMap<String, i32>> = HashMap::new();

        let labels: Vec<String> = (0..self.get_node_count())
            .map(|i| self.resolve_label(i))
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

    pub fn get_node_strength(&mut self, node_label: &str) -> HashMap<&'static str, f32> {
        let mut strengths: HashMap<&str, f32> = HashMap::new();
        strengths.insert("out_strength", 0.);
        strengths.insert("in_strength", 0.);
        strengths.insert("total_strength", 0.);

        let target_idx = match self.label_to_id(node_label) {
            Some(idx) => idx,
            None => return strengths,
        };

        let edges = self.structure.get_all_edges();

        for (from_idx, to_idx, weight, directed) in edges {
            if from_idx == target_idx {
                *strengths.get_mut("out_strength").unwrap() += weight;
                if !directed {
                    *strengths.get_mut("in_strength").unwrap() += weight;
                }
            }

            if to_idx == target_idx {
                *strengths.get_mut("in_strength").unwrap() += weight;
                if !directed {
                    *strengths.get_mut("out_strength").unwrap() += weight;
                }
            }
        }

        *strengths.get_mut("total_strength").unwrap() = strengths["out_strength"] + strengths["in_strength"];

        return strengths;
    }

    pub fn get_centrality_degrees(&mut self, node_label: &str) -> HashMap<&'static str, f32> {
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

    pub fn get_degree_distribution(&mut self) -> HashMap<&'static str, HashMap<i32 ,f32>>{
        let mut computed_degrees: Vec<HashMap<String, i32>> = Vec::new();

        let labels: Vec<String> = (0..self.get_node_count()).map(|i| self.resolve_label(i)).collect();

        for label in &labels {
            computed_degrees.push(self.compute_degrees(label));
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

        let mut distribution: HashMap<&'static str, HashMap<i32 ,f32>> = HashMap::new();

        distribution.insert("undirected_distribution", undirected_distribution);
        distribution.insert("in_distribution", in_distribution);
        distribution.insert("out_distribution", out_distribution);

        return distribution;
    }

    pub fn compute_entropy(&mut self) -> HashMap<&'static str, f32> {
        let mut result: HashMap<&'static str, f32> = HashMap::new();

        let dist: HashMap<&'static str, HashMap<i32 ,f32>> = self.get_degree_distribution();
        let mut in_entropy: f32 = 0.;
        let mut out_entropy: f32 = 0.;
        let mut undirected_entropy: f32 = 0.;

        for (_degree, dist_value) in &dist["in_distribution"] {
            in_entropy += dist_value * dist_value.ln();
        }

        for (_degree, dist_value) in &dist["out_distribution"] {
            out_entropy += dist_value * dist_value.ln();
        }

        for (_degree, dist_value) in &dist["undirected_distribution"] {
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

    pub fn get_skewness(&mut self) -> HashMap<&'static str, f32> {

        fn _rank_degree_for_skewness(degree_collection: HashMap<String, i32>) -> Vec<(usize, i32)> {
            let mut sorted_degrees: Vec<(String, i32)> = degree_collection.into_iter().collect();
            sorted_degrees.sort_by(|a, b| b.1.cmp(&a.1));

            let mut ranked: Vec<(usize, i32)> = Vec::new();


            for i in 1..=sorted_degrees.len() {
                let (_node_label, degree) = sorted_degrees[i - 1].clone();
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
                    let degrees: Vec<i32> = ranked.iter().map(|(_rank, degree)| *degree).collect();
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

        let mut result: HashMap<&'static str, f32> = HashMap::new();

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

        let mut stack: Vec<usize> = Vec::new();
        let starting_idx = self.label_to_id(start_node_label).expect("Error: Initial node not found");

        stack.push(starting_idx);

        while let Some(idx) = stack.pop() {
            let label = self.resolve_label(idx);

            if !visited.insert(label.clone()) {
                continue;
            }

            final_order.push(label);

            let neighbors = self.structure.get_neighbors_ids(idx);
            for neighbor_idx in neighbors.into_iter().rev() {
                let neighbor_label = self.resolve_label(neighbor_idx);
                if !visited.contains(&neighbor_label) {
                    stack.push(neighbor_idx);
                }
            }
        }

        return final_order;
    }

    pub fn bfs(&mut self, start_node_label: &str) -> Vec<String> {
        let mut final_order: Vec<String> = Vec::new();
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();

        let starting_idx = self.label_to_id(start_node_label).expect("Error: Initial node not found");

        visited.insert(start_node_label.to_string());
        q.push_back(starting_idx);

        while let Some(idx) = q.pop_front() {
            let label = self.resolve_label(idx);
            final_order.push(label);

            let neighbors = self.structure.get_neighbors_ids(idx);
            for neighbor_idx in neighbors {
                let neighbor_label = self.resolve_label(neighbor_idx);
                if visited.insert(neighbor_label) {
                    q.push_back(neighbor_idx);
                }
            }
        }

        return final_order;
    }

    pub fn dijkstra(&mut self, start_node_label: &str) -> HashMap<String, f64>{
        let size = self.get_node_count();
        let _start_idx = self.label_to_id(start_node_label).expect("Node not found");
        let mut distances: HashMap<String, f64> = HashMap::new();

        for i in 0..size {
            let lbl = self.resolve_label(i);
            distances.insert(lbl, f64::INFINITY);
        }

        distances.insert(start_node_label.to_string(), 0.);

        let mut visited = vec![false; size];
        let adj_matrix = self.generate_adjacency_matrix();

        for _ in 0..size {
            let mut min_distance = f64::INFINITY;
            let mut u: Option<usize> = None;

            for i in 0..size {
                let lbl = self.resolve_label(i);
                if !visited[i] && distances[&lbl] < min_distance {
                    min_distance = distances[&lbl];
                    u = Some(i);
                }
            }

            if u.is_none() {
                break;
            }
            let u: usize = u.unwrap();
            visited[u] = true;

            for v in 0..size {
                if adj_matrix[u][v] != 0. && !visited[v] {
                    let u_lbl = self.resolve_label(u);
                    let v_lbl = self.resolve_label(v);

                    let alt = distances[&u_lbl] as f32 + adj_matrix[u][v];
                    if alt < distances[&v_lbl] as f32 {
                        distances.insert(v_lbl.clone(), alt as f64);
                    }
                }
            }
        }

        return distances;
    }

    pub fn get_nodes_for_render(&self) -> Vec<_Node> {
        let count = self.structure.node_count();
        if !self.structure.manages_labels() && self.metadata.node_info.len() == count {
            return self.metadata.node_info.clone();
        }

        return (0..count).map(|i| {
            let (x, y) = if self.structure.manages_positions() {
                self.structure.get_node_position(i)
                    .map(|(px, py)| (Some(px), Some(py)))
                    .unwrap_or((None, None))
            } else {
                (None, None)
            };
            _Node {
                label: self.resolve_label(i),
                index: Some(i),
                x,
                y,
            }
        }).collect();
    }

    pub fn output_svg(&mut self, layout: Layout, override_positions: bool, style: GraphStyle) -> String {
        let edges: Vec<(usize, usize)> = self.structure.get_all_edges()
            .map(|(u, v, _, _)| (u, v))
            .collect();

        let mut render_nodes = self.get_nodes_for_render();

        if !self.positions_set || override_positions {
            let layout_func = get_layout_function(layout);
            layout_func(&mut render_nodes, &edges);

            if self.structure.manages_positions() {
                for node in &render_nodes {
                    if let (Some(idx), Some(x), Some(y)) = (node.index, node.x, node.y) {
                        self.structure.set_node_position(idx, x, y);
                    }
                }
            } else if self.metadata.node_info.len() == render_nodes.len() {
                for (dst, src) in self.metadata.node_info.iter_mut().zip(render_nodes.iter()) {
                    dst.x = src.x;
                    dst.y = src.y;
                }
            }

            self.positions_set = true;
        } else if !self.structure.manages_positions() && self.metadata.node_info.len() == render_nodes.len() {
            for (dst, src) in render_nodes.iter_mut().zip(self.metadata.node_info.iter()) {
                dst.x = src.x;
                dst.y = src.y;
            }
        }

        let mut svg: Svg = Svg::new();
        let connections = self.get_connections(None, None, false);

        let svg_string = svg.get_svg(
            &render_nodes,
            &connections,
            style
        );
        return svg_string;
    }

    pub fn output_html(&mut self, file_name: &str, layout: Layout, override_positions: bool, style: GraphStyle) {
        let svg_string = self.output_svg(layout, override_positions, style);
        let html_writer = HtmlWriter{};
        html_writer.write_file(file_name, &svg_string).expect("Error while creating the file");
    }

    pub fn output_net_file(&mut self, path: &str) {
        let nodes = self.get_nodes_for_render();
        let edges: Vec<(usize, usize, f32, bool)> = self.structure.get_all_edges().collect();
        write_net_file(path, nodes, &edges).expect("Error while creating the file");
    }

    pub fn output_json_file(&mut self, path: &str) {
        let nodes = self.get_nodes_for_render();
        let edges: Vec<(usize, usize, f32, bool)> = self.structure.get_all_edges().collect();
        write_json_file(path, nodes, &edges).expect("Error while creating the file");
    }

    pub fn output_mtx_file(&mut self, path: &str) {
        let n_nodes = self.structure.node_count();
        let n_edges = self.get_edge_count();
        write_mtx_file(path, n_nodes, n_edges, self.structure.get_all_edges()).expect("Error while creating the file");
    }

    pub fn output_edge_list_file(&mut self, path: &str) {
        let nodes = self.get_nodes_for_render();
        write_edge_list_file(path, &nodes, self.structure.get_all_edges()).expect("Error while creating the file");
    }
}


impl<S: IGraphStructure + Default> _Graph<S> {
    pub fn default() -> Self {
        return _Graph {
            metadata: GraphMetadata::new(),
            structure: S::default(),
            positions_set: false,
            build_time_ms: None,
        };
    }

    pub fn from_net_file(path: &str) -> Self {
        let start = Instant::now();

        let mut new_graph = read_net_file(path).expect("Failed to read .net file");

        let duration = start.elapsed();

        new_graph.build_time_ms = Some(duration.as_secs_f64() * 1000.0);

        return new_graph;
    }

    pub fn from_json_file(path: &str) -> Self {
        let start = Instant::now();

        let mut new_graph = read_json_file(path).expect("Failed to read .json file");

        let duration = start.elapsed();

        new_graph.build_time_ms = Some(duration.as_secs_f64() * 1000.0);

        return new_graph;
    }

    pub fn from_mtx_file(path: &str) -> Self {
        let start = Instant::now();

        let mut new_graph = read_mtx_file(path).expect("Failed to read .mtx file");

        let duration = start.elapsed();

        new_graph.build_time_ms = Some(duration.as_secs_f64() * 1000.0);

        return new_graph;
    }

    pub fn from_edge_list_file(path: &str, directed: bool) -> Self {
        let start = Instant::now();

        let mut new_graph = read_edge_list_file(path, directed).expect("Failed to read edge list file");

        let duration = start.elapsed();

        new_graph.build_time_ms = Some(duration.as_secs_f64() * 1000.0);

        return new_graph;
    }

    pub fn from_adjacency_matrix(adj_matrix: Vec<Vec<f32>>, directed: Option<bool>, custom_labels: Option<Vec<String>>) -> Self {
        let start = Instant::now();
        let mut adj_matrix_graph = _Graph::default();

        let labels = custom_labels.unwrap_or_else(|| {
            (0..adj_matrix.len()).map(|x| x.to_string()).collect()
        });

        for label in labels {
            adj_matrix_graph.add_node(label);
        }

        for i in 0..adj_matrix.len() {
            for j in 0..adj_matrix.len() {
                if !directed.unwrap_or(false) && j < i {
                    continue;
                }

                let weight = adj_matrix[i][j];

                if weight != 0. {
                    let from_label = adj_matrix_graph.resolve_label(i);
                    let to_label = adj_matrix_graph.resolve_label(j);
                    adj_matrix_graph.create_connection(from_label, to_label, weight, directed);
                }
            }
        }

        let duration = start.elapsed();
        adj_matrix_graph.build_time_ms = Some(duration.as_secs_f64() * 1000.0);


        return adj_matrix_graph;
    }

    pub fn from_openalex(
        search: Option<&str>,
        author: Option<&str>,
        author_id: Option<&str>,
        author_orcid: Option<&str>,
        keyword: Option<&str>,
        graph_type: OpenAlexGraphType,
        api_key: &str,
        limit: Option<usize>,
        min_weight: Option<f32>,
        save_json_path: Option<&str>
    ) -> Self {

        let graph = dispatch_openalex_graph_creation(
            search,
            author,
            author_id,
            author_orcid,
            keyword,
            graph_type,
            api_key,
            limit,
            min_weight,
            save_json_path
        );

        return graph;
    }

    pub fn from_overpass_address(address: String, radius: f64) -> Self {
        let point = get_point_from_address(address).expect("Request to Nominatim failed!");
        let graph = make_overpass_graph(radius, point);

        return graph;
    }

    pub fn from_synthetic(graph_type: SyntheticGraphType) -> Self {
        match graph_type {
            SyntheticGraphType::ErdosRenyi { n, p } => {
                return erdos_renyi::generate_erdos_renyi(n, p);
            }
            SyntheticGraphType::BarabasiAlbert { n, m } => {
                return barabasi_albert::generate_barabasi_albert(n, m);
            }
            SyntheticGraphType::WattsStrogatz { n, k, beta } => {
                return watts_strogatz::generate_watts_strogatz(n, k, beta);
            }
        }
    }
}

impl _Graph<DiskGraph> {
    pub fn from_overpass_address_disk(address: String, radius: f64) -> Self {
        let point = get_point_from_address(address).expect("Request to Nominatim failed!");
        return make_overpass_disk_graph(radius, point);
    }
}
