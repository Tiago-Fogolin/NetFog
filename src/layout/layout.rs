use std::cell::RefCell;
use std::rc::Rc;
use std::f64::consts::PI;
use rand::prelude::*;
use crate::{_Node, graph_core};
use pyo3::prelude::*;

// Fixed for now
const MAX_WIDTH:f64 = 1500.;
const MIN_WIDTH:f64 = 20.;
const MAX_HEIGHT:f64 = 700.;
const MIN_HEIGHT:f64 = 20.;

const SCREEN_CENTER_X:f64 = 800.;
const SCREEN_CENTER_Y:f64 = 400.;

pub fn denormalize_x(x: f64) -> f64 {
    let new_x = x * (MAX_WIDTH - MIN_WIDTH) + MIN_WIDTH;
    return new_x;
}

pub fn denormalize_y(y: f64) -> f64 {
    let new_y = y * (MAX_HEIGHT - MIN_HEIGHT) + MIN_HEIGHT;
    return new_y;
}

pub fn normalize_x(x: f64) -> f64 {
    let new_x = (x - MIN_WIDTH) / (MAX_WIDTH - MIN_WIDTH);
    return new_x;
}

pub fn normalize_y(y: f64) -> f64 {
    let new_y = (y - MIN_HEIGHT) / (MAX_HEIGHT - MIN_HEIGHT);
    return new_y;
}

type LayoutFn = fn(&Vec<Rc<RefCell<_Node>>>);

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum Layout {
    Random,
    Circular,
    Spring
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

fn generate_circular_positions(nodes: &Vec<Rc<RefCell<_Node>>>) {
    let len = nodes.len();
    let center_x = SCREEN_CENTER_X;
    let center_y = SCREEN_CENTER_Y;
    let radius = 200.0;

    for (i, n) in nodes.iter().enumerate() {
        let mut node_ref = n.borrow_mut();

        let angle = (i as f64 / len as f64) * 2.0 * PI;

        let new_x = center_x + radius * angle.cos();
        let new_y = center_y + radius * angle.sin();

        node_ref.x = Some(new_x);
        node_ref.y = Some(new_y);
    }
}

use std::collections::HashMap;

pub fn generate_force_layout_positions(nodes: &Vec<Rc<RefCell<_Node>>>) {
    generate_random_positions(nodes);

    let mut edges = Vec::new();
    for n in nodes {
        let node_ref = n.borrow();
        let label_a = &node_ref.label;

        for conn in &node_ref.connections {
            if let Some(target_rc) = conn.node.upgrade() {
                let target_ref = target_rc.borrow();
                let label_b = &target_ref.label;

                if label_a < label_b {
                    edges.push((label_a.clone(), label_b.clone()));
                } else {
                    edges.push((label_b.clone(), label_a.clone()));
                }
            }
        }
    }
    edges.sort();
    edges.dedup();

    let iterations = 50;
    let area = MAX_WIDTH * MAX_HEIGHT;
    let k = (area / nodes.len() as f64).sqrt();
    let mut temperature = MAX_WIDTH / 10.0;

    let fa = |d: f64, k: f64| (d * d) / k;
    let fr = |d: f64, k: f64| (k * k) / d;

    for _ in 0..iterations {
        let mut disp: HashMap<String, (f64, f64)> = nodes
            .iter()
            .map(|n| (n.borrow().label.clone(), (0.0, 0.0)))
            .collect();

        for v_rc in nodes {
            let v = v_rc.borrow();
            for u_rc in nodes {
                let u = u_rc.borrow();
                if v.label != u.label {
                    let dx = v.x.unwrap_or(0.0) - u.x.unwrap_or(0.0);
                    let dy = v.y.unwrap_or(0.0) - u.y.unwrap_or(0.0);
                    let dist = dx.hypot(dy) + 0.01;

                    let force = fr(dist, k);
                    if let Some(d) = disp.get_mut(&v.label) {
                        d.0 += (dx / dist) * force;
                        d.1 += (dy / dist) * force;
                    }
                }
            }
        }

        let pos_map: HashMap<String, (f64, f64)> = nodes
            .iter()
            .map(|n| {
                let r = n.borrow();
                (r.label.clone(), (r.x.unwrap_or(0.0), r.y.unwrap_or(0.0)))
            })
            .collect();

        for (v_label, u_label) in &edges {
            if let (Some(pos_v), Some(pos_u)) = (pos_map.get(v_label), pos_map.get(u_label)) {
                let dx = pos_v.0 - pos_u.0;
                let dy = pos_v.1 - pos_u.1;
                let dist = dx.hypot(dy) + 0.01;
                let force = fa(dist, k);

                let pull_x = (dx / dist) * force;
                let pull_y = (dy / dist) * force;

                if let Some(d) = disp.get_mut(v_label) {
                    d.0 -= pull_x;
                    d.1 -= pull_y;
                }
                if let Some(d) = disp.get_mut(u_label) {
                    d.0 += pull_x;
                    d.1 += pull_y;
                }
            }
        }

        for n_rc in nodes {
            let mut n = n_rc.borrow_mut();
            if let Some(d) = disp.get(&n.label) {
                let disp_len = d.0.hypot(d.1);
                if disp_len > 0.0 {
                    let limited_x = (d.0 / disp_len) * disp_len.min(temperature);
                    let limited_y = (d.1 / disp_len) * disp_len.min(temperature);

                    let new_x = n.x.unwrap_or(0.0) + limited_x;
                    let new_y = n.y.unwrap_or(0.0) + limited_y;

                    n.x = Some(new_x.clamp(MIN_WIDTH, MAX_WIDTH));
                    n.y = Some(new_y.clamp(MIN_HEIGHT, MAX_HEIGHT));
                }
            }
        }

        temperature *= 0.95;
    }
}


pub fn get_layout_function(layout: Layout) -> LayoutFn {
    match layout {
        Layout::Random => generate_random_positions,
        Layout::Circular => generate_circular_positions,
        Layout::Spring => generate_force_layout_positions
    }

}
