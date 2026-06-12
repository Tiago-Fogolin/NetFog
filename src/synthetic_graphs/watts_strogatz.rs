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

    for i in 0..n {
        graph.add_node(i.to_string());
    }

    // Build ring lattice: node i connects to i+j mod n for j in 1..=k/2.
    // Each directed pair (i, neighbor) is unique; as undirected it gives n*k/2 edges.
    // Track adjacency for rewiring collision checks.
    let mut adj: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n {
        for j in 1..=(k / 2) {
            let nb = (i + j) % n;
            adj.insert((i.min(nb), i.max(nb)));
        }
    }

    if beta <= 0.0 {
        for &(u, v) in &adj {
            graph.create_connection(u.to_string(), v.to_string(), 1.0, Some(false));
        }
        return graph;
    }

    let mut rng = rand::thread_rng();

    // Process in deterministic ring order so beta=1.0 still rewires systematically.
    let ring_pairs: Vec<(usize, usize)> = (0..n)
        .flat_map(|i| (1..=(k / 2)).map(move |j| (i, (i + j) % n)))
        .collect();

    // Working adjacency set for rewiring (directed, both directions)
    let mut working: HashSet<(usize, usize)> = ring_pairs
        .iter()
        .flat_map(|&(u, v)| [(u, v), (v, u)])
        .collect();

    let mut final_edges: HashSet<(usize, usize)> = HashSet::new();

    for (u, v) in ring_pairs {
        // Only process each undirected edge once (from the smaller-offset direction)
        let canonical = (u.min(v), u.max(v));
        if final_edges.contains(&canonical) {
            continue;
        }

        if rng.gen_range(0.0..1.0) < beta {
            // Try to rewire (u, v) → (u, w) where w ≠ u and not already adjacent
            let mut w = rng.gen_range(0..n);
            let mut attempts = 0;
            while (w == u || working.contains(&(u, w))) && attempts < n {
                w = (w + 1) % n;
                attempts += 1;
            }

            if w != u && !working.contains(&(u, w)) {
                // Remove old edge from working set
                working.remove(&(u, v));
                working.remove(&(v, u));
                // Add new edge
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

    for &(u, v) in &final_edges {
        graph.create_connection(u.to_string(), v.to_string(), 1.0, Some(false));
    }

    return graph;
}
