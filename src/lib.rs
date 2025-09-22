pub mod graph_core;
pub mod file_writer_core;

pub mod graph_py;

pub use graph_core::node::_Node;
pub use file_writer_core::file_writer::{HtmlWriter, Writeable};

pub use graph_core::graph::_Graph;
pub use graph_core::graph::{ConnectionProperty};

pub use graph_py::py_graph::Graph;
pub use graph_py::py_node::Node;
use pyo3::prelude::*;
use pyo3_stub_gen::*;


#[pymodule]
fn netfog(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Node>()?;
    m.add_class::<Graph>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);