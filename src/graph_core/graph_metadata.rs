use std::rc::Rc;
use rustc_hash::FxHashMap;
use crate::_Node;

#[derive(Clone)]
pub struct GraphMetadata {
    pub label_id_map: FxHashMap<Rc<str>, usize>,
    pub node_info: Vec<_Node>,
    pub label_mode: Option<bool>,
}

impl GraphMetadata {
    pub fn new() -> Self {
        GraphMetadata {
            label_id_map: FxHashMap::default(),
            node_info: Vec::new(),
            label_mode: None,
        }
    }

    pub fn check_label_mode(&mut self, is_label: bool) {
        match self.label_mode {
            None => self.label_mode = Some(is_label),
            Some(mode) if mode != is_label => {
                if is_label {
                    panic!("Cannot create a labeled node: this graph was created with id-only nodes (add_nodes/batch_add_anonymous_nodes) -- mixing id-only and labeled node creation is not allowed");
                } else {
                    panic!("Cannot create an id-only node: this graph already has labeled nodes -- mixing id-only and labeled node creation is not allowed");
                }
            },
            _ => {},
        }
    }
}
