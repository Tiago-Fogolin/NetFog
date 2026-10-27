use crate::graph_core::graph::_Graph;
use crate::graph_core::graph_structure_interface::IGraphStructure;
use rand::Rng;

/// Generates an Erdős–Rényi G(n, p) random graph.
///
/// Each pair of nodes is connected with independent probability `p`.
pub fn generate_erdos_renyi<S: IGraphStructure + Default>(n: usize, p: f64) -> _Graph<S> {
    let mut graph = _Graph::<S>::default();
    graph.batch_add_anonymous_nodes(n);

    if p <= 0.0 {
        return graph;
    }

    let mut rng = rand::thread_rng();
    let mut edges: Vec<(usize, usize, f32, bool)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if p >= 1.0 || rng.gen_range(0.0..1.0) < p {
                edges.push((i, j, 1.0, false));
            }
        }
    }

    graph.batch_create_connections_from_ids(&edges);

    return graph;
}
