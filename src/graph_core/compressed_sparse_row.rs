use crate::graph_core::graph_structure_interface::IGraphStructure;

pub struct CompressedSparseRow {
    pub node_count: usize,
    pub offsets: Vec<usize>,
    pub edges: Vec<usize>,
    pub weights: Vec<f32>,
    pub directed_flags: Vec<bool>,
    incoming_directed: Vec<Vec<usize>>,
}

impl Default for CompressedSparseRow {
    fn default() -> Self {
        CompressedSparseRow {
            node_count: 0,
            offsets: vec![0],
            edges: Vec::new(),
            weights: Vec::new(),
            directed_flags: Vec::new(),
            incoming_directed: Vec::new(),
        }
    }
}

fn counting_sort_indices_by<T: Copy>(items: &[T], bucket_count: usize, key: impl Fn(&T) -> usize) -> Vec<usize> {
    let mut bucket_starts = vec![0usize; bucket_count + 1];
    for item in items {
        bucket_starts[key(item) + 1] += 1;
    }
    for i in 0..bucket_count {
        bucket_starts[i + 1] += bucket_starts[i];
    }

    let mut cursor = bucket_starts.clone();
    let mut order = vec![0usize; items.len()];
    for (idx, item) in items.iter().enumerate() {
        let k = key(item);
        order[cursor[k]] = idx;
        cursor[k] += 1;
    }

    return order;
}

impl IGraphStructure for CompressedSparseRow {
    fn add_node(&mut self) -> usize {
        self.node_count += 1;
        self.offsets.push(*self.offsets.last().unwrap());
        self.incoming_directed.push(Vec::new());

        return self.node_count - 1;
    }

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>) {
        let is_directed = directed.unwrap_or(false);
        let insert_pos = self.offsets[from_index + 1];

        self.edges.insert(insert_pos, to_index);
        self.weights.insert(insert_pos, weight);
        self.directed_flags.insert(insert_pos, is_directed);

        for k in (from_index + 1)..self.offsets.len() {
            self.offsets[k] += 1;
        }

        if is_directed {
            self.incoming_directed[to_index].push(from_index);
        } else if from_index != to_index {
            let insert_pos_rev = self.offsets[to_index + 1];
            self.edges.insert(insert_pos_rev, from_index);
            self.weights.insert(insert_pos_rev, weight);
            self.directed_flags.insert(insert_pos_rev, false);
            for k in (to_index + 1)..self.offsets.len() {
                self.offsets[k] += 1;
            }
        }
    }
    fn remove_connection(&mut self, from_index: usize, to_index: usize) {
        let start = self.offsets[from_index];
        let end = self.offsets[from_index + 1];

        if let Some(pos) = (start..end).find(|&i| self.edges[i] == to_index) {
            self.edges.remove(pos);
            self.weights.remove(pos);
            self.directed_flags.remove(pos);
            for k in (from_index + 1)..self.offsets.len() {
                self.offsets[k] -= 1;
            }
        }

        let start = self.offsets[to_index];
        let end = self.offsets[to_index + 1];

        if let Some(pos) = (start..end).find(|&i| self.edges[i] == from_index) {
            self.edges.remove(pos);
            self.weights.remove(pos);
            self.directed_flags.remove(pos);
            for k in (to_index + 1)..self.offsets.len() {
                self.offsets[k] -= 1;
            }
        }

        self.incoming_directed[to_index].retain(|&src| src != from_index);
        self.incoming_directed[from_index].retain(|&src| src != to_index);
    }

    fn has_connection(&self, source_index: usize, target_index: usize) -> bool {
        let start = self.offsets[source_index];
        let end = self.offsets[source_index + 1];
        self.edges[start..end].iter().any(|&e| e == target_index)
    }

    fn node_count(&self) -> usize {
        return self.node_count;
    }

    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize> {
        use std::collections::HashSet;
        let mut nbors = HashSet::new();
        let start = self.offsets[node_index];
        let end = self.offsets[node_index + 1];

        for &tgt in &self.edges[start..end] {
            nbors.insert(tgt);
        }

        for &src in &self.incoming_directed[node_index] {
            nbors.insert(src);
        }

        let mut ids: Vec<usize> = nbors.into_iter().collect();
        ids.sort();

        return ids;
    }

    fn batch_add_nodes(&mut self, additional_count: usize) {
        self.offsets.reserve(additional_count);
        let last = *self.offsets.last().unwrap();
        for _ in 0..additional_count {
            self.offsets.push(last);
        }
        self.incoming_directed.reserve(additional_count);
        for _ in 0..additional_count {
            self.incoming_directed.push(Vec::new());
        }
        self.node_count += additional_count;
    }

    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        let n = self.node_count;

        for &(from, to, _, _) in connections {
            assert!(from < n && to < n, "batch_create_connections: index out of bounds");
        }

        let mut new_offsets: Vec<usize> = vec![0; n + 1];

        for i in 0..n {
            new_offsets[i + 1] = self.offsets[i + 1] - self.offsets[i];
        }

        for &(from, to, _weight, directed) in connections {
            new_offsets[from + 1] += 1;
            if !directed && from != to {
                new_offsets[to + 1] += 1;
            }
        }

        for i in 0..n {
            new_offsets[i + 1] += new_offsets[i];
        }

        let total_edges = new_offsets[n];

        let mut new_edges: Vec<usize> = Vec::with_capacity(total_edges);
        let mut new_weights: Vec<f32> = Vec::with_capacity(total_edges);
        let mut new_directed_flags: Vec<bool> = Vec::with_capacity(total_edges);
        unsafe {
            new_edges.set_len(total_edges);
            new_weights.set_len(total_edges);
            new_directed_flags.set_len(total_edges);
        }

        let mut cursors: Vec<usize> = new_offsets[..n].to_vec();

        unsafe {
            for i in 0..n {
                let start = self.offsets[i];
                let end = self.offsets[i + 1];
                for j in start..end {
                    let pos = *cursors.get_unchecked(i);
                    *new_edges.get_unchecked_mut(pos) = self.edges[j];
                    *new_weights.get_unchecked_mut(pos) = self.weights[j];
                    *new_directed_flags.get_unchecked_mut(pos) = self.directed_flags[j];
                    *cursors.get_unchecked_mut(i) += 1;
                }
            }
        }

        let forward_order = counting_sort_indices_by(connections, n, |&(from, _, _, _)| from);
        unsafe {
            for &idx in &forward_order {
                let (from, to, weight, directed) = connections[idx];
                let pos = *cursors.get_unchecked(from);
                *new_edges.get_unchecked_mut(pos) = to;
                *new_weights.get_unchecked_mut(pos) = weight;
                *new_directed_flags.get_unchecked_mut(pos) = directed;
                *cursors.get_unchecked_mut(from) += 1;
            }
        }

        let mirrors: Vec<(usize, usize, f32, bool)> = connections.iter()
            .copied()
            .filter(|&(from, to, _, directed)| !directed && from != to)
            .collect();

        if !mirrors.is_empty() {
            let reverse_order = counting_sort_indices_by(&mirrors, n, |&(_, to, _, _)| to);
            unsafe {
                for &idx in &reverse_order {
                    let (from, to, weight, _) = mirrors[idx];
                    let pos_rev = *cursors.get_unchecked(to);
                    *new_edges.get_unchecked_mut(pos_rev) = from;
                    *new_weights.get_unchecked_mut(pos_rev) = weight;
                    *new_directed_flags.get_unchecked_mut(pos_rev) = false;
                    *cursors.get_unchecked_mut(to) += 1;
                }
            }
        }

        for &(from, to, _weight, directed) in connections {
            if directed {
                self.incoming_directed[to].push(from);
            }
        }

        self.offsets = new_offsets;
        self.edges = new_edges;
        self.weights = new_weights;
        self.directed_flags = new_directed_flags;
    }

    fn get_all_edges(&self) -> impl Iterator<Item = (usize, usize, f32, bool)> + '_ {
        return (0..self.node_count).flat_map(move |i| {
            let start = self.offsets[i];
            let end = self.offsets[i + 1];
            (start..end).filter_map(move |j| {
                let tgt = self.edges[j];
                let weight = self.weights[j];
                let directed = self.directed_flags[j];
                if weight == 0.0 {
                    return None;
                }
                if directed {
                    return Some((i, tgt, weight, true));
                }
                if i <= tgt {
                    return Some((i, tgt, weight, false));
                }
                return None;
            })
        });
    }
}
