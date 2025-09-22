
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Connection {
    pub node: Weak<RefCell<_Node>>,
    pub weight: i32,
    pub directed: bool
}


pub struct _Node {
    pub connections: Vec<Connection>,
    pub label: String
}

impl _Node {
    pub fn add_connection(&mut self, node: Rc<RefCell<_Node>>, weight: i32, directed: Option<bool>) {
        let directed = directed.unwrap_or(false);
        let new_conn = Connection {
            node: Rc::downgrade(&node),
            weight: weight,
            directed: directed
        };

        self.connections.push(new_conn);
    }
}