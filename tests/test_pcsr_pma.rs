use netfog::graph_core::packed_compressed_sparse_row::PackedCompressedSparseRow;
use netfog::graph_core::graph_structure_interface::IGraphStructure;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[test]
fn test_neighbors_correct_through_many_doublings() {
    let n: usize = 3_000;
    let mut structure = PackedCompressedSparseRow::default();
    structure.batch_add_nodes(n);

    let mut rng = StdRng::seed_from_u64(42);
    let mut expected: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut connections: Vec<(usize, usize, f32, bool)> = Vec::new();
    for _ in 0..(n * 10) {
        let a = rng.gen_range(0..n);
        let mut b = rng.gen_range(0..n);
        if b == a { b = (b + 1) % n; }
        connections.push((a, b, 1.0, false));
        expected[a].push(b);
        expected[b].push(a);
    }

    structure.batch_create_connections(&connections);

    for node in 0..n {
        let mut got = structure.get_neighbors_ids(node);
        got.sort();
        got.dedup();
        let mut want = expected[node].clone();
        want.sort();
        want.dedup();
        assert_eq!(got, want, "mismatch at node {}", node);
    }
}

#[test]
fn test_directed_and_weight_update_survive_rebalancing() {
    let n: usize = 2_000;
    let mut structure = PackedCompressedSparseRow::default();
    structure.batch_add_nodes(n);

    let mut connections: Vec<(usize, usize, f32, bool)> = Vec::new();
    for i in 0..n {
        connections.push((i, (i + 1) % n, 1.0, true));
    }
    structure.batch_create_connections(&connections);

    for i in 0..n {
        assert!(structure.has_connection(i, (i + 1) % n));
        assert!(!structure.has_connection((i + 1) % n, i));
    }

    structure.create_connection(0, 1, 9.5, Some(true));
    let edge = structure.get_all_edges().find(|&(u, v, _, _)| u == 0 && v == 1);
    assert_eq!(edge, Some((0, 1, 9.5, true)));
}

#[test]
#[ignore]
fn test_large_batch_insert_does_not_blow_up_memory() {
     let n: usize = 100_000;
    let avg_degree: usize = 10;
    let mut structure = PackedCompressedSparseRow::default();
    structure.batch_add_nodes(n);

    let mut rng = StdRng::seed_from_u64(7);
    let num_edges = n * avg_degree / 2;
    let mut connections: Vec<(usize, usize, f32, bool)> = Vec::with_capacity(num_edges);
    for _ in 0..num_edges {
        let a = rng.gen_range(0..n);
        let mut b = rng.gen_range(0..n);
        if b == a { b = (b + 1) % n; }
        connections.push((a, b, 1.0, false));
    }

    structure.batch_create_connections(&connections);

    assert_eq!(structure.node_count(), n);
}
