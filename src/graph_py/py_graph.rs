use pyo3::prelude::*;
use std::cell::RefCell;
use std::rc::{Rc};
use crate::graph_core::graph::{_Graph,ConnectionProperty};
use crate::graph_core::graph_structure_interface::IGraphStructure;
use crate::graph_core::adjacency_matrix::AdjacencyMatrix;
use crate::graph_core::adjacency_list::AdjacencyList;
use crate::graph_core::compressed_sparse_row::CompressedSparseRow;
use crate::graph_core::packed_compressed_sparse_row::PackedCompressedSparseRow;
use crate::graph_py::py_node::Node;
use crate::layout::layout::Layout;
use crate::layout::style::GraphStyle;
use crate::external_apis::core::OpenAlexGraphType;

use crate::synthetic_graphs::core::{SyntheticGraphType, PySyntheticGraphType};
use pyo3::types::PyDict;
use pyo3_stub_gen::derive::gen_stub_pyclass;

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Default)]
pub enum GraphStructureType {
    AdjacencyMatrix,
    #[default]
    AdjacencyList,
    CompressedSparseRow,
    PackedCompressedSparseRow,
}

enum GraphInner {
    Matrix(Rc<RefCell<_Graph<AdjacencyMatrix>>>),
    List(Rc<RefCell<_Graph<AdjacencyList>>>),
    CompressedSparseRow(Rc<RefCell<_Graph<CompressedSparseRow>>>),
    PackedCompressedSparseRow(Rc<RefCell<_Graph<PackedCompressedSparseRow>>>),
}

macro_rules! with_graph_mut {
    ($self:expr, |$g:ident| $body:expr) => {
        match &$self.inner {
            GraphInner::Matrix(inner) => {
                let mut $g = (*inner).borrow_mut();
                $body
            },
            GraphInner::List(inner) => {
                let mut $g = (*inner).borrow_mut();
                $body
            },
            GraphInner::CompressedSparseRow(inner) => {
                let mut $g = (*inner).borrow_mut();
                $body
            },
            GraphInner::PackedCompressedSparseRow(inner) => {
                let mut $g = (*inner).borrow_mut();
                $body
            },
        }
    }
}

macro_rules! with_graph {
    ($self:expr, |$g:ident| $body:expr) => {
        match &$self.inner {
            GraphInner::Matrix(inner) => {
                let $g = (*inner).borrow();
                $body
            },
            GraphInner::List(inner) => {
                let $g = (*inner).borrow();
                $body
            },
            GraphInner::CompressedSparseRow(inner) => {
                let $g = (*inner).borrow();
                $body
            },
            GraphInner::PackedCompressedSparseRow(inner) => {
                let $g = (*inner).borrow();
                $body
            },

        }
    }
}

macro_rules! make_graph {
    ($structure:expr, |$T:ident| $build:expr) => {{
        let type_val = $structure.unwrap_or(GraphStructureType::AdjacencyList);
        match type_val {
            GraphStructureType::AdjacencyMatrix => {
                type $T = AdjacencyMatrix;
                let graph = $build;
                Graph { inner: GraphInner::Matrix(Rc::new(RefCell::new(graph))) }
            },
            GraphStructureType::AdjacencyList => {
                type $T = AdjacencyList;
                let graph = $build;
                Graph { inner: GraphInner::List(Rc::new(RefCell::new(graph))) }
            },
            GraphStructureType::CompressedSparseRow => {
                type $T = CompressedSparseRow;
                let graph = $build;
                Graph { inner: GraphInner::CompressedSparseRow(Rc::new(RefCell::new(graph))) }
            },
            GraphStructureType::PackedCompressedSparseRow => {
                type $T = PackedCompressedSparseRow;
                let graph = $build;
                Graph { inner: GraphInner::PackedCompressedSparseRow(Rc::new(RefCell::new(graph))) }
            },
        }
    }}
}

#[gen_stub_pyclass]
#[pyclass(unsendable, module="netfog")]
pub struct Graph {
    inner: GraphInner,
}

#[pyclass(unsendable, module="netfog")]
pub struct PyEdgeIterator {
    inner: Box<dyn Iterator<Item = (usize, usize, f32, bool)>>,
}

#[pymethods]
impl PyEdgeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        return slf;
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(usize, usize, f32, bool)> {
        return slf.inner.next();
    }
}

#[pymethods]
impl Graph {
    #[new]
    #[pyo3(signature = (structure=None))]
    fn new(structure: Option<crate::graph_py::py_graph::GraphStructureType>) -> Self {
        let type_val = structure.unwrap_or(crate::graph_py::py_graph::GraphStructureType::AdjacencyList);
        let inner = match type_val {
            crate::graph_py::py_graph::GraphStructureType::AdjacencyMatrix => GraphInner::Matrix(Rc::new(RefCell::new(_Graph::<AdjacencyMatrix>::default()))),
            crate::graph_py::py_graph::GraphStructureType::AdjacencyList => GraphInner::List(Rc::new(RefCell::new(_Graph::<AdjacencyList>::default()))),
            crate::graph_py::py_graph::GraphStructureType::CompressedSparseRow => GraphInner::CompressedSparseRow(Rc::new(RefCell::new(_Graph::<CompressedSparseRow>::default()))),
            crate::graph_py::py_graph::GraphStructureType::PackedCompressedSparseRow => GraphInner::PackedCompressedSparseRow(Rc::new(RefCell::new(_Graph::<PackedCompressedSparseRow>::default()))),
        };
        return Graph { inner };
    }

    fn add_node(&self, py: Python<'_>, label: String) -> PyResult<Py<Node>> {
        with_graph_mut!(self, |g| {
            g.add_node(label.clone());
        });
        let node_val = with_graph!(self, |g| {
            if g.structure.manages_labels() {
                let idx = g.structure.get_id_by_label(&label)
                    .unwrap_or_else(|| g.get_node_count().saturating_sub(1));
                let (x, y) = g.structure.get_node_position(idx)
                    .map(|(px, py)| (Some(px), Some(py)))
                    .unwrap_or((None, None));
                crate::graph_core::node::_Node {
                    label: label.clone(),
                    index: Some(idx),
                    x,
                    y,
                }
            } else {
                g.metadata.node_info.last().unwrap().clone()
            }
        });
        let node = Node { inner: node_val };
        return Py::new(py, node);
    }

    #[pyo3(signature = (from_label, to_label, weight=0., directed=false))]
    fn create_connection(&self, from_label: String, to_label: String, weight: f32, directed: Option<bool>) {
        with_graph_mut!(self, |g| {
            g.create_connection(from_label, to_label, weight, directed);
        });
    }

    #[pyo3(signature = (labels))]
    fn add_nodes_from(&mut self, labels: Vec<String>) {
        with_graph_mut!(self, |g| {
            g.batch_add_nodes(labels);
        });
    }

    #[pyo3(signature = (connections))]
    fn add_edges_from(&mut self, connections: Vec<(String, String, f32, Option<bool>)>) {
        with_graph_mut!(self, |g| {
            g.batch_create_connections(connections);
        });
    }

    pub fn get_all_edges(&self) -> PyEdgeIterator {
        let edges: Vec<(usize, usize, f32, bool)> =
            with_graph!(self, |g| g.structure.get_all_edges().collect());
        return PyEdgeIterator { inner: Box::new(edges.into_iter()) };
    }

    fn node_by_label(&self, node_label: &str, py: Python<'_>) ->  PyResult<Py<Node>> {
        let node_val_opt = with_graph!(self, |g| {
            if g.structure.manages_labels() {
                g.structure.get_id_by_label(node_label).map(|idx| {
                    let (x, y) = g.structure.get_node_position(idx)
                        .map(|(px, py)| (Some(px), Some(py)))
                        .unwrap_or((None, None));
                    crate::graph_core::node::_Node {
                        label: node_label.to_string(),
                        index: Some(idx),
                        x,
                        y,
                    }
                })
            } else {
                g.metadata.label_id_map.get(node_label).map(|&idx| g.metadata.node_info[idx].clone())
            }
        });

        if let Some(node_val) = node_val_opt {
            let node = Node { inner: node_val };
            return Py::new(py, node);
        }
        return Err(pyo3::exceptions::PyValueError::new_err("Node not found"));
    }

    #[pyo3(signature = (from_name="from", to_name="to", use_id=false))]
    fn get_connections(&self, from_name: Option<&str>, to_name:Option<&str>, use_id: bool, py: Python<'_>) ->  PyResult<Vec<Py<PyDict>>> {
         let connections_snapshot = with_graph_mut!(self, |g| g.get_connections(from_name, to_name, use_id));

        let mut py_connections: Vec<Py<PyDict>> = Vec::new();

        for rust_map in connections_snapshot {
            let py_dict = PyDict::new(py);

            for (key, value) in rust_map {
                match value {
                    ConnectionProperty::From(s) => py_dict.set_item(key, s)?,
                    ConnectionProperty::To(s) => py_dict.set_item(key, s)?,
                    ConnectionProperty::Weight(w) => py_dict.set_item(key, w)?,
                    ConnectionProperty::Directed(d) => py_dict.set_item(key, d)?,
                }
            }

            py_connections.push(py_dict.into());
        }

        Ok(py_connections)
    }

    fn generate_adjacency_matrix(&self) -> PyResult<Vec<Vec<f32>>> {
        return Ok(with_graph_mut!(self, |g| g.generate_adjacency_matrix()));
    }

    fn get_total_weight(&self) -> PyResult<f64> {
        return Ok(with_graph_mut!(self, |g| g.get_total_weight()));
    }

    fn get_mean_weight(&self) -> PyResult<f32> {
        return Ok(with_graph_mut!(self, |g| g.get_mean_weight()));
    }

    pub fn get_node_count(&self) -> PyResult<usize> {
        return Ok(with_graph!(self, |g| g.get_node_count()));
    }

    pub fn get_edge_count(&self) -> PyResult<usize> {
        return Ok(with_graph_mut!(self, |g| g.get_edge_count()));
    }

    pub fn get_density(&self, directed: Option<bool>) -> PyResult<f32> {
        return Ok(with_graph_mut!(self, |g| g.get_density(directed)));
    }

    pub fn compute_degrees(&self, node_label: &str, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let degrees_snapshot = with_graph_mut!(self, |g| g.compute_degrees(node_label));
        let degrees = PyDict::new(py);

        for (key, value) in degrees_snapshot.iter() {
            degrees.set_item(key, value)?;
        }

        return Ok(degrees.into());
    }

    pub fn get_centrality_degrees(&self, node_label: &str, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let centrality_snapshot = with_graph_mut!(self, |g| g.get_centrality_degrees(node_label));
        let centralities = PyDict::new(py);

        for (key, value) in centrality_snapshot.iter() {
            centralities.set_item(key, value)?;
        }

        return Ok(centralities.into());
    }

    pub fn get_node_strength(&self, node_label: &str, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let strength_snapshot = with_graph_mut!(self, |g| g.get_node_strength(node_label));
        let strength = PyDict::new(py);

        for (key, value) in strength_snapshot.iter() {
            strength.set_item(key, value)?;
        }

        return Ok(strength.into());
    }

    pub fn get_all_nodes_degrees(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let degrees = PyDict::new(py);
        let all_node_degrees_hash = with_graph_mut!(self, |g| g.get_all_nodes_degrees());

        for (key, value) in all_node_degrees_hash.iter() {
            degrees.set_item(key, value)?;
        }

        return Ok(degrees.into());
    }

    #[pyo3(signature = (directed=false))]
    pub fn get_average_degree(&self, directed: Option<bool>) -> PyResult<f32> {
        let average_degree = with_graph_mut!(self, |g| g.get_average_degree(directed));
        return Ok(average_degree);
    }

    pub fn get_degree_distribution(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let distribution_snapshot = with_graph_mut!(self, |g| g.get_degree_distribution());
        let distribution = PyDict::new(py);

        for (key, value) in distribution_snapshot.iter() {
            distribution.set_item(key, value)?;
        }

        return Ok(distribution.into());
    }

    pub fn compute_entropy(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let entropy_snapshot = with_graph_mut!(self, |g| g.compute_entropy());
        let entropy = PyDict::new(py);

        for (key, value) in entropy_snapshot.iter() {
            entropy.set_item(key, value)?;
        }

        return Ok(entropy.into());
    }

    pub fn get_max_possible_entropy(&mut self) -> PyResult<f64> {
        let result = with_graph_mut!(self, |g| g.get_max_possible_entropy());
        return Ok(result);
    }

    pub fn get_skewness(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let skewness_snapshot = with_graph_mut!(self, |g| g.get_skewness());
        let skewness = PyDict::new(py);

        for (key, value) in skewness_snapshot.iter() {
            skewness.set_item(key, value)?;
        }

        return Ok(skewness.into());
    }

    pub fn dfs(&mut self, start_node_label: &str) -> Vec<String> {
        let elements = with_graph_mut!(self, |g| g.dfs(start_node_label));
        return elements;
    }

    pub fn bfs(&mut self, start_node_label: &str) -> Vec<String> {
        let elements = with_graph_mut!(self, |g| g.bfs(start_node_label));
        return elements;
    }

    pub fn dijkstra(&self, start_node_label: &str, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dijkstra_snapshot = with_graph_mut!(self, |g| g.dijkstra(start_node_label));
        let dijkstra = PyDict::new(py);

        for (key, value) in dijkstra_snapshot.iter() {
            dijkstra.set_item(key, value)?;
        }

        return Ok(dijkstra.into());
    }

    #[pyo3(signature = (layout=Layout::Random, override_positions=false, style=None))]
    pub fn output_svg(&mut self, layout: Layout, override_positions: bool, style: Option<GraphStyle>) -> String {
        let graph_style = match style {
            Some(s) => s,
            None => GraphStyle::default()
        };

        let svg_str = with_graph_mut!(self, |g| g.output_svg(layout, override_positions, graph_style));

        return svg_str;
    }

     #[pyo3(signature = (file_name, layout=Layout::Random, override_positions=false, style=None))]
     pub fn output_html(&mut self, file_name: &str, layout: Layout, override_positions: bool, style: Option<GraphStyle>) -> PyResult<()> {
         let graph_style = match style {
             Some(s) => s,
             None => GraphStyle::default()
         };

         with_graph_mut!(self, |g| g.output_html(file_name, layout, override_positions, graph_style));

         return Ok(());
     }

     pub fn output_net_file(&mut self, file_name: &str) -> PyResult<()> {
         with_graph_mut!(self, |g| g.output_net_file(file_name));
         return Ok(());
     }

     pub fn output_json_file(&mut self, file_name: &str) -> PyResult<()> {
         with_graph_mut!(self, |g| g.output_json_file(file_name));
         return Ok(());
     }

     pub fn output_mtx_file(&mut self, file_name: &str) -> PyResult<()> {
         with_graph_mut!(self, |g| g.output_mtx_file(file_name));
         return Ok(());
     }

     pub fn output_edge_list_file(&mut self, file_name: &str) -> PyResult<()> {
         with_graph_mut!(self, |g| g.output_edge_list_file(file_name));
         return Ok(());
     }

    #[getter]
    fn nodes(&self) -> Vec<Node> {
        with_graph!(self, |g| {
            g.get_nodes_for_render()
                .into_iter()
                .map(|node_val| Node { inner: node_val })
                .collect()
        })
    }

    #[getter]
    fn build_time_ms(&self) -> Option<f64> {
        with_graph!(self, |g| g.build_time_ms)
    }

    #[staticmethod]
    #[pyo3(signature = (adj_matrix, directed=false, custom_labels=None, structure=None))]
    fn from_adjacency_matrix(adj_matrix: Vec<Vec<f32>>, directed: Option<bool>, custom_labels: Option<Vec<String>>, structure: Option<crate::graph_py::py_graph::GraphStructureType>) -> Graph {
        return make_graph!(structure, |S| _Graph::<S>::from_adjacency_matrix(adj_matrix, directed, custom_labels));
    }

    #[staticmethod]
    #[pyo3(signature = (file_path, structure=None))]
    fn from_net_file(file_path: &str, structure: Option<crate::graph_py::py_graph::GraphStructureType>) -> Graph {
        return make_graph!(structure, |S| _Graph::<S>::from_net_file(file_path));
    }

    #[staticmethod]
    #[pyo3(signature = (file_path, structure=None))]
    fn from_json_file(file_path: &str, structure: Option<crate::graph_py::py_graph::GraphStructureType>) -> Graph {
        return make_graph!(structure, |S| _Graph::<S>::from_json_file(file_path));
    }

    #[staticmethod]
    #[pyo3(signature = (file_path, structure=None))]
    fn from_mtx_file(file_path: &str, structure: Option<crate::graph_py::py_graph::GraphStructureType>) -> Graph {
        return make_graph!(structure, |S| _Graph::<S>::from_mtx_file(file_path));
    }

    #[staticmethod]
    #[pyo3(signature = (file_path, directed=false, structure=None))]
    fn from_edge_list_file(file_path: &str, directed: bool, structure: Option<crate::graph_py::py_graph::GraphStructureType>) -> Graph {
        return make_graph!(structure, |S| _Graph::<S>::from_edge_list_file(file_path, directed));
    }

    #[staticmethod]
    #[pyo3(signature = (api_key, graph_type, search=None, author=None, author_id=None, author_orcid=None, keyword=None, limit=None, min_weight=None, save_json_path=None, structure=None))]
    fn from_openalex(
        api_key: &str,
        graph_type: OpenAlexGraphType,
        search: Option<&str>,
        author: Option<&str>,
        author_id: Option<&str>,
        author_orcid: Option<&str>,
        keyword: Option<&str>,
        limit: Option<usize>,
        min_weight: Option<f32>,
        save_json_path: Option<&str>,
        structure: Option<crate::graph_py::py_graph::GraphStructureType>
    ) -> Graph {
        return make_graph!(structure, |S| _Graph::<S>::from_openalex(
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
        ));
    }

    #[staticmethod]
    #[pyo3(signature = (address, radius, structure=None))]
    fn from_overpass_address(
        address: String,
        radius: f64,
        structure: Option<crate::graph_py::py_graph::GraphStructureType>
    ) -> Graph {
        return make_graph!(structure, |S| _Graph::<S>::from_overpass_address(address.clone(), radius));
    }

    #[staticmethod]
    #[pyo3(signature = (graph_type, structure=None))]
    fn from_synthetic(
        graph_type: PySyntheticGraphType,
        structure: Option<crate::graph_py::py_graph::GraphStructureType>
    ) -> Graph {
        return make_graph!(structure, |S| _Graph::<S>::from_synthetic(graph_type.inner.clone()));
    }
}
