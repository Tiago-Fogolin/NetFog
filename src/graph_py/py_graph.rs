
use pyo3::prelude::*;
use std::cell::RefCell;
use std::rc::{Rc};
use crate::graph_core::graph::{_Graph};
use crate::graph_py::py_node::Node;
use crate::layout::layout::Layout;
use crate::layout::style::GraphStyle;
use pyo3::types::PyDict;
use pyo3_stub_gen::derive::gen_stub_pyclass;



#[gen_stub_pyclass]
#[pyclass(unsendable, module="netfog")]
pub struct Graph {
    inner: Rc<RefCell<_Graph>>,
}

#[pymethods]
impl Graph {
    #[new]
    fn new() -> Self {
        Graph {
            inner: Rc::new(RefCell::new(_Graph::default())),
        }
    }

    fn add_node(&self, py: Python<'_>, label: String) -> PyResult<Py<Node>> {
        self.inner.borrow_mut().add_node(label.clone());
        let node_rc = self.inner.borrow().nodes.last().unwrap().clone();
        let node = Node { inner: node_rc };
        return Py::new(py, node);
    }

    #[pyo3(signature = (from_label, to_label, weight=0., directed=false))]
    fn create_connection(&self, from_label: String, to_label: String, weight: f32, directed: Option<bool>) {
        self.inner.borrow_mut().create_connection(from_label, to_label, weight, directed);
    }

    fn node_by_label(&self, node_label: &str, py: Python<'_>) ->  PyResult<Py<Node>> {
        let node_rc = self.inner.borrow().nodes.last().unwrap().clone();
        let node = Node { inner: node_rc };
        return Py::new(py, node);
    }

    fn get_connections(&self, py: Python<'_>) ->  PyResult<Vec<Py<PyDict>>> {
        let nodes_snapshot: Vec<_> = self.inner.borrow().nodes.iter().cloned().collect();
        let mut all_connections = Vec::new();

        for rc_node in nodes_snapshot {
            let node = rc_node.borrow();
            for conn in node.connections.iter() {
                let dict = PyDict::new(py);

                dict.set_item("from", node.label.clone())?;
                let to_label = match conn.node.upgrade() {
                    Some(to_rc) => to_rc.borrow().label.clone(),
                    None => "[Removed]".to_string(),
                };
                dict.set_item("to", to_label)?;
                dict.set_item("weight", conn.weight)?;
                dict.set_item("directed", conn.directed)?;

                all_connections.push(dict.into());
            }
        }

        return Ok(all_connections);
    }

    fn generate_adjacency_matrix(&self) -> PyResult<Vec<Vec<f32>>> {
        return Ok(self.inner.borrow_mut().generate_adjacency_matrix());
    }

    fn get_total_weight(&self) -> PyResult<f32> {
        return Ok(self.inner.borrow_mut().get_total_weight());
    }

    fn get_mean_weight(&self) -> PyResult<f32> {
        return Ok(self.inner.borrow_mut().get_mean_weight());
    }

    pub fn get_node_count(&self) -> PyResult<usize> {
        return Ok(self.inner.borrow().get_node_count());
    }

    pub fn get_edge_count(&self) -> PyResult<usize> {
        return Ok(self.inner.borrow_mut().get_edge_count());
    }

    pub fn get_density(&self, directed: Option<bool>) -> PyResult<f32> {
        return Ok(self.inner.borrow_mut().get_density(directed));
    }

    pub fn compute_degrees(&self, node_label: &str, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let degrees_snapshot = self.inner.borrow_mut().compute_degrees(node_label);
        let degrees = PyDict::new(py);

        for (key, value) in degrees_snapshot.iter() {
            degrees.set_item(key, value)?;
        }

        return Ok(degrees.into());
    }

    pub fn get_centrality_degrees(&self, node_label: &str, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let mut inner = self.inner.borrow_mut();
        let centrality_snapshot = inner.get_centrality_degrees(node_label);
        let centralities = PyDict::new(py);

        for (key, value) in centrality_snapshot.iter() {
            centralities.set_item(key, value)?;
        }

        return Ok(centralities.into());
    }

    pub fn get_node_strength(&self, node_label: &str, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let mut inner = self.inner.borrow_mut();
        let strength_snapshot = inner.get_node_strength(node_label);
        let strength = PyDict::new(py);

        for (key, value) in strength_snapshot.iter() {
            strength.set_item(key, value)?;
        }

        return Ok(strength.into());
    }

    pub fn get_all_nodes_degrees(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let degrees = PyDict::new(py);
        let all_node_degrees_hash = self.inner.borrow_mut().get_all_nodes_degrees();

        for (key, value) in all_node_degrees_hash.iter() {
            degrees.set_item(key, value)?;
        }

        return Ok(degrees.into());
    }

    #[pyo3(signature = (directed=false))]
    pub fn get_average_degree(&self, directed: Option<bool>) -> PyResult<f32> {
        let average_degree = self.inner.borrow_mut().get_average_degree(directed);
        return Ok(average_degree);
    }

    pub fn get_degree_distribution(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let mut inner = self.inner.borrow_mut();
        let distribution_snapshot = inner.get_degree_distribution();
        let distribution = PyDict::new(py);

        for (key, value) in distribution_snapshot.iter() {
            distribution.set_item(key, value)?;
        }

        return Ok(distribution.into());
    }

    pub fn compute_entropy(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let mut inner = self.inner.borrow_mut();
        let entropy_snapshot = inner.compute_entropy();
        let entropy = PyDict::new(py);

        for (key, value) in entropy_snapshot.iter() {
            entropy.set_item(key, value)?;
        }

        return Ok(entropy.into());
    }

    pub fn get_max_possible_entropy(&mut self) -> PyResult<f64> {
        let result = self.inner.borrow_mut().get_max_possible_entropy();
        return Ok(result);
    }

    pub fn get_skewness(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let mut inner = self.inner.borrow_mut();
        let skewness_snapshot = inner.get_skewness();
        let skewness = PyDict::new(py);

        for (key, value) in skewness_snapshot.iter() {
            skewness.set_item(key, value)?;
        }

        return Ok(skewness.into());
    }

    pub fn dfs(&mut self, start_node_label: &str) -> Vec<String> {
        let elements = self.inner.borrow_mut().dfs(start_node_label);
        return elements;
    }

    pub fn bfs(&mut self, start_node_label: &str) -> Vec<String> {
        let elements = self.inner.borrow_mut().bfs(start_node_label);
        return elements;
    }

    pub fn dijkstra(&self, start_node_label: &str, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let mut inner = self.inner.borrow_mut();
        let dijkstra_snapshot = inner.dijkstra(start_node_label);
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

        let svg_str = self.inner.borrow_mut().output_svg(layout, override_positions, graph_style);

        return svg_str;
    }

     #[pyo3(signature = (file_name, layout=Layout::Random, override_positions=false, style=None))]
     pub fn output_html(&mut self, file_name: &str, layout: Layout, override_positions: bool, style: Option<GraphStyle>) -> PyResult<()> {
         let graph_style = match style {
             Some(s) => s,
             None => GraphStyle::default()
         };

         let mut inner = self.inner.borrow_mut();

         inner.output_html(file_name, layout, override_positions, graph_style);

         return Ok(());
     }

     pub fn output_net_file(&mut self, file_name: &str) -> PyResult<()> {
         let mut inner = self.inner.borrow_mut();

         inner.output_net_file(file_name);

         return Ok(());
     }

     pub fn output_json_file(&mut self, file_name: &str) -> PyResult<()> {
         let mut inner = self.inner.borrow_mut();

         inner.output_json_file(file_name);

         return Ok(());
     }

    #[getter]
    fn nodes(&self) -> Vec<Node> {
        self.inner.borrow().nodes.iter()
            .map(|rc_node| Node { inner: rc_node.clone() })
            .collect()
    }

    #[staticmethod]
    #[pyo3(signature = (adj_matrix, directed=false, custom_labels=None))]
    fn from_adjacency_matrix(adj_matrix: Vec<Vec<f32>>, directed: Option<bool>, custom_labels: Option<Vec<String>>) -> Graph {
        let graph = _Graph::from_adjacency_matrix(adj_matrix, directed, custom_labels);
        return Graph {
            inner: Rc::new(RefCell::new(graph)),
        }
    }

    #[staticmethod]
    fn from_net_file(file_path: &str) -> Graph {
        let graph = _Graph::from_net_file(file_path);
        return Graph {
            inner: Rc::new(RefCell::new(graph)),
        }
    }

    #[staticmethod]
    fn from_json_file(file_path: &str) -> Graph {
        let graph = _Graph::from_json_file(file_path);
        return Graph {
            inner: Rc::new(RefCell::new(graph)),
        }
    }
}
