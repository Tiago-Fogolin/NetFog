// Estrutura experimental, feita so pra "brincar" -- nao e production-ready.
// So suporta grafo nao-direcionado (create_connection/batch_create_connections
// com directed=true panica de proposito). Ideia: em vez de mergear a entrada
// direcionada com a saida a cada get_neighbors_ids (como as outras 4
// estruturas fazem, via incoming_directed + HashSet/sort em tempo de
// consulta), aqui o merge/sort/dedup acontece 1 vez na escrita
// (batch_create_connections), deixando cada segmento de no sempre ordenado
// e sem duplicata -- get_neighbors_ids vira so uma copia de slice, sem sort,
// sem dedup, sem hash. Serve de exemplo no TCC de uma estrutura desenhada
// sob medida pra um unico padrao de acesso (BFS/DFS), trocando escrita cara
// por leitura praticamente de graca -- citar esse arquivo no artigo.

use crate::graph_core::graph_structure_interface::IGraphStructure;

pub struct BfsCsr {
    pub node_count: usize,
    pub offsets: Vec<usize>,
    pub edges: Vec<usize>,
    pub weights: Vec<f32>,
}

impl Default for BfsCsr {
    fn default() -> Self {
        return BfsCsr {
            node_count: 0,
            offsets: vec![0],
            edges: Vec::new(),
            weights: Vec::new(),
        };
    }
}

impl IGraphStructure for BfsCsr {
    fn add_node(&mut self) -> usize {
        self.node_count += 1;
        self.offsets.push(*self.offsets.last().unwrap());

        return self.node_count - 1;
    }

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>) {
        self.batch_create_connections(&[(from_index, to_index, weight, directed.unwrap_or(false))]);

        return;
    }

    fn remove_connection(&mut self, from_index: usize, to_index: usize) {
        let start = self.offsets[from_index];
        let end = self.offsets[from_index + 1];

        if let Some(pos) = (start..end).find(|&i| self.edges[i] == to_index) {
            self.edges.remove(pos);
            self.weights.remove(pos);
            for k in (from_index + 1)..self.offsets.len() {
                self.offsets[k] -= 1;
            }
        }

        if from_index != to_index {
            let start = self.offsets[to_index];
            let end = self.offsets[to_index + 1];

            if let Some(pos) = (start..end).find(|&i| self.edges[i] == from_index) {
                self.edges.remove(pos);
                self.weights.remove(pos);
                for k in (to_index + 1)..self.offsets.len() {
                    self.offsets[k] -= 1;
                }
            }
        }

        return;
    }

    fn has_connection(&self, source_index: usize, target_index: usize) -> bool {
        let start = self.offsets[source_index];
        let end = self.offsets[source_index + 1];

        return self.edges[start..end].binary_search(&target_index).is_ok();
    }

    fn node_count(&self) -> usize {
        return self.node_count;
    }

    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize> {
        let start = self.offsets[node_index];
        let end = self.offsets[node_index + 1];

        return self.edges[start..end].to_vec();
    }

    fn batch_add_nodes(&mut self, additional_count: usize) {
        self.offsets.reserve(additional_count);
        let last = *self.offsets.last().unwrap();
        for _ in 0..additional_count {
            self.offsets.push(last);
        }
        self.node_count += additional_count;

        return;
    }

    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        let n = self.node_count;

        for &(from, to, _weight, directed) in connections {
            assert!(from < n && to < n, "batch_create_connections: index out of bounds");
            assert!(!directed, "BfsCsr: estrutura so suporta grafo nao-direcionado");
        }

        let mut adjacency: Vec<Vec<(usize, f32)>> = vec![Vec::new(); n];

        for i in 0..n {
            let start = self.offsets[i];
            let end = self.offsets[i + 1];
            for j in start..end {
                adjacency[i].push((self.edges[j], self.weights[j]));
            }
        }

        for &(from, to, weight, _directed) in connections {
            adjacency[from].push((to, weight));
            if from != to {
                adjacency[to].push((from, weight));
            }
        }

        for neighbors in adjacency.iter_mut() {
            neighbors.sort_unstable_by_key(|&(id, _)| id);
            neighbors.dedup_by_key(|&mut (id, _)| id);
        }

        let mut offsets: Vec<usize> = vec![0; n + 1];
        for i in 0..n {
            offsets[i + 1] = offsets[i] + adjacency[i].len();
        }

        let total = offsets[n];
        let mut edges: Vec<usize> = Vec::with_capacity(total);
        let mut weights: Vec<f32> = Vec::with_capacity(total);

        for neighbors in &adjacency {
            for &(id, w) in neighbors {
                edges.push(id);
                weights.push(w);
            }
        }

        self.offsets = offsets;
        self.edges = edges;
        self.weights = weights;

        return;
    }

    fn get_all_edges(&self) -> impl Iterator<Item = (usize, usize, f32, bool)> + '_ {
        return (0..self.node_count).flat_map(move |i| {
            let start = self.offsets[i];
            let end = self.offsets[i + 1];
            (start..end).filter_map(move |j| {
                let tgt = self.edges[j];
                let weight = self.weights[j];
                if weight == 0.0 {
                    return None;
                }
                if i <= tgt {
                    return Some((i, tgt, weight, false));
                }
                return None;
            })
        });
    }
}
