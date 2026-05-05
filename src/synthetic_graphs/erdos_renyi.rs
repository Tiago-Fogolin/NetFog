use crate::graph_core::graph::_Graph;
use crate::graph_core::graph_structure_interface::IGraphStructure;
use rand::Rng;

/// Generates an Erdős–Rényi G(n, p) random graph.
///
/// Each pair of nodes is connected with independent probability `p`.
pub fn generate_erdos_renyi<S: IGraphStructure + Default>(n: usize, p: f64) -> _Graph<S> {
    let mut graph = _Graph::<S>::default();

    for i in 0..n {
        graph.add_node(i.to_string());
    }

    if p <= 0.0 {
        return graph;
    }

    let mut rng = rand::thread_rng();
    for i in 0..n {
        for j in (i + 1)..n {
            if p >= 1.0 || rng.gen_range(0.0..1.0) < p {
                graph.create_connection(i.to_string(), j.to_string(), 1.0, Some(false));
            }
        }
    }

    return graph;
}
