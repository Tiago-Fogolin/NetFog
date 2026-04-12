use crate::graph_core::graph_structure_interface::IGraphStructure;

pub struct AdjacencyMatrix {
    pub node_count: usize,
    pub mtz: Vec<Vec<(f32, bool)>>
}

impl Default for AdjacencyMatrix {
    fn default() -> Self {
        return AdjacencyMatrix {
            node_count: 0,
            mtz: Vec::new()
        }

    }
}

impl IGraphStructure for AdjacencyMatrix {
    fn add_node(&mut self) -> usize {
        self.node_count += 1;

        for row in &mut self.mtz {
            row.push((0.0, false));
        }

        let new_row = vec![(0.0, false); self.node_count];

        self.mtz.push(new_row);

        return self.node_count - 1;
    }

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>) {
        let is_directed = directed.unwrap_or(false);
        self.mtz[from_index][to_index] = (weight, is_directed);
        if !is_directed {
            self.mtz[to_index][from_index] = (weight, false);
        }
    }

    fn remove_connection(&mut self, from_index: usize, to_index: usize) {
        self.mtz[from_index][to_index] = (0., false);
        self.mtz[to_index][from_index] = (0., false);
    }

    fn has_connection(&self, source_index: usize, target_index: usize) -> bool {
        return self.mtz[source_index][target_index].0 != 0.;
    }

    fn node_count(&self) -> usize {
        return self.node_count;
    }

    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();


        for i in 0..self.node_count {
            if self.mtz[i][node_index].0 > 0. || self.mtz[node_index][i].0 > 0. {
                ids.push(i);
            }
        }


        return ids;
    }

    fn batch_add_nodes(&mut self, additional_count: usize) {
        let new_total_size = self.node_count + additional_count;

        for row in &mut self.mtz {
            row.resize(new_total_size, (0.0, false));
        }

        for _ in 0..additional_count {
            self.mtz.push(vec![(0.0, false); new_total_size]);
        }

        self.node_count = new_total_size;
    }

    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        for connection in connections {
            let from = connection.0;
            let to = connection.1;
            let weight = connection.2;
            let directed = connection.3;

            self.mtz[from][to] = (weight, directed);
            if !directed {
                self.mtz[to][from] = (weight, false);
            }
        }

    }

    fn get_all_edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f32, bool)>> {
        let mut edges = Vec::new();
        for i in 0..self.node_count {
            for j in 0..self.node_count {
                let (weight, directed) = self.mtz[i][j];
                if weight != 0.0 {
                    if directed {
                        edges.push((i, j, weight, true));
                    } else {
                        if i <= j {
                            edges.push((i, j, weight, false));
                        }
                    }
                }
            }
        }
        return Box::new(edges.into_iter());
    }
}
