use crate::graph_core::graph::_Graph;
use crate::graph_core::graph_structure_interface::IGraphStructure;
use rand::Rng;
use std::collections::HashSet;

/// Generates a Barabási–Albert preferential-attachment graph.
///
/// Starts with a small seed and repeatedly adds nodes, each connecting to
/// `m` existing nodes with probability proportional to their degree.
pub fn generate_barabasi_albert<S: IGraphStructure + Default>(n: usize, m: usize) -> _Graph<S> {
    assert!(m >= 1 && m < n, "m must satisfy 1 <= m < n");

    let mut graph = _Graph::<S>::default();
    graph.batch_add_anonymous_nodes(n);

    let mut edges: Vec<(usize, usize, f32, bool)> = Vec::new();

    for i in 0..m {
        for j in (i + 1)..m {
            edges.push((i, j, 1.0, false));
        }
    }

    let mut degrees = vec![m - 1; m];
    let mut _total_degree: usize = m * (m - 1);

    let mut rng = rand::thread_rng();

    for new_node in m..n {
        degrees.push(0);

        let mut chosen: HashSet<usize> = HashSet::new();
        let attach_to = m.min(new_node);

        while chosen.len() < attach_to {
            let eligible_sum: usize = (0..new_node)
                .filter(|e| !chosen.contains(e))
                .map(|e| degrees[e].max(1))
                .sum();

            if eligible_sum == 0 {
                break;
            }

            let target_val: f64 = rng.gen_range(0.0..1.0) * (eligible_sum as f64);
            let mut cumsum = 0.0;
            let mut picked = None;

            for existing in 0..new_node {
                if !chosen.contains(&existing) {
                    cumsum += degrees[existing].max(1) as f64;
                    if cumsum > target_val {
                        picked = Some(existing);
                        break;
                    }
                }
            }

            if picked.is_none() {
                for existing in (0..new_node).rev() {
                    if !chosen.contains(&existing) {
                        picked = Some(existing);
                        break;
                    }
                }
            }

            if let Some(target) = picked {
                chosen.insert(target);
            }
        }

        for &target in &chosen {
            edges.push((new_node, target, 1.0, false));
            degrees[new_node] += 1;
            degrees[target] += 1;
            _total_degree += 2;
        }
    }

    graph.batch_create_connections_from_ids(&edges);

    return graph;
}
