use netfog::TestGraph;
use netfog::synthetic_graphs::core::SyntheticGraphType;

// ── Erdős–Rényi ──────────────────────────────────────────────────────────────

#[test]
fn test_erdos_renyi_node_count() {
    let g = TestGraph::from_synthetic(SyntheticGraphType::ErdosRenyi { n: 50, p: 0.1 });
    assert_eq!(g.get_node_count(), 50);
}

#[test]
fn test_erdos_renyi_empty_graph() {
    // p = 0.0 → no edges expected
    let mut g = TestGraph::from_synthetic(SyntheticGraphType::ErdosRenyi { n: 20, p: 0.0 });
    assert_eq!(g.get_edge_count(), 0);
}

#[test]
fn test_erdos_renyi_complete_graph() {
    // p = 1.0 → every pair connected (undirected → n*(n-1)/2 edges)
    let n = 10usize;
    let mut g = TestGraph::from_synthetic(SyntheticGraphType::ErdosRenyi { n, p: 1.0 });
    assert_eq!(g.get_edge_count(), n * (n - 1) / 2);
}

// ── Barabási–Albert ──────────────────────────────────────────────────────────

#[test]
fn test_barabasi_albert_node_count() {
    let g = TestGraph::from_synthetic(SyntheticGraphType::BarabasiAlbert { n: 100, m: 2 });
    assert_eq!(g.get_node_count(), 100);
}

#[test]
fn test_barabasi_albert_min_edges() {
    // Each of the (n - m) added nodes contributes exactly m edges → at least (n-m)*m edges
    let n = 50usize;
    let m = 3usize;
    let mut g = TestGraph::from_synthetic(SyntheticGraphType::BarabasiAlbert { n, m });
    assert!(g.get_edge_count() >= (n - m) * m);
}

// ── Watts–Strogatz ───────────────────────────────────────────────────────────

#[test]
fn test_watts_strogatz_node_count() {
    let g = TestGraph::from_synthetic(SyntheticGraphType::WattsStrogatz { n: 30, k: 4, beta: 0.1 });
    assert_eq!(g.get_node_count(), 30);
}

#[test]
fn test_watts_strogatz_edge_count_no_rewiring() {
    // beta = 0.0 → ring lattice: exactly n*k/2 edges
    let n = 20usize;
    let k = 4usize;
    let mut g = TestGraph::from_synthetic(SyntheticGraphType::WattsStrogatz { n, k, beta: 0.0 });
    assert_eq!(g.get_edge_count(), n * k / 2);
}
