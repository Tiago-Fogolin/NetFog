use crate::graph_core::graph_structure_interface::IGraphStructure;

pub struct AdjacencyMatrix {
    pub node_count: usize,
    capacity: usize,
    pub mtz: Vec<(f32, bool)>,
    incoming_directed: Vec<Vec<usize>>,
}

impl Default for AdjacencyMatrix {
    fn default() -> Self {
        return AdjacencyMatrix {
            node_count: 0,
            capacity: 0,
            mtz: Vec::new(),
            incoming_directed: Vec::new(),
        };
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

impl AdjacencyMatrix {
    fn grow_capacity(&mut self, min_capacity: usize) {
        if min_capacity <= self.capacity {
            return;
        }

        let mut new_capacity = self.capacity.max(1);
        while new_capacity < min_capacity {
            new_capacity *= 2;
        }

        let mut new_mtz = vec![(0.0, false); new_capacity * new_capacity];
        for i in 0..self.node_count {
            let old_row_start = i * self.capacity;
            let new_row_start = i * new_capacity;
            new_mtz[new_row_start..new_row_start + self.node_count]
                .copy_from_slice(&self.mtz[old_row_start..old_row_start + self.node_count]);
        }

        self.mtz = new_mtz;
        self.capacity = new_capacity;
    }
}

impl IGraphStructure for AdjacencyMatrix {
    fn add_node(&mut self) -> usize {
        self.grow_capacity(self.node_count + 1);
        self.node_count += 1;
        self.incoming_directed.push(Vec::new());
        return self.node_count - 1;
    }

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>) {
        let is_directed = directed.unwrap_or(false);
        let cap = self.capacity;
        self.mtz[from_index * cap + to_index] = (weight, is_directed);
        if is_directed {
            self.incoming_directed[to_index].push(from_index);
        } else {
            self.mtz[to_index * cap + from_index] = (weight, false);
        }
    }

    fn remove_connection(&mut self, from_index: usize, to_index: usize) {
        let cap = self.capacity;
        self.mtz[from_index * cap + to_index] = (0., false);
        self.mtz[to_index * cap + from_index] = (0., false);

        self.incoming_directed[to_index].retain(|&src| src != from_index);
        self.incoming_directed[from_index].retain(|&src| src != to_index);
    }

    fn has_connection(&self, source_index: usize, target_index: usize) -> bool {
        return self.mtz[source_index * self.capacity + target_index].0 != 0.;
    }

    fn node_count(&self) -> usize {
        return self.node_count;
    }

    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize> {
        let cap = self.capacity;
        let row_start = node_index * cap;

        let mut ids: Vec<usize> = Vec::new();
        for i in 0..self.node_count {
            if self.mtz[row_start + i].0 > 0. {
                ids.push(i);
            }
        }

        for &src in &self.incoming_directed[node_index] {
            ids.push(src);
        }

        ids.sort();
        ids.dedup();
        return ids;
    }

    fn batch_add_nodes(&mut self, additional_count: usize) {
        let new_total_size = self.node_count + additional_count;
        self.grow_capacity(new_total_size);
        self.node_count = new_total_size;
        for _ in 0..additional_count {
            self.incoming_directed.push(Vec::new());
        }
    }

    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        let n = self.node_count;
        let cap = self.capacity;

        let mut added_incoming_directed = vec![0usize; n];
        for &(_from, to, _weight, directed) in connections {
            if directed {
                added_incoming_directed[to] += 1;
            }
        }
        for i in 0..n {
            if added_incoming_directed[i] > 0 {
                self.incoming_directed[i].reserve(added_incoming_directed[i]);
            }
        }

        let order = counting_sort_indices_by(connections, n, |&(from, _, _, _)| from);
        for &idx in &order {
            let (from, to, weight, directed) = connections[idx];

            self.mtz[from * cap + to] = (weight, directed);
            if directed {
                self.incoming_directed[to].push(from);
            } else {
                self.mtz[to * cap + from] = (weight, false);
            }
        }
    }

    fn get_all_edges(&self) -> impl Iterator<Item = (usize, usize, f32, bool)> + '_ {
        let cap = self.capacity;
        return (0..self.node_count).flat_map(move |i| {
            let row_start = i * cap;
            (0..self.node_count).filter_map(move |j| {
                let (weight, directed) = self.mtz[row_start + j];
                if weight == 0.0 {
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
