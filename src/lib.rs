pub mod graph_core;
pub mod file_writer_core;
pub mod file_reader_core;
pub mod svg_creation;
pub mod layout;
pub mod graph_py;
pub mod external_apis;

pub use graph_core::node::_Node;
pub use file_writer_core::file_writer::{HtmlWriter, Writeable};

pub use graph_core::graph::_Graph;

#[cfg(feature = "test_matrix")]
pub type TestGraph = graph_core::graph::_Graph<graph_core::adjacency_matrix::AdjacencyMatrix>;
#[cfg(feature = "test_csr")]
pub type TestGraph = graph_core::graph::_Graph<graph_core::compressed_sparse_row::CompressedSparseRow>;
#[cfg(feature = "test_pcsr")]
pub type TestGraph = graph_core::graph::_Graph<graph_core::packed_compressed_sparse_row::PackedCompressedSparseRow>;
#[cfg(feature = "test_disk")]
pub type TestGraph = graph_core::graph::_Graph<graph_core::disk_graph::DiskGraph>;
#[cfg(not(any(feature = "test_matrix", feature = "test_csr", feature = "test_pcsr", feature = "test_disk")))]
pub type TestGraph = graph_core::graph::_Graph<graph_core::adjacency_list::AdjacencyList>;

pub use graph_core::graph::{ConnectionProperty};

pub use graph_py::py_graph::GraphStructureType;
pub use graph_py::py_graph::Graph;
pub use graph_py::py_node::Node;
use pyo3::prelude::*;
use pyo3_stub_gen::*;

use crate::layout::layout::Layout;
use crate::layout::style::GraphStyle;

use crate::external_apis::core::OpenAlexGraphType;

#[pymodule]
fn netfog(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Node>()?;
    m.add_class::<GraphStructureType>()?;
    m.add_class::<Graph>()?;
    m.add_class::<Layout>()?;
    m.add_class::<GraphStyle>()?;
    m.add_class::<OpenAlexGraphType>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
