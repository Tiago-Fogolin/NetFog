use crate::graph_core::graph::_Graph;
use crate::graph_core::graph_structure_interface::IGraphStructure;
use rand::Rng;
use std::collections::HashSet;

/// Generates a Watts–Strogatz small-world graph.
///
/// Starts from a regular ring lattice where every node is connected to its
/// `k` nearest neighbours, then rewires each edge with probability `beta`.
pub fn generate_watts_strogatz<S: IGraphStructure + Default>(n: usize, k: usize, beta: f64) -> _Graph<S> {
    assert!(k % 2 == 0, "k must be even");
    assert!(k < n, "k must be less than n");

    let mut graph = _Graph::<S>::default();
    graph.batch_add_anonymous_nodes(n);

    let mut adj: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n {
        for j in 1..=(k / 2) {
            let nb = (i + j) % n;
            adj.insert((i.min(nb), i.max(nb)));
        }
    }

    if beta <= 0.0 {
        let edges: Vec<(usize, usize, f32, bool)> = adj.iter().map(|&(u, v)| (u, v, 1.0, false)).collect();
        graph.batch_create_connections_from_ids(&edges);
        return graph;
    }

    let mut rng = rand::thread_rng();

    let ring_pairs: Vec<(usize, usize)> = (0..n)
        .flat_map(|i| (1..=(k / 2)).map(move |j| (i, (i + j) % n)))
        .collect();

    let mut working: HashSet<(usize, usize)> = ring_pairs
        .iter()
        .flat_map(|&(u, v)| [(u, v), (v, u)])
        .collect();

    let mut final_edges: HashSet<(usize, usize)> = HashSet::new();

    for (u, v) in ring_pairs {
        let canonical = (u.min(v), u.max(v));
        if final_edges.contains(&canonical) {
            continue;
        }

        if rng.gen_range(0.0..1.0) < beta {
            let mut w = rng.gen_range(0..n);
            let mut attempts = 0;
            while (w == u || working.contains(&(u, w))) && attempts < n {
                w = (w + 1) % n;
                attempts += 1;
            }

            if w != u && !working.contains(&(u, w)) {
                working.remove(&(u, v));
                working.remove(&(v, u));
                working.insert((u, w));
                working.insert((w, u));
                final_edges.insert((u.min(w), u.max(w)));
            } else {
                final_edges.insert(canonical);
            }
        } else {
            final_edges.insert(canonical);
        }
    }

    let edges: Vec<(usize, usize, f32, bool)> = final_edges.iter().map(|&(u, v)| (u, v, 1.0, false)).collect();
    graph.batch_create_connections_from_ids(&edges);

    return graph;
}
