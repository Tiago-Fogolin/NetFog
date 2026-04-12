use pyo3::prelude::*;
use crate::graph_core::node::_Node;
use pyo3_stub_gen::derive::gen_stub_pyclass;

#[gen_stub_pyclass]
#[pyclass(unsendable, module="netfog")]
#[derive(Clone)]
pub struct Node {
    pub inner: _Node,
}

#[pymethods]
impl Node {
    #[new]
    fn new(label: String) -> Self {
        Node {
            inner: _Node {
                label,
                x: None,
                y: None,
                index: None
            },
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("Node(\"{}\")", self.inner.label));
    }

    #[getter]
    fn label(&self) -> String {
        self.inner.label.clone()
    }

    #[getter]
    fn id(&self) -> Option<usize> {
        self.inner.index
    }
}
