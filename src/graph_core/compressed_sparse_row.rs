use crate::graph_core::graph_structure_interface::IGraphStructure;

pub struct CompressedSparseRow {
    pub node_count: usize,
    pub offsets: Vec<usize>,
    pub edges: Vec<usize>,
    pub weights: Vec<f32>,
    pub directed_flags: Vec<bool>,
}

impl Default for CompressedSparseRow {
    fn default() -> Self {
        CompressedSparseRow {
            node_count: 0,
            offsets: vec![0],
            edges: Vec::new(),
            weights: Vec::new(),
            directed_flags: Vec::new(),
        }
    }
}

impl IGraphStructure for CompressedSparseRow {
    fn add_node(&mut self) -> usize {
        self.node_count += 1;
        self.offsets.push(*self.offsets.last().unwrap());

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

        if !is_directed && from_index != to_index {
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

        for i in 0..self.node_count {
            let s = self.offsets[i];
            let e = self.offsets[i + 1];
            if self.edges[s..e].contains(&node_index) {
                nbors.insert(i);
            }
        }

        let mut ids: Vec<usize> = nbors.into_iter().collect();
        ids.sort();

        return ids;
    }

    fn batch_add_nodes(&mut self, additional_count: usize) {
        let last = *self.offsets.last().unwrap();
        for _ in 0..additional_count {
            self.offsets.push(last);
        }
        self.node_count += additional_count;
    }

    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        let mut all_edges: Vec<Vec<(usize, f32, bool)>> = vec![Vec::new(); self.node_count];

        for i in 0..self.node_count {
            let start = self.offsets[i];
            let end = self.offsets[i + 1];
            for j in start..end {
                all_edges[i].push((self.edges[j], self.weights[j], self.directed_flags[j]));
            }
        }

        for &(from, to, weight, directed) in connections {
            all_edges[from].push((to, weight, directed));
            if !directed && from != to {
                all_edges[to].push((from, weight, false));
            }
        }

        self.offsets = vec![0];
        self.edges.clear();
        self.weights.clear();
        self.directed_flags.clear();
        for node_edges in &all_edges {
            for &(tgt, w, d) in node_edges {
                self.edges.push(tgt);
                self.weights.push(w);
                self.directed_flags.push(d);
            }
            self.offsets.push(self.edges.len());
        }
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