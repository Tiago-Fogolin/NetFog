pub trait IGraphStructure {
    fn add_node(&mut self) -> usize;

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>);
    fn remove_connection(&mut self, from_index: usize, to_index: usize);
    fn has_connection(&self, source_index: usize, target_index: usize) -> bool;

    fn node_count(&self) -> usize;
    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize>;

    fn batch_add_nodes(&mut self, count: usize);
    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]);
    
    fn get_all_edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f32, bool)>>;

    fn resolve_label(&self, _id: usize) -> Option<String> {
        return None;
    }

    /// Returns true if this structure manages label↔id mappings internally (e.g. on disk).
    /// When true, _Graph will NOT write to GraphMetadata for labels.
    fn manages_labels(&self) -> bool { false }

    fn set_node_label(&mut self, _id: usize, _label: &str) {}

    fn get_id_by_label(&self, _label: &str) -> Option<usize> { None }

    /// Returns true if this structure manages node positions internally (e.g. on disk).
    /// When true, _Graph will NOT write positions to GraphMetadata.
    fn manages_positions(&self) -> bool { false }

    fn set_node_position(&mut self, _id: usize, _x: f64, _y: f64) {}

    fn get_node_position(&self, _id: usize) -> Option<(f64, f64)> { None }
}
