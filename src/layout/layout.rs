use std::cell::RefCell;
use std::rc::Rc;

use rand::prelude::*;
use crate::{_Node, graph_core};

// Fixed for now
const MAX_WIDTH:f64 = 1500.;
const MIN_WIDTH:f64 = 20.;
const MAX_HEIGHT:f64 = 700.;
const MIN_HEIGHT:f64 = 20.;

type LayoutFn = fn(&Vec<Rc<RefCell<_Node>>>);

pub enum Layout {
    Random,
}

fn generate_random_positions(nodes: &Vec<Rc<RefCell<_Node>>>) {
    let mut rng = rand::thread_rng();

    for n in nodes {
        let mut node_ref = n.borrow_mut();

        let new_x = rng.gen_range(MIN_WIDTH..MAX_WIDTH);
        let new_y = rng.gen_range(MIN_HEIGHT..MAX_HEIGHT);

        node_ref.x = Some(new_x);
        node_ref.y = Some(new_y);
    }
}


pub fn get_layout_function(layout: Layout) -> LayoutFn {
    match layout {
        Layout::Random => generate_random_positions
    }

}
