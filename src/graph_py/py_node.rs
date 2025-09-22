use pyo3::prelude::*;
use std::cell::RefCell;
use std::rc::{Rc};
use crate::graph_core::node::_Node;
use pyo3_stub_gen::derive::gen_stub_pyclass;

#[gen_stub_pyclass]
#[pyclass(unsendable, module="netfog")]
#[derive(Clone)]
pub struct Node {
    pub inner: Rc<RefCell<_Node>>,
}

#[pymethods]
impl Node {
    #[new]
    fn new(label: String) -> Self {
        Node {
            inner: Rc::new(RefCell::new(_Node {
                label,
                connections: Vec::new(),
            })),
        }
    }

    fn add_connection(&self, node: &Node, weight: i32, directed: Option<bool>) {
        self.inner.borrow_mut().add_connection(node.inner.clone(), weight, directed);
    }

    #[getter]
    fn label(&self) -> String {
        self.inner.borrow().label.clone()
    }

}
