use crate::graph_core::graph_structure_interface::IGraphStructure;
use memmap2::MmapMut;
use redb::{Database, TableDefinition};
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

const LABEL_TO_ID: TableDefinition<&str, u64> = TableDefinition::new("label_to_id");
const ID_TO_LABEL: TableDefinition<u64, &str> = TableDefinition::new("id_to_label");

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
    pub weight: f32,
}

impl Edge {
    pub fn is_directed(&self) -> bool {
        (self.to & (1 << 31)) != 0
    }

    pub fn to_index(&self) -> u32 {
        self.to & !(1 << 31)
    }

    pub fn new(from: u32, to: u32, weight: f32, directed: bool) -> Self {
        let mut t = to;
        if directed {
            t |= 1 << 31;
        }
        Edge { from, to: t, weight }
    }
}

pub struct DiskState {
    pub edges_file: std::fs::File,
    pub offsets_file: std::fs::File,
    pub layout_file: std::fs::File,
    pub edges_mmap: MmapMut,
    pub offsets_mmap: MmapMut,
    pub layout_mmap: MmapMut,
    pub write_buffer: Vec<Edge>,
}

pub struct DiskGraph {
    pub node_count: usize,
    pub max_buffer_edges: usize,
    pub base_dir: PathBuf,
    pub metadata_db: Database,
    pub disk_state: Rc<RefCell<DiskState>>,
}

impl DiskGraph {
    pub fn new(base_dir: impl AsRef<Path>, buffer_ram_mb: usize) -> Self {
        let max_buffer_edges = (buffer_ram_mb * 1024 * 1024) / 12;
        let dir = base_dir.as_ref();
        std::fs::create_dir_all(dir).unwrap();

        let edges_path = dir.join("edges.bin");
        let offsets_path = dir.join("offsets.bin");
        let layout_path = dir.join("layout.bin");
        let db_path = dir.join("metadata.db");

        let edges_file = OpenOptions::new().read(true).write(true).create(true).open(&edges_path).unwrap();
        let offsets_file = OpenOptions::new().read(true).write(true).create(true).open(&offsets_path).unwrap();
        let layout_file = OpenOptions::new().read(true).write(true).create(true).open(&layout_path).unwrap();

        if edges_file.metadata().unwrap().len() == 0 {
            edges_file.set_len(12).unwrap();
        }
        if offsets_file.metadata().unwrap().len() == 0 {
            offsets_file.set_len(8).unwrap();
        }
        if layout_file.metadata().unwrap().len() == 0 {
            layout_file.set_len(16).unwrap();
        }

        let edges_mmap = unsafe { MmapMut::map_mut(&edges_file).unwrap() };
        let offsets_mmap = unsafe { MmapMut::map_mut(&offsets_file).unwrap() };
        let layout_mmap = unsafe { MmapMut::map_mut(&layout_file).unwrap() };

        let metadata_db = Database::create(&db_path).unwrap();
        let write_tx = metadata_db.begin_write().unwrap();
        write_tx.open_table(LABEL_TO_ID).unwrap();
        write_tx.open_table(ID_TO_LABEL).unwrap();
        write_tx.commit().unwrap();

        let node_count = (offsets_mmap.len() / 8) - 1;

        let disk_state = DiskState {
            edges_file,
            offsets_file,
            layout_file,
            edges_mmap,
            offsets_mmap,
            layout_mmap,
            write_buffer: Vec::new(),
        };

        return DiskGraph {
            node_count: if node_count == 0 { 0 } else { node_count },
            max_buffer_edges,
            base_dir: dir.to_path_buf(),
            metadata_db,
            disk_state: Rc::new(RefCell::new(disk_state)),
        };
    }

    pub fn set_label(&self, id: usize, label: &str) {
        let write_tx = self.metadata_db.begin_write().unwrap();
        {
            let mut lti = write_tx.open_table(LABEL_TO_ID).unwrap();
            lti.insert(label, id as u64).unwrap();
            let mut itl = write_tx.open_table(ID_TO_LABEL).unwrap();
            itl.insert(id as u64, label).unwrap();
        }
        write_tx.commit().unwrap();
    }

    pub fn get_label(&self, id: usize) -> Option<String> {
        let read_tx = self.metadata_db.begin_read().unwrap();
        let itl = read_tx.open_table(ID_TO_LABEL).unwrap();
        return itl.get(id as u64).unwrap().map(|v| v.value().to_string());
    }

    pub fn get_id(&self, label: &str) -> Option<usize> {
        let read_tx = self.metadata_db.begin_read().unwrap();
        let lti = read_tx.open_table(LABEL_TO_ID).unwrap();
        return lti.get(label).unwrap().map(|v| v.value() as usize);
    }

    pub fn set_layout(&self, id: usize, coordinate: [f64; 2]) {
        let mut state = self.disk_state.borrow_mut();
        let offset = id * 16;
        if state.layout_mmap.len() <= offset + 16 {
            let new_len = (offset + 16).max(state.layout_mmap.len() * 2);
            state.layout_file.set_len(new_len as u64).unwrap();
            state.layout_mmap = unsafe { MmapMut::map_mut(&state.layout_file).unwrap() };
        }
        let bytes = unsafe { std::slice::from_raw_parts(coordinate.as_ptr() as *const u8, 16) };
        state.layout_mmap[offset..offset + 16].copy_from_slice(bytes);
    }

    pub fn get_layout(&self, id: usize) -> Option<[f64; 2]> {
        let state = self.disk_state.borrow();
        let offset = id * 16;
        if offset + 16 > state.layout_mmap.len() {
            return None;
        }
        let mut result = [0.0; 2];
        let bytes = &state.layout_mmap[offset..offset + 16];
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), result.as_mut_ptr() as *mut u8, 16);
        }
        return Some(result);
    }

    pub fn flush_to_disk(&self) {
        let mut state = self.disk_state.borrow_mut();
        if state.write_buffer.is_empty() {
            return;
        }

        let new_edges = std::mem::take(&mut state.write_buffer);

        let capacity = state.edges_mmap.len() / 12;
        let disk_edges = unsafe {
            std::slice::from_raw_parts_mut(
                state.edges_mmap.as_mut_ptr() as *mut Edge,
                capacity,
            )
        };

        let mut valid_count = 0;
        for i in 0..capacity {
            let edge = disk_edges[i];
            if edge.from != 0 || edge.to != 0 || edge.weight != 0.0 {
                if i != valid_count {
                    disk_edges[valid_count] = edge;
                }
                valid_count += 1;
            }
        }

        let required_total_edges = valid_count + new_edges.len();
        let required_bytes = required_total_edges * 12;

        if state.edges_mmap.len() < required_bytes {
            state.edges_file.set_len(required_bytes as u64).unwrap();
            state.edges_mmap = unsafe { MmapMut::map_mut(&state.edges_file).unwrap() };
        }

        let active_disk_edges = unsafe {
            std::slice::from_raw_parts_mut(
                state.edges_mmap.as_mut_ptr() as *mut Edge,
                required_total_edges,
            )
        };

        for (i, edge) in new_edges.iter().enumerate() {
            active_disk_edges[valid_count + i] = *edge;
        }

        active_disk_edges.sort_unstable_by_key(|e| (e.from, e.to_index()));

        let required_nodes = active_disk_edges
            .last()
            .map(|e| e.from.max(e.to_index()) as usize + 1)
            .unwrap_or(self.node_count);

        let offsets_len = (required_nodes + 2) * 8;

        if state.offsets_mmap.len() < offsets_len {
            let new_len = offsets_len.max(state.offsets_mmap.len() * 2).max(1024);
            state.offsets_file.set_len(new_len as u64).unwrap();
            state.offsets_mmap = unsafe { MmapMut::map_mut(&state.offsets_file).unwrap() };
        }

        let mut current_node = 0;
        let mut byte_offset: u64 = 0;

        for edge in active_disk_edges.iter() {
            while current_node < edge.from as usize {
                let offset_pos = current_node * 8;
                let bytes = byte_offset.to_le_bytes();
                state.offsets_mmap[offset_pos..offset_pos + 8].copy_from_slice(&bytes);
                current_node += 1;
            }
            byte_offset += 12;
        }

        while current_node <= required_nodes + 1 {
            let offset_pos = current_node * 8;
            let bytes = byte_offset.to_le_bytes();
            state.offsets_mmap[offset_pos..offset_pos + 8].copy_from_slice(&bytes);
            current_node += 1;
        }

        state.edges_mmap.flush().unwrap();
        state.offsets_mmap.flush().unwrap();
    }
}

pub struct DiskGraphEdgeIterator {
    state: Rc<RefCell<DiskState>>,
    current_index: usize,
    max_index: usize,
}

impl Iterator for DiskGraphEdgeIterator {
    type Item = (usize, usize, f32, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.max_index {
            return None;
        }
        let offset = self.current_index * 12;
        let mut edge = Edge { from: 0, to: 0, weight: 0.0 };
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.state.borrow().edges_mmap[offset..offset + 12].as_ptr(),
                &mut edge as *mut Edge as *mut u8,
                12,
            );
        }
        self.current_index += 1;

        if edge.from == 0 && edge.to == 0 && edge.weight == 0.0 {
            return self.next();
        }

        let is_directed = edge.is_directed();
        let to_idx = edge.to_index();

        if !is_directed && edge.from > to_idx {
            return self.next();
        }

        return Some((edge.from as usize, to_idx as usize, edge.weight, is_directed));
    }
}

impl IGraphStructure for DiskGraph {
    fn add_node(&mut self) -> usize {
        let index = self.node_count;
        self.node_count += 1;

        let mut state = self.disk_state.borrow_mut();
        let offsets_len = (self.node_count + 2) * 8;
        if state.offsets_mmap.len() < offsets_len {
            let new_len = offsets_len.max(state.offsets_mmap.len() * 2).max(1024);
            state.offsets_file.set_len(new_len as u64).unwrap();
            state.offsets_mmap = unsafe { MmapMut::map_mut(&state.offsets_file).unwrap() };
        }

        return index;
    }

    fn create_connection(&mut self, from_index: usize, to_index: usize, weight: f32, directed: Option<bool>) {
        let len = {
            let mut state = self.disk_state.borrow_mut();
            let is_directed = directed.unwrap_or(false);
            state.write_buffer.push(Edge::new(from_index as u32, to_index as u32, weight, is_directed));
            if !is_directed && from_index != to_index {
                state.write_buffer.push(Edge::new(to_index as u32, from_index as u32, weight, false));
            }
            state.write_buffer.len()
        };
        if len >= self.max_buffer_edges {
            self.flush_to_disk();
        }
    }

    fn remove_connection(&mut self, from_index: usize, to_index: usize) {
        self.flush_to_disk();
        let mut state = self.disk_state.borrow_mut();

        let mut current_edges = Vec::new();
        let edge_bytes = &state.edges_mmap[..];
        for i in 0..(edge_bytes.len() / 12) {
            let offset = i * 12;
            let mut edge = Edge { from: 0, to: 0, weight: 0.0 };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    edge_bytes[offset..offset + 12].as_ptr(),
                    &mut edge as *mut Edge as *mut u8,
                    12,
                );
            }
            if edge.from == 0 && edge.to == 0 && edge.weight == 0.0 {
                continue;
            }

            if edge.from as usize == from_index && edge.to_index() as usize == to_index {
                continue;
            }
            if edge.from as usize == to_index && edge.to_index() as usize == from_index && !edge.is_directed() {
                continue;
            }
            current_edges.push(edge);
        }

        let required_edge_bytes = current_edges.len() * 12;
        if state.edges_mmap.len() > required_edge_bytes {
            state.edges_mmap[required_edge_bytes..].fill(0);
        }

        for (i, edge) in current_edges.iter().enumerate() {
            let offset = i * 12;
            let bytes = unsafe { std::slice::from_raw_parts(edge as *const Edge as *const u8, 12) };
            state.edges_mmap[offset..offset + 12].copy_from_slice(bytes);
        }
        state.edges_mmap.flush().unwrap();
    }

    fn has_connection(&self, source_index: usize, target_index: usize) -> bool {
        self.flush_to_disk();
        let state = self.disk_state.borrow();

        let start_pos = source_index * 8;
        if start_pos + 16 > state.offsets_mmap.len() {
            return false;
        }

        let mut start_offset_bytes = [0u8; 8];
        let mut end_offset_bytes = [0u8; 8];
        start_offset_bytes.copy_from_slice(&state.offsets_mmap[start_pos..start_pos + 8]);
        end_offset_bytes.copy_from_slice(&state.offsets_mmap[start_pos + 8..start_pos + 16]);

        let start_byte = u64::from_le_bytes(start_offset_bytes) as usize;
        let end_byte = u64::from_le_bytes(end_offset_bytes) as usize;

        if start_byte >= state.edges_mmap.len() || end_byte > state.edges_mmap.len() {
            return false;
        }

        let mut i = start_byte;
        while i < end_byte {
            let mut edge = Edge { from: 0, to: 0, weight: 0.0 };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    state.edges_mmap[i..i + 12].as_ptr(),
                    &mut edge as *mut Edge as *mut u8,
                    12,
                );
            }
            if edge.to_index() as usize == target_index {
                return true;
            }
            i += 12;
        }

        false
    }

    fn node_count(&self) -> usize {
        return self.node_count;
    }

    fn get_neighbors_ids(&self, node_index: usize) -> Vec<usize> {
        self.flush_to_disk();
        let state = self.disk_state.borrow();

        let mut ids = Vec::new();
        let edges_len = state.edges_mmap.len();

        let mut i = 0;
        while i + 12 <= edges_len {
            let mut edge = Edge { from: 0, to: 0, weight: 0.0 };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    state.edges_mmap[i..i + 12].as_ptr(),
                    &mut edge as *mut Edge as *mut u8,
                    12,
                );
            }
            if edge.from == 0 && edge.to == 0 && edge.weight == 0.0 {
            } else {
                if edge.from as usize == node_index {
                    ids.push(edge.to_index() as usize);
                }
                if edge.to_index() as usize == node_index {
                    ids.push(edge.from as usize);
                }
            }
            i += 12;
        }

        ids.sort();
        ids.dedup();
        return ids;
    }

    fn batch_add_nodes(&mut self, additional_count: usize) {
        self.node_count += additional_count;
        let mut state = self.disk_state.borrow_mut();
        let offsets_len = (self.node_count + 2) * 8;
        if state.offsets_mmap.len() < offsets_len {
            state.offsets_file.set_len(offsets_len as u64).unwrap();
            state.offsets_mmap = unsafe { MmapMut::map_mut(&state.offsets_file).unwrap() };
        }
    }

    fn batch_create_connections(&mut self, connections: &[(usize, usize, f32, bool)]) {
        let mut state = self.disk_state.borrow_mut();
        for &(from_index, to_index, weight, directed) in connections {
            state.write_buffer.push(Edge::new(from_index as u32, to_index as u32, weight, directed));
            if !directed && from_index != to_index {
                state.write_buffer.push(Edge::new(to_index as u32, from_index as u32, weight, false));
            }
        }
    }

    fn get_all_edges(&self) -> impl Iterator<Item = (usize, usize, f32, bool)> + '_ {
        self.flush_to_disk();
        let max_index = self.disk_state.borrow().edges_mmap.len() / 12;
        return DiskGraphEdgeIterator {
            state: self.disk_state.clone(),
            current_index: 0,
            max_index,
        };
    }

    fn resolve_label(&self, id: usize) -> Option<String> {
        return self.get_label(id);
    }

    fn manages_labels(&self) -> bool { true }

    fn set_node_label(&mut self, id: usize, label: &str) {
        self.set_label(id, label);
    }

    fn get_id_by_label(&self, label: &str) -> Option<usize> {
        self.get_id(label)
    }

    fn is_disk_based(&self) -> bool { return true; }

    fn manages_positions(&self) -> bool { true }

    fn set_node_position(&mut self, id: usize, x: f64, y: f64) {
        self.set_layout(id, [x, y]);
    }

    fn get_node_position(&self, id: usize) -> Option<(f64, f64)> {
        self.get_layout(id).map(|[x, y]| (x, y))
    }
}

impl Drop for DiskGraph {
    fn drop(&mut self) {
        self.flush_to_disk();
    }
}

impl Default for DiskGraph {
    fn default() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let c = COUNTER.fetch_add(1, Ordering::SeqCst);
        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("netfog_disk_{}_{}", t, c));
        return DiskGraph::new(path, 512);
    }
}
