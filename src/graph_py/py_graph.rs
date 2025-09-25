
use pyo3::prelude::*;
use std::cell::RefCell;
use std::rc::{Rc};
use crate::graph_core::graph::{_Graph};
use crate::graph_py::py_node::Node;
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

    fn create_connection(&self, from_label: String, to_label: String, weight: f32, directed: Option<bool>) {
        self.inner.borrow_mut().create_connection(from_label, to_label, weight, directed);
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

    pub fn get_all_nodes_degrees(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let degrees = PyDict::new(py);
        let all_node_degrees_hash = self.inner.borrow_mut().get_all_nodes_degrees();

        for (key, value) in all_node_degrees_hash.iter() {
            degrees.set_item(key, value)?;
        }

        return Ok(degrees.into());
    }

    pub fn get_average_degree(&self, directed: Option<bool>) -> PyResult<f32> {
        let average_degree = self.inner.borrow_mut().get_average_degree(directed);
        return Ok(average_degree);
    }
    
    #[getter]
    fn nodes(&self) -> Vec<Node> {
        self.inner.borrow().nodes.iter()
            .map(|rc_node| Node { inner: rc_node.clone() })
            .collect()
    }

    #[staticmethod]
    fn from_adjacency_matrix(adj_matrix: Vec<Vec<f32>>, directed: Option<bool>, custom_labels: Option<Vec<String>>) -> Graph {
        let graph = _Graph::from_adjacency_matrix(adj_matrix, directed, custom_labels);
        return Graph {
            inner: Rc::new(RefCell::new(graph)),
        }
    }
}
