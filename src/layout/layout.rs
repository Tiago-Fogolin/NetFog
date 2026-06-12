use std::f64::consts::PI;
use rand::prelude::*;
use crate::_Node;
use pyo3::prelude::*;

// Fixed for now
const MAX_WIDTH:f64 = 1500.;
const MIN_WIDTH:f64 = 20.;
const MAX_HEIGHT:f64 = 700.;
const MIN_HEIGHT:f64 = 20.;

const SCREEN_CENTER_X:f64 = 800.;
const SCREEN_CENTER_Y:f64 = 400.;

#[derive(Debug)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

impl BoundingBox {
    pub fn lat_lon_to_screen(&self, lat: f64, lon: f64) -> [f64; 2] {
        let geo_width = self.max_lon - self.min_lon;
        let geo_height = self.max_lat - self.min_lat;

        let screen_width = MAX_WIDTH - MIN_WIDTH;
        let screen_height = MAX_HEIGHT - MIN_HEIGHT;

        if geo_width == 0.0 || geo_height == 0.0 {
            return [SCREEN_CENTER_X, SCREEN_CENTER_Y];
        }

        let scale_x = screen_width / geo_width;
        let scale_y = screen_height / geo_height;
        let final_scale = scale_x.min(scale_y);

        let raw_x = (lon - self.min_lon) * final_scale;

        let raw_y = (self.max_lat - lat) * final_scale;

        let map_pixel_width = geo_width * final_scale;
        let map_pixel_height = geo_height * final_scale;

        let offset_x = SCREEN_CENTER_X - (map_pixel_width / 2.0);
        let offset_y = SCREEN_CENTER_Y - (map_pixel_height / 2.0);

        let final_x = offset_x + raw_x;
        let final_y = offset_y + raw_y;

        return [normalize_x(final_x), normalize_y(final_y)];
    }
}

pub fn denormalize_x(x: f64) -> f64 {
    let new_x = (x + 1.0) / 2.0 * (MAX_WIDTH - MIN_WIDTH) + MIN_WIDTH;
    return new_x;
}

pub fn denormalize_y(y: f64) -> f64 {
    let new_y = (y + 1.0) / 2.0 * (MAX_HEIGHT - MIN_HEIGHT) + MIN_HEIGHT;
    return new_y;
}

pub fn normalize_x(x: f64) -> f64 {
    let new_x = (x - MIN_WIDTH) / (MAX_WIDTH - MIN_WIDTH) * 2.0 - 1.0;
    return new_x;
}

pub fn normalize_y(y: f64) -> f64 {
    let new_y = (y - MIN_HEIGHT) / (MAX_HEIGHT - MIN_HEIGHT) * 2.0 - 1.0;
    return new_y;
}

type LayoutFn = fn(&mut [_Node], &[(usize, usize)]);

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum Layout {
    Random,
    Circular,
    Spring,
    ForceAtlas2
}

fn generate_random_positions(nodes: &mut [_Node], _edges: &[(usize, usize)]) {
    let mut rng = rand::thread_rng();

    for node_ref in nodes {
        let new_x = rng.gen_range(MIN_WIDTH..MAX_WIDTH);
        let new_y = rng.gen_range(MIN_HEIGHT..MAX_HEIGHT);

        node_ref.x = Some(normalize_x(new_x));
        node_ref.y = Some(normalize_y(new_y));
    }
}

fn generate_circular_positions(nodes: &mut [_Node], _edges: &[(usize, usize)]) {
    let len = nodes.len();
    let center_x = SCREEN_CENTER_X;
    let center_y = SCREEN_CENTER_Y;
    let radius = 200.0;

    for (i, node_ref) in nodes.iter_mut().enumerate() {
        let angle = (i as f64 / len as f64) * 2.0 * PI;

        let new_x = center_x + radius * angle.cos();
        let new_y = center_y + radius * angle.sin();

        node_ref.x = Some(normalize_x(new_x));
        node_ref.y = Some(normalize_y(new_y));
    }
}



pub fn generate_force_layout_positions(nodes: &mut [_Node], edges: &[(usize, usize)]) {
    generate_random_positions(nodes, edges);

    let iterations = 50;
    let area = MAX_WIDTH * MAX_HEIGHT;
    let k = (area / nodes.len() as f64).sqrt() * 2.;
    let mut temperature = MAX_WIDTH / 10.0;

    let fa = |d: f64, k: f64| (d * d) / k;
    let fr = |d: f64, k: f64| (k * k) / d;

    let mut pos: Vec<(f64, f64)> = nodes.iter().map(|n| (n.x.unwrap_or(0.0), n.y.unwrap_or(0.0))).collect();

    for _ in 0..iterations {
        let mut disp: Vec<(f64, f64)> = vec![(0.0, 0.0); nodes.len()];

        for v in 0..nodes.len() {
            for u in 0..nodes.len() {
                if v != u {
                    let dx = pos[v].0 - pos[u].0;
                    let dy = pos[v].1 - pos[u].1;
                    let dist = dx.hypot(dy) + 0.01;

                    let force = fr(dist, k);
                    disp[v].0 += (dx / dist) * force;
                    disp[v].1 += (dy / dist) * force;
                }
            }
        }

        for &(v, u) in edges {
            let dx = pos[v].0 - pos[u].0;
            let dy = pos[v].1 - pos[u].1;
            let dist = dx.hypot(dy) + 0.01;
            let force = fa(dist, k);

            let pull_x = (dx / dist) * force;
            let pull_y = (dy / dist) * force;

            disp[v].0 -= pull_x;
            disp[v].1 -= pull_y;
            disp[u].0 += pull_x;
            disp[u].1 += pull_y;
        }

        let gravity = 2.;
        for v in 0..nodes.len() {
            let dx = SCREEN_CENTER_X - pos[v].0;
            let dy = SCREEN_CENTER_Y - pos[v].1;

            disp[v].0 += dx * gravity;
            disp[v].1 += dy * gravity;
        }

        for v in 0..nodes.len() {
            let d = disp[v];
            let disp_len = d.0.hypot(d.1);
            if disp_len > 0.0 {
                let limited_x = (d.0 / disp_len) * disp_len.min(temperature);
                let limited_y = (d.1 / disp_len) * disp_len.min(temperature);

                pos[v].0 = (pos[v].0 + limited_x).clamp(MIN_WIDTH, MAX_WIDTH);
                pos[v].1 = (pos[v].1 + limited_y).clamp(MIN_HEIGHT, MAX_HEIGHT);
            }
        }

        temperature *= 0.95;
    }

    for (i, node) in nodes.iter_mut().enumerate() {
        node.x = Some(normalize_x(pos[i].0));
        node.y = Some(normalize_y(pos[i].1));
    }
}

pub fn generate_force_atlas_2_positions(nodes: &mut [_Node], edges: &[(usize, usize)]) {
    generate_random_positions(nodes, edges);

    let mut degrees: Vec<f64> = vec![0.0; nodes.len()];

    for &(u, v) in edges {
        degrees[u] += 1.0;
        degrees[v] += 1.0;
    }

    let iterations = 100;
    let kr = 50.0;
    let kg = 1.0;
    let center_x = SCREEN_CENTER_X;
    let center_y = SCREEN_CENTER_Y;
    let mut temperature = MAX_WIDTH / 10.0;

    let mut pos: Vec<(f64, f64)> = nodes.iter().map(|n| (n.x.unwrap_or(0.0), n.y.unwrap_or(0.0))).collect();

    for _ in 0..iterations {
        let mut disp: Vec<(f64, f64)> = vec![(0.0, 0.0); nodes.len()];

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let dx = pos[i].0 - pos[j].0;
                let dy = pos[i].1 - pos[j].1;
                let dist = dx.hypot(dy) + 0.01;

                let mass_v = degrees[i] + 1.0;
                let mass_u = degrees[j] + 1.0;

                let force = kr * (mass_v * mass_u) / dist;

                let push_x = (dx / dist) * force;
                let push_y = (dy / dist) * force;

                disp[i].0 += push_x;
                disp[i].1 += push_y;
                disp[j].0 -= push_x;
                disp[j].1 -= push_y;
            }
        }

        for &(v, u) in edges {
            let dx = pos[v].0 - pos[u].0;
            let dy = pos[v].1 - pos[u].1;
            let dist = dx.hypot(dy) + 0.01;

            let force = dist;

            let pull_x = (dx / dist) * force;
            let pull_y = (dy / dist) * force;

            disp[v].0 -= pull_x;
            disp[v].1 -= pull_y;
            disp[u].0 += pull_x;
            disp[u].1 += pull_y;
        }

        for v in 0..nodes.len() {
            let dx = center_x - pos[v].0;
            let dy = center_y - pos[v].1;
            let dist = dx.hypot(dy) + 0.01;

            let mass = degrees[v] + 1.0;
            let force = kg * mass;

            let pull_x = (dx / dist) * force;
            let pull_y = (dy / dist) * force;

            disp[v].0 += pull_x;
            disp[v].1 += pull_y;
        }

        for v in 0..nodes.len() {
            let d = disp[v];
            let disp_len = d.0.hypot(d.1);
            if disp_len > 0.0 {
                let limited_x = (d.0 / disp_len) * disp_len.min(temperature);
                let limited_y = (d.1 / disp_len) * disp_len.min(temperature);

                pos[v].0 = (pos[v].0 + limited_x).clamp(MIN_WIDTH, MAX_WIDTH);
                pos[v].1 = (pos[v].1 + limited_y).clamp(MIN_HEIGHT, MAX_HEIGHT);
            }
        }

        temperature *= 0.95;
    }

    for (i, node) in nodes.iter_mut().enumerate() {
        node.x = Some(normalize_x(pos[i].0));
        node.y = Some(normalize_y(pos[i].1));
    }
}

pub fn get_layout_function(layout: Layout) -> LayoutFn {
    match layout {
        Layout::Random => generate_random_positions,
        Layout::Circular => generate_circular_positions,
        Layout::Spring => generate_force_layout_positions,
        Layout::ForceAtlas2 => generate_force_atlas_2_positions
    }

}
