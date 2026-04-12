use crate::graph_core::graph_structure_interface::IGraphStructure;
use std::collections::HashSet;

pub struct AdjacencyList {
    pub node_count: usize,
    pub connections: Vec<Vec<(usize, f32, bool)>>,
}

impl Default for AdjacencyList {
    fn default() -> Self {
        return AdjacencyList {
            node_count: 0,
            connections: Vec::new(),
        };
    }
}

impl IGraphStructure for AdjacencyList {
    fn add_node(&mut self) -> usize {
        self.node_count += 1;
        self.connections.push(Vec::new());
        return self.node_count - 1;
    }

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>) {
        let is_directed = directed.unwrap_or(false);
        self.connections[from_index].push((to_index, weight, is_directed));

        if !is_directed && from_index != to_index {
            self.connections[to_index].push((from_index, weight, false));
        }
    }

    fn remove_connection(&mut self, from_index: usize, to_index: usize) {
        self.connections[from_index].retain(|&(tgt, _, _)| tgt != to_index);
        self.connections[to_index].retain(|&(tgt, _, _)| tgt != from_index);
    }

    fn has_connection(&self, source_index: usize, target_index: usize) -> bool {
        return self.connections[source_index].iter().any(|&(tgt, w, _)| tgt == target_index && w != 0.);
    }

    fn node_count(&self) -> usize {
        return self.node_count;
    }

    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize> {
        let mut nbors = HashSet::new();

        for &(tgt, w, _) in &self.connections[node_index] {
            if w > 0. {
                nbors.insert(tgt);
            }
        }

        for i in 0..self.node_count {
            for &(tgt, w, _) in &self.connections[i] {
                if tgt == node_index && w > 0. {
                    nbors.insert(i);
                }
            }
        }

        let mut ids: Vec<usize> = nbors.into_iter().collect();
        ids.sort();
        return ids;
    }

    fn batch_add_nodes(&mut self, additional_count: usize) {
        self.node_count += additional_count;
        for _ in 0..additional_count {
            self.connections.push(Vec::new());
        }
    }

    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        for conn in connections {
            self.create_connection(conn.0, conn.1, conn.2, Some(conn.3));
        }
    }

    fn get_all_edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f32, bool)>> {
        let mut edges = Vec::new();

        for i in 0..self.node_count {
            for &(j, weight, directed) in &self.connections[i] {
                if weight != 0. {
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
