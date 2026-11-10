pub mod graph_core;
pub mod file_writer_core;
pub mod file_reader_core;
pub mod svg_creation;
pub mod layout;
#[cfg(feature = "python")]
pub mod graph_py;
pub mod external_apis;
pub mod synthetic_graphs;
pub mod visualization;

pub use graph_core::node::_Node;
pub use file_writer_core::file_writer::write_html;

pub use graph_core::graph::_Graph;

#[cfg(feature = "test_matrix")]
pub type TestGraph = graph_core::graph::_Graph<graph_core::adjacency_matrix::AdjacencyMatrix>;
#[cfg(feature = "test_csr")]
pub type TestGraph = graph_core::graph::_Graph<graph_core::compressed_sparse_row::CompressedSparseRow>;
#[cfg(feature = "test_pcsr")]
pub type TestGraph = graph_core::graph::_Graph<graph_core::packed_compressed_sparse_row::PackedCompressedSparseRow>;
#[cfg(not(any(feature = "test_matrix", feature = "test_csr", feature = "test_pcsr")))]
pub type TestGraph = graph_core::graph::_Graph<graph_core::adjacency_list::AdjacencyList>;

pub use graph_core::graph::{ConnectionProperty};

#[cfg(feature = "python")]
pub use graph_py::py_graph::GraphStructureType;
#[cfg(feature = "python")]
pub use synthetic_graphs::core::PySyntheticGraphType;
#[cfg(feature = "python")]
pub use graph_py::py_graph::Graph;
#[cfg(feature = "python")]
pub use graph_py::py_node::Node;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::*;

#[cfg(feature = "python")]
use crate::layout::layout::Layout;
#[cfg(feature = "python")]
use crate::layout::style::GraphStyle;

#[cfg(feature = "python")]
use crate::external_apis::core::OpenAlexGraphType;

#[cfg(feature = "python")]
#[pymodule]
fn netfog(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Node>()?;
    m.add_class::<GraphStructureType>()?;
    m.add_class::<Graph>()?;
    m.add_class::<Layout>()?;
    m.add_class::<GraphStyle>()?;
    m.add_class::<OpenAlexGraphType>()?;
    m.add_class::<PySyntheticGraphType>()?;
    Ok(())
}

#[cfg(feature = "python")]
define_stub_info_gatherer!(stub_info);
