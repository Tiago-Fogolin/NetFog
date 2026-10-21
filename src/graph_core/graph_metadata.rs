use rustc_hash::FxHashMap;
use crate::_Node;

#[derive(Clone)]
pub struct GraphMetadata {
    pub label_id_map: FxHashMap<String, usize>,
    pub node_info: Vec<_Node>
}

impl GraphMetadata {
    pub fn new() -> Self {
        GraphMetadata {
            label_id_map: FxHashMap::default(),
            node_info: Vec::new(),
        }
    }
}
