use crate::graph_core::graph_structure_interface::IGraphStructure;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Edge {
    dest: u32,
    weight: f32,
    directed: bool,
}

impl Edge {
    fn sentinel(node_idx: u32) -> Self {
        Edge {
            dest: u32::MAX,
            weight: f32::from_bits(node_idx),
            directed: false,
        }
    }

    fn null() -> Self {
        Edge {
            dest: 0,
            weight: 0.0,
            directed: false,
        }
    }

    fn is_sentinel(&self) -> bool {
        self.dest == u32::MAX
    }

    fn is_null(&self) -> bool {
        self.weight == 0.0 && !self.is_sentinel()
    }

    fn get_sentinel_node_idx(&self) -> u32 {
        self.weight.to_bits()
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Node {
    beginning: u32,
    end: u32,
}

pub struct PackedCompressedSparseRow {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    n: u32,
    h: u32,
    log_n: u32,
}

impl Default for PackedCompressedSparseRow {
    fn default() -> Self {
        let n = 16;
        let log_n = 4;
        let h = (n as f64 / log_n as f64).log2() as u32;

        PackedCompressedSparseRow {
            nodes: Vec::new(),
            edges: vec![Edge::null(); n as usize],
            n,
            h,
            log_n,
        }
    }
}

impl PackedCompressedSparseRow {
    fn find_node(&self, index: u32, len: u32) -> u32 {
        (index / len) * len
    }

    fn get_density(&self, index: u32, len: u32) -> f64 {
        let mut full = 0;
        let end = (index + len).min(self.n);
        for i in index..end {
            if !self.edges[i as usize].is_null() {
                full += 1;
            }
        }
        full as f64 / len as f64
    }

    fn density_bound(&self, depth: u32) -> (f64, f64) {
        let lower = 0.08 - (0.04 * depth as f64) / (self.h.max(1) as f64);
        let upper = 0.75 + (0.25 * depth as f64) / (self.h.max(1) as f64);
        (lower, upper)
    }

    fn fix_sentinel(&mut self, node_index: u32, pos: u32) {
        let idx = if node_index == u32::MAX { 0 } else { node_index as usize };
        if idx < self.nodes.len() {
            self.nodes[idx].beginning = pos;
            if idx > 0 {
                self.nodes[idx - 1].end = pos;
            }

            let last_idx = self.nodes.len() - 1;
            self.nodes[last_idx].end = self.n;
        }
    }

    fn redistribute(&mut self, index: u32, len: u32) {
        let mut space = Vec::with_capacity(len as usize);
        for i in index..index + len {
            if i < self.n && !self.edges[i as usize].is_null() {
                space.push(self.edges[i as usize]);
            }
            if i < self.n {
                self.edges[i as usize] = Edge::null();
            }
        }

        if space.is_empty() { return; }

        let step = len as f64 / space.len() as f64;
        for (i, &edge) in space.iter().enumerate() {
            let in_pos = index + (i as f64 * step).floor() as u32;
            if in_pos < self.n {
                self.edges[in_pos as usize] = edge;
                if edge.is_sentinel() {
                    self.fix_sentinel(edge.get_sentinel_node_idx(), in_pos);
                }
            }
        }
    }

    fn double_list(&mut self) {
        let old_n = self.n;
        self.n *= 2;
        self.edges.resize(self.n as usize, Edge::null());
        self.log_n = (self.n as f64).log2().log2().exp2().max(2.0) as u32;
        self.h = (self.n as f64 / self.log_n as f64).log2() as u32;
        self.redistribute(0, self.n);
    }

    fn slide_right(&mut self, index: u32) -> bool {
        let mut curr = index;
        while curr < self.n && !self.edges[curr as usize].is_null() {
            curr += 1;
        }

        if curr >= self.n {
            return false;
        }

        for i in (index..curr).rev() {
            let el = self.edges[i as usize];
            self.edges[(i + 1) as usize] = el;
            if el.is_sentinel() {
                self.fix_sentinel(el.get_sentinel_node_idx(), i + 1);
            }
        }
        self.edges[index as usize] = Edge::null();
        true
    }

    fn binary_search(&self, start: u32, end: u32, dest: u32) -> u32 {
        let mut l = start;
        let mut r = end;
        if l >= r { return l; }

        while l < r {
            let mid = l + (r - l) / 2;
            let mut check = mid;
            let mut found = false;
            let mut offset = 0;
            while mid + offset < r || (mid as i32 - offset as i32) >= l as i32 {
                if mid + offset < r && !self.edges[(mid + offset) as usize].is_null() {
                    check = mid + offset;
                    found = true;
                    break;
                }
                if (mid as i32 - offset as i32) >= l as i32 && !self.edges[(mid - offset) as usize].is_null() {
                    check = mid - offset;
                    found = true;
                    break;
                }
                offset += 1;
            }
            if !found { return mid; }
            let item_dest = self.edges[check as usize].dest;
            if item_dest == dest {
                return check;
            } else if item_dest < dest {
                l = check + 1;
            } else {
                r = check;
            }
        }
        l
    }

    fn insert(&mut self, index: u32, elem: Edge, src: u32) -> u32 {
        let mut loc = index;

        if loc >= self.n {
            self.double_list();
            loc = self.find_edge_pos(src, elem.dest);
        }

        if !self.edges[loc as usize].is_null() {
            if !elem.is_sentinel() && self.edges[loc as usize].dest == elem.dest {
                self.edges[loc as usize].weight = elem.weight;
                self.edges[loc as usize].directed = elem.directed;
                return loc;
            }

            if !self.slide_right(loc) {
                self.double_list();
                loc = self.find_edge_pos(src, elem.dest);
                return self.insert(loc, elem, src);
            }
        }

        self.edges[loc as usize] = elem;
        if elem.is_sentinel() {
            self.fix_sentinel(elem.get_sentinel_node_idx(), loc);
        }

        let mut curr_len = self.log_n;
        let mut curr_level = self.h;
        let mut node_idx = self.find_node(loc, curr_len);
        let mut d = self.get_density(node_idx, curr_len);
        let mut bounds = self.density_bound(curr_level);

        while d >= bounds.1 {
            curr_len *= 2;
            if curr_len <= self.n {
                curr_level = curr_level.saturating_sub(1);
                node_idx = self.find_node(loc, curr_len);
                d = self.get_density(node_idx, curr_len);
                bounds = self.density_bound(curr_level);
            } else {
                self.double_list();
                return self.find_edge_pos(src, elem.dest);
            }
        }
        self.redistribute(node_idx, curr_len);
        self.find_edge_pos(src, elem.dest)
    }

    fn find_edge_pos(&self, src: u32, dest: u32) -> u32 {
        if src as usize >= self.nodes.len() { return self.n; }
        let node = self.nodes[src as usize];
        self.binary_search(node.beginning + 1, node.end, dest)
    }
}

impl IGraphStructure for PackedCompressedSparseRow {
    fn add_node(&mut self) -> usize {
        let node_idx = self.nodes.len() as u32;
        let mut node = Node::default();
        if node_idx > 0 {
            node.beginning = self.nodes[node_idx as usize - 1].end.saturating_sub(1);
            node.end = self.n;
        } else {
            node.beginning = 0;
            node.end = self.n;
        }
        self.nodes.push(node);
        let sentinel = if node_idx == 0 { Edge::sentinel(u32::MAX) } else { Edge::sentinel(node_idx) };
        self.insert(node.beginning, sentinel, node_idx);
        node_idx as usize
    }

    fn create_connection(&mut self, from: usize, to: usize, weight: f32, directed: Option<bool>) {
        if weight == 0.0 { return; }
        let is_directed = directed.unwrap_or(false);
        let loc = self.find_edge_pos(from as u32, to as u32);
        let edge = Edge { dest: to as u32, weight, directed: is_directed };
        self.insert(loc, edge, from as u32);
        if !is_directed && from != to {
            let loc_rev = self.find_edge_pos(to as u32, from as u32);
            let edge_rev = Edge { dest: from as u32, weight, directed: false };
            self.insert(loc_rev, edge_rev, to as u32);
        }
    }

    fn remove_connection(&mut self, from: usize, to: usize) {
        let loc = self.find_edge_pos(from as u32, to as u32);
        if loc < self.n && self.edges[loc as usize].dest == to as u32 {
            let directed = self.edges[loc as usize].directed;
            self.edges[loc as usize] = Edge::null();
            self.redistribute(self.find_node(loc, self.log_n), self.log_n);
            if !directed && from != to {
                let loc_rev = self.find_edge_pos(to as u32, from as u32);
                if loc_rev < self.n && self.edges[loc_rev as usize].dest == from as u32 {
                    self.edges[loc_rev as usize] = Edge::null();
                    self.redistribute(self.find_node(loc_rev, self.log_n), self.log_n);
                }
            }
        }
    }

    fn has_connection(&self, source: usize, target: usize) -> bool {
        if source >= self.nodes.len() { return false; }
        let loc = self.find_edge_pos(source as u32, target as u32);
        loc < self.n && !self.edges[loc as usize].is_null() && self.edges[loc as usize].dest == target as u32
    }

    fn node_count(&self) -> usize { self.nodes.len() }

    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize> {
        let mut nbors = HashSet::new();
        if node_index >= self.nodes.len() { return Vec::new(); }
        let node = self.nodes[node_index];
        for i in node.beginning + 1..node.end.min(self.n) {
            let e = self.edges[i as usize];
            if !e.is_null() && !e.is_sentinel() { nbors.insert(e.dest as usize); }
        }
        let mut ids: Vec<usize> = nbors.into_iter().collect();
        ids.sort();
        ids
    }

    fn batch_add_nodes(&mut self, count: usize) { for _ in 0..count { self.add_node(); } }
    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        for &(f, t, w, d) in connections { self.create_connection(f, t, w, Some(d)); }
    }

    fn get_all_edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f32, bool)>> {
        let mut edges = Vec::new();
        for i in 0..self.nodes.len() {
            let node = self.nodes[i];
            for j in node.beginning + 1..node.end.min(self.n) {
                let e = self.edges[j as usize];
                if !e.is_null() && !e.is_sentinel() {
                    if e.directed { edges.push((i, e.dest as usize, e.weight, true)); }
                    else if i <= e.dest as usize { edges.push((i, e.dest as usize, e.weight, false)); }
                }
            }
        }
        Box::new(edges.into_iter())
    }
}
