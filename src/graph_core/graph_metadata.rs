use std::collections::HashMap;
use crate::_Node;

#[derive(Clone)]
pub struct GraphMetadata {
    pub label_id_map: HashMap<String, usize>,
    pub node_info: Vec<_Node>
}

impl GraphMetadata {
    pub fn new() -> Self {
        GraphMetadata {
            label_id_map: HashMap::new(),
            node_info: Vec::new(),
        }
    }
}
