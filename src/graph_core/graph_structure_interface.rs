pub trait IGraphStructure {
    fn add_node(&mut self) -> usize;

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>);
    fn remove_connection(&mut self, from_index: usize, to_index: usize);
    fn has_connection(&self, source_index: usize, target_index: usize) -> bool;

    fn node_count(&self) -> usize;
    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize>;

    fn batch_add_nodes(&mut self, count: usize);
    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]);
    
    fn get_all_edges(&self) -> Vec<(usize, usize, f32, bool)>;
}
