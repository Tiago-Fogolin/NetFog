use crate::graph_core::graph_structure_interface::IGraphStructure;

pub struct AdjacencyList {
    pub node_count: usize,
    targets: Vec<Vec<usize>>,
    weights: Vec<Vec<f32>>,
    directed_flags: Vec<Vec<bool>>,
    incoming_directed: Vec<Vec<(usize, f32)>>,
}

impl Default for AdjacencyList {
    fn default() -> Self {
        return AdjacencyList {
            node_count: 0,
            targets: Vec::new(),
            weights: Vec::new(),
            directed_flags: Vec::new(),
            incoming_directed: Vec::new(),
        };
    }
}

impl AdjacencyList {
    fn remove_target(targets: &mut Vec<usize>, weights: &mut Vec<f32>, directed_flags: &mut Vec<bool>, needle: usize) {
        let mut i = 0;
        while i < targets.len() {
            if targets[i] == needle {
                targets.remove(i);
                weights.remove(i);
                directed_flags.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

impl IGraphStructure for AdjacencyList {
    fn add_node(&mut self) -> usize {
        self.node_count += 1;
        self.targets.push(Vec::new());
        self.weights.push(Vec::new());
        self.directed_flags.push(Vec::new());
        self.incoming_directed.push(Vec::new());
        return self.node_count - 1;
    }

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>) {
        let is_directed = directed.unwrap_or(false);
        self.targets[from_index].push(to_index);
        self.weights[from_index].push(weight);
        self.directed_flags[from_index].push(is_directed);

        if is_directed {
            self.incoming_directed[to_index].push((from_index, weight));
        } else if from_index != to_index {
            self.targets[to_index].push(from_index);
            self.weights[to_index].push(weight);
            self.directed_flags[to_index].push(false);
        }
    }

    fn remove_connection(&mut self, from_index: usize, to_index: usize) {
        Self::remove_target(&mut self.targets[from_index], &mut self.weights[from_index], &mut self.directed_flags[from_index], to_index);
        Self::remove_target(&mut self.targets[to_index], &mut self.weights[to_index], &mut self.directed_flags[to_index], from_index);

        self.incoming_directed[to_index].retain(|&(src, _)| src != from_index);
        self.incoming_directed[from_index].retain(|&(src, _)| src != to_index);
    }

    fn has_connection(&self, source_index: usize, target_index: usize) -> bool {
        return self.targets[source_index].iter().zip(self.weights[source_index].iter())
            .any(|(&tgt, &w)| tgt == target_index && w != 0.);
    }

    fn node_count(&self) -> usize {
        return self.node_count;
    }

    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();

        for idx in 0..self.targets[node_index].len() {
            if self.weights[node_index][idx] > 0. {
                ids.push(self.targets[node_index][idx]);
            }
        }

        for &(src, w) in &self.incoming_directed[node_index] {
            if w > 0. {
                ids.push(src);
            }
        }

        ids.sort();
        ids.dedup();
        return ids;
    }

    fn batch_add_nodes(&mut self, additional_count: usize) {
        self.node_count += additional_count;
        for _ in 0..additional_count {
            self.targets.push(Vec::new());
            self.weights.push(Vec::new());
            self.directed_flags.push(Vec::new());
            self.incoming_directed.push(Vec::new());
        }
    }

    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        let n = self.node_count;
        let mut added_out_degree = vec![0usize; n];
        let mut added_incoming_directed = vec![0usize; n];

        for &(from, to, _weight, directed) in connections {
            added_out_degree[from] += 1;
            if directed {
                added_incoming_directed[to] += 1;
            } else if from != to {
                added_out_degree[to] += 1;
            }
        }

        for i in 0..n {
            if added_out_degree[i] > 0 {
                self.targets[i].reserve(added_out_degree[i]);
                self.weights[i].reserve(added_out_degree[i]);
                self.directed_flags[i].reserve(added_out_degree[i]);
            }
            if added_incoming_directed[i] > 0 {
                self.incoming_directed[i].reserve(added_incoming_directed[i]);
            }
        }

        for conn in connections {
            self.create_connection(conn.0, conn.1, conn.2, Some(conn.3));
        }
    }

    fn get_all_edges(&self) -> impl Iterator<Item = (usize, usize, f32, bool)> + '_ {
        return (0..self.node_count).flat_map(move |i| {
            (0..self.targets[i].len()).filter_map(move |idx| {
                let j = self.targets[i][idx];
                let weight = self.weights[i][idx];
                let directed = self.directed_flags[i][idx];
                if weight == 0. {
                    return None;
                }
                if directed {
                    return Some((i, j, weight, true));
                }
                if i <= j {
                    return Some((i, j, weight, false));
                }
                return None;
            })
        });
    }
}
