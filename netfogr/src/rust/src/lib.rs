use extendr_api::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use netfog::graph_core::graph::_Graph;
use netfog::graph_core::graph_structure_interface::IGraphStructure;
use netfog::graph_core::adjacency_list::AdjacencyList;
use netfog::graph_core::adjacency_matrix::AdjacencyMatrix;
use netfog::graph_core::compressed_sparse_row::CompressedSparseRow;
use netfog::graph_core::packed_compressed_sparse_row::PackedCompressedSparseRow;
use netfog::layout::layout::Layout;
use netfog::layout::style::GraphStyle;

enum RGraphInner {
    Matrix(Rc<RefCell<_Graph<AdjacencyMatrix>>>),
    List(Rc<RefCell<_Graph<AdjacencyList>>>),
    CompressedSparseRow(Rc<RefCell<_Graph<CompressedSparseRow>>>),
    PackedCompressedSparseRow(Rc<RefCell<_Graph<PackedCompressedSparseRow>>>),
}

macro_rules! with_graph_mut {
    ($self:expr, |$g:ident| $body:expr) => {
        match &$self.inner {
            RGraphInner::Matrix(inner) => {
                let mut $g = (*inner).borrow_mut();
                $body
            },
            RGraphInner::List(inner) => {
                let mut $g = (*inner).borrow_mut();
                $body
            },
            RGraphInner::CompressedSparseRow(inner) => {
                let mut $g = (*inner).borrow_mut();
                $body
            },
            RGraphInner::PackedCompressedSparseRow(inner) => {
                let mut $g = (*inner).borrow_mut();
                $body
            },
        }
    }
}

macro_rules! with_graph {
    ($self:expr, |$g:ident| $body:expr) => {
        match &$self.inner {
            RGraphInner::Matrix(inner) => {
                let $g = (*inner).borrow();
                $body
            },
            RGraphInner::List(inner) => {
                let $g = (*inner).borrow();
                $body
            },
            RGraphInner::CompressedSparseRow(inner) => {
                let $g = (*inner).borrow();
                $body
            },
            RGraphInner::PackedCompressedSparseRow(inner) => {
                let $g = (*inner).borrow();
                $body
            },
        }
    }
}

fn make_inner(structure: &str) -> RGraphInner {
    match structure {
        "adjacency_matrix" => return RGraphInner::Matrix(Rc::new(RefCell::new(_Graph::<AdjacencyMatrix>::default()))),
        "compressed_sparse_row" => return RGraphInner::CompressedSparseRow(Rc::new(RefCell::new(_Graph::<CompressedSparseRow>::default()))),
        "packed_compressed_sparse_row" => return RGraphInner::PackedCompressedSparseRow(Rc::new(RefCell::new(_Graph::<PackedCompressedSparseRow>::default()))),
        "adjacency_list" => return RGraphInner::List(Rc::new(RefCell::new(_Graph::<AdjacencyList>::default()))),
        _ => return RGraphInner::List(Rc::new(RefCell::new(_Graph::<AdjacencyList>::default()))),
    }
}

enum NodeRefR {
    Id(usize),
    Label(String),
}

fn parse_node_ref(value: Robj) -> NodeRefR {
    if let Some(label) = value.as_str() {
        return NodeRefR::Label(label.to_string());
    }

    if let Some(id) = value.as_real() {
        return NodeRefR::Id(id as usize);
    }

    panic!("node reference must be a string label or a numeric id");
}

fn resolve_node_idx<S: IGraphStructure>(g: &_Graph<S>, node_ref: &NodeRefR) -> usize {
    match node_ref {
        NodeRefR::Id(id) => return *id,
        NodeRefR::Label(label) => return g.label_to_id(label).unwrap_or_else(|| panic!("Node '{}' not found", label)),
    }
}

fn parse_layout(layout: &str) -> Layout {
    match layout {
        "circular" => return Layout::Circular,
        "spring" => return Layout::Spring,
        "force_atlas2" => return Layout::ForceAtlas2,
        _ => return Layout::Random,
    }
}

fn degrees_to_list(degrees: std::collections::HashMap<String, i32>) -> List {
    return List::from_names_and_values(
        degrees.keys().cloned(),
        degrees.values().map(|&v| v as f64),
    ).unwrap();
}

fn f32_map_to_list(map: std::collections::HashMap<&'static str, f32>) -> List {
    return List::from_names_and_values(
        map.keys().map(|k| k.to_string()),
        map.values().map(|&v| v as f64),
    ).unwrap();
}

fn edges_to_list(edges: Vec<(usize, usize, f32, bool)>) -> List {
    let values: Vec<Robj> = edges.into_iter().map(|(from, to, weight, directed)| {
        return List::from_names_and_values(
            vec!["from", "to", "weight", "directed"],
            vec![
                Robj::from(from as f64),
                Robj::from(to as f64),
                Robj::from(weight as f64),
                Robj::from(directed),
            ],
        ).unwrap().into_robj();
    }).collect();

    return List::from_values(values);
}

#[extendr]
struct RGraph {
    inner: RGraphInner,
}

#[extendr]
impl RGraph {
    fn new(structure: &str) -> Self {
        return RGraph { inner: make_inner(structure) };
    }

    fn add_node(&self, label: &str) {
        with_graph_mut!(self, |g| {
            g.add_node(label.to_string());
        });

        return;
    }

    fn add_nodes(&self, count: i32) {
        with_graph_mut!(self, |g| {
            g.batch_add_anonymous_nodes(count as usize);
        });

        return;
    }

    fn create_connection(&self, from: Robj, to: Robj, weight: f64, directed: bool) {
        let from_ref = parse_node_ref(from);
        let to_ref = parse_node_ref(to);

        with_graph_mut!(self, |g| {
            let from_idx = resolve_node_idx(&g, &from_ref);
            let to_idx = resolve_node_idx(&g, &to_ref);

            g.create_connection_by_id(from_idx, to_idx, weight as f32, Some(directed));
        });

        return;
    }

    fn add_nodes_from(&self, labels: Vec<String>) {
        let borrowed: Vec<&str> = labels.iter().map(String::as_str).collect();
        with_graph_mut!(self, |g| {
            g.batch_add_nodes(borrowed);
        });

        return;
    }

    fn add_edges_from_ids(&self, from: Vec<f64>, to: Vec<f64>, weight: Vec<f64>, directed: Vec<Rbool>) {
        let mapped_edges: Vec<(usize, usize, f32, bool)> = (0..from.len())
            .map(|i| (from[i] as usize, to[i] as usize, weight[i] as f32, directed[i].is_true()))
            .collect();

        with_graph_mut!(self, |g| {
            g.batch_create_connections_from_ids(&mapped_edges);
        });

        return;
    }

    fn add_edges_from(&self, from: Vec<String>, to: Vec<String>, weight: Vec<f64>, directed: Vec<Rbool>) {
        let borrowed: Vec<(&str, &str, f32, Option<bool>)> = (0..from.len())
            .map(|i| (from[i].as_str(), to[i].as_str(), weight[i] as f32, Some(directed[i].is_true())))
            .collect();

        with_graph_mut!(self, |g| {
            g.batch_create_connections(borrowed);
        });

        return;
    }

    fn node_by_label(&self, node: Robj) -> List {
        let node_ref = parse_node_ref(node);

        let result = with_graph!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.node_by_id(idx)
        });

        let found = result.unwrap_or_else(|| panic!("Node not found"));

        return List::from_names_and_values(
            vec!["label", "id", "x", "y"],
            vec![
                Robj::from(found.label.to_string()),
                Robj::from(found.index.map(|v| v as f64)),
                Robj::from(found.x),
                Robj::from(found.y),
            ],
        ).unwrap();
    }

    fn get_all_edges(&self) -> List {
        let edges: Vec<(usize, usize, f32, bool)> = with_graph!(self, |g| g.structure.get_all_edges().collect());

        return edges_to_list(edges);
    }

    fn generate_adjacency_matrix(&self) -> RMatrix<f64> {
        let matrix = with_graph_mut!(self, |g| g.generate_adjacency_matrix());
        let n = matrix.len();

        return RMatrix::new_matrix(n, n, |r, c| matrix[r][c] as f64);
    }

    fn get_total_weight(&self) -> f64 {
        return with_graph_mut!(self, |g| g.get_total_weight());
    }

    fn get_mean_weight(&self) -> f64 {
        return with_graph_mut!(self, |g| g.get_mean_weight()) as f64;
    }

    fn get_node_count(&self) -> i32 {
        let count = with_graph!(self, |g| g.get_node_count());

        return count as i32;
    }

    fn get_edge_count(&self) -> i32 {
        let count = with_graph_mut!(self, |g| g.get_edge_count());

        return count as i32;
    }

    fn get_density(&self, directed: bool) -> f64 {
        return with_graph_mut!(self, |g| g.get_density(Some(directed))) as f64;
    }

    fn get_average_degree(&self, directed: bool) -> f64 {
        return with_graph_mut!(self, |g| g.get_average_degree(Some(directed))) as f64;
    }

    fn compute_degrees(&self, node: Robj) -> List {
        let node_ref = parse_node_ref(node);

        let degrees = with_graph_mut!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.compute_degrees_by_id(idx)
        });

        return degrees_to_list(degrees);
    }

    fn get_centrality_degrees(&self, node: Robj) -> List {
        let node_ref = parse_node_ref(node);

        let centralities = with_graph_mut!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.get_centrality_degrees_by_id(idx)
        });

        return f32_map_to_list(centralities);
    }

    fn get_node_strength(&self, node: Robj) -> List {
        let node_ref = parse_node_ref(node);

        let strengths = with_graph_mut!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.get_node_strength_by_id(idx)
        });

        return f32_map_to_list(strengths);
    }

    fn compute_entropy(&self) -> List {
        let entropy = with_graph_mut!(self, |g| g.compute_entropy());

        return f32_map_to_list(entropy);
    }

    fn get_max_possible_entropy(&self) -> f64 {
        return with_graph_mut!(self, |g| g.get_max_possible_entropy());
    }

    fn get_skewness(&self) -> List {
        let skewness = with_graph_mut!(self, |g| g.get_skewness());

        return f32_map_to_list(skewness);
    }

    fn dfs(&self, start: Robj) -> Vec<i32> {
        let node_ref = parse_node_ref(start);

        let order = with_graph_mut!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.dfs_by_id(idx)
        });

        return order.into_iter().map(|v| v as i32).collect();
    }

    fn bfs(&self, start: Robj) -> Vec<i32> {
        let node_ref = parse_node_ref(start);

        let order = with_graph_mut!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.bfs_by_id(idx)
        });

        return order.into_iter().map(|v| v as i32).collect();
    }

    fn dijkstra(&self, start: Robj) -> Vec<f64> {
        let node_ref = parse_node_ref(start);

        let distances = with_graph_mut!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.dijkstra_by_id(idx)
        });

        return distances;
    }

    fn connected_components(&self) -> Vec<i32> {
        let components = with_graph_mut!(self, |g| g.connected_components());

        return components.into_iter().map(|v| v as i32).collect();
    }

    fn pagerank(&self, damping: f64, max_iter: i32, tolerance: f64) -> Vec<f64> {
        let ranks = with_graph_mut!(self, |g| g.pagerank(damping as f32, max_iter as usize, tolerance as f32));

        return ranks.into_iter().map(|v| v as f64).collect();
    }

    fn triangle_count(&self) -> i32 {
        let total = with_graph_mut!(self, |g| g.triangle_count());

        return total as i32;
    }

    fn triangle_count_by_node(&self) -> Vec<i32> {
        let counts = with_graph_mut!(self, |g| g.triangle_count_by_node());

        return counts.into_iter().map(|v| v as i32).collect();
    }

    fn betweenness_centrality(&self) -> Vec<f64> {
        let scores = with_graph_mut!(self, |g| g.betweenness_centrality());

        return scores.into_iter().map(|v| v as f64).collect();
    }

    fn closeness_centrality(&self, node: Robj) -> f64 {
        let node_ref = parse_node_ref(node);

        let result = with_graph_mut!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.closeness_centrality_by_id(idx)
        });

        return result as f64;
    }

    fn closeness_centrality_all(&self) -> Vec<f64> {
        let scores = with_graph_mut!(self, |g| g.closeness_centrality_all());

        return scores.into_iter().map(|v| v as f64).collect();
    }

    fn clustering_coefficient(&self, node: Robj) -> f64 {
        let node_ref = parse_node_ref(node);

        let result = with_graph_mut!(self, |g| {
            let idx = resolve_node_idx(&g, &node_ref);
            g.clustering_coefficient_by_id(idx)
        });

        return result as f64;
    }

    fn transitivity(&self) -> f64 {
        return with_graph_mut!(self, |g| g.transitivity()) as f64;
    }

    fn k_core_decomposition(&self) -> List {
        let core = with_graph_mut!(self, |g| g.k_core_decomposition());

        let mut keys: Vec<usize> = core.keys().cloned().collect();
        keys.sort();

        let names: Vec<String> = keys.iter().map(|k| k.to_string()).collect();
        let values: Vec<f64> = keys.iter().map(|k| core[k] as f64).collect();

        return List::from_names_and_values(names, values).unwrap();
    }

    fn output_svg(&self, layout: &str, override_positions: bool) -> String {
        let parsed_layout = parse_layout(layout);
        let style = GraphStyle::default();

        return with_graph_mut!(self, |g| g.output_svg(parsed_layout, override_positions, style));
    }

    fn output_html(&self, file_name: &str, layout: &str, override_positions: bool) {
        let parsed_layout = parse_layout(layout);
        let style = GraphStyle::default();

        with_graph_mut!(self, |g| g.output_html(file_name, parsed_layout, override_positions, style));

        return;
    }

    fn output_net_file(&self, path: &str) {
        with_graph_mut!(self, |g| g.output_net_file(path));

        return;
    }

    fn output_json_file(&self, path: &str) {
        with_graph_mut!(self, |g| g.output_json_file(path));

        return;
    }

    fn output_mtx_file(&self, path: &str) {
        with_graph_mut!(self, |g| g.output_mtx_file(path));

        return;
    }

    fn output_edge_list_file(&self, path: &str) {
        with_graph_mut!(self, |g| g.output_edge_list_file(path));

        return;
    }
}

extendr_module! {
    mod netfogr;
    impl RGraph;
}
