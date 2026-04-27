use crate::Graph;
use crate::external_apis::core::{OverpassResponse, NominatimResponse};
use reqwest::blocking::Client;
use std::error::Error;
use std::collections::HashMap;
use std::f64;
use crate::graph_core::graph_structure_interface::IGraphStructure;
use crate::graph_core::graph::_Graph;
use crate::graph_core::disk_graph::DiskGraph;
use crate::layout::layout::BoundingBox;
use redb::{Database, TableDefinition, ReadableTable};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Staging tables for the Overpass disk-based pipeline.
// OVERPASS_NODES: OSM node ID → 16 raw bytes (lat f64 LE | lon f64 LE)
// OVERPASS_WAYS:  OSM way  ID → 1 byte (oneway flag) + N×8 bytes (i64 node IDs LE)
const OVERPASS_NODES: TableDefinition<u64, &[u8]> = TableDefinition::new("overpass_nodes");
const OVERPASS_WAYS: TableDefinition<u64, &[u8]> = TableDefinition::new("overpass_ways");

fn make_request_overpass(radius: f64, point: NominatimResponse) -> Result<OverpassResponse, Box<dyn Error>>  {
    let client = Client::builder()
        .user_agent("NetFog Library")
        .build()?;

    let url = "https://overpass-api.de/api/interpreter";

    let query = format!(
        "[out:json][timeout:25];\n\
        (\n  \
            way[\"highway\"](around:{}, {}, {});\n\
        );\n\
        out body;\n\
        >;\n\
        out skel qt;",
        radius, point.lat, point.lon
    );

    let response = client
        .post(url)
        .form(&[("data", &query)])
        .send()?
        .error_for_status()?
        .json::<OverpassResponse>()?;

    return Ok(response);
}


pub fn make_overpass_graph<S: IGraphStructure + Default>(radius: f64, point: NominatimResponse) -> _Graph<S> {
    let overpass_resp = make_request_overpass(radius, point).expect("Request to Overpass failed!");

    let mut osm_to_netfog: HashMap<i64, String> = HashMap::new();
    let mut current_internal_id: usize = 0;

    let mut graph: _Graph<S> = _Graph::default();

    let mut min_lat = f64::INFINITY;
    let mut max_lat = -f64::INFINITY;

    let mut min_lon = f64::INFINITY;
    let mut max_lon = -f64::INFINITY;


    for element in overpass_resp.elements.iter() {
        if element.osm_type == "node" {

            if let (Some(lat), Some(lon)) = (element.lat, element.lon) {
                max_lat = max_lat.max(lat);
                min_lat = min_lat.min(lat);

                max_lon = max_lon.max(lon);
                min_lon = min_lon.min(lon);

            }
        }
    }

    let bbox: BoundingBox = BoundingBox {
        max_lat: max_lat,
        min_lat: min_lat,
        max_lon: max_lon,
        min_lon: min_lon
    };

    for element in overpass_resp.elements.iter() {
        if element.osm_type == "node" {


            if let (Some(lat), Some(lon)) = (element.lat, element.lon) {
                let [x,y] = bbox.lat_lon_to_screen(lat, lon);
                graph.add_node_with_pos(current_internal_id.to_string(), x, y);
                osm_to_netfog.insert(element.id, current_internal_id.to_string());
                current_internal_id += 1;

            }
        }
    }

    for element in overpass_resp.elements.iter() {
        if element.osm_type == "way" {

            // ignore unrecheable roads
            if let Some(tags) = &element.tags {
                if let Some(access) = tags.get("access") {
                    if access == "private" || access == "no" {
                        continue;
                    }
                }
            }


            if let Some(nodes) = &element.nodes {

                let is_oneway = element.tags.as_ref()
                    .and_then(|t| t.get("oneway"))
                    .map(|v| v == "yes")
                    .unwrap_or(false);

                for window in nodes.windows(2) {
                    let osm_from = window[0];
                    let osm_to = window[1];

                    if let (Some(netfog_from), Some(netfog_to)) = (osm_to_netfog.get(&osm_from), osm_to_netfog.get(&osm_to)) {

                        graph.create_connection(netfog_from.clone(), netfog_to.clone(), 1.0, Some(is_oneway));

                    }
                }
            }
        }
    }

    return graph;

}

// ------------------------------------------------------------------
// Staging-area helper
// ------------------------------------------------------------------

/// Creates a uniquely-named temporary redb file and initialises both
/// Overpass staging tables. Returns the open `Database` handle and the
/// path so the caller can delete the file after loading.
fn create_overpass_staging_db() -> (Database, PathBuf) {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let c = COUNTER.fetch_add(1, Ordering::SeqCst);
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir()
        .join(format!("netfog_overpass_staging_{}_{}.redb", t, c));
    let db = Database::create(&path).unwrap();
    {
        let write_tx = db.begin_write().unwrap();
        let _ = write_tx.open_table(OVERPASS_NODES).unwrap();
        let _ = write_tx.open_table(OVERPASS_WAYS).unwrap();
        write_tx.commit().unwrap();
    }
    return (db, path);
}

// ------------------------------------------------------------------
// DiskGraph-exclusive Overpass pipeline
// ------------------------------------------------------------------

pub fn make_overpass_disk_graph(radius: f64, point: NominatimResponse) -> _Graph<DiskGraph> {
    let overpass_resp = make_request_overpass(radius, point)
        .expect("Request to Overpass failed!");



    let (staging_db, staging_path) = create_overpass_staging_db();

    let mut min_lat = f64::INFINITY;
    let mut max_lat = f64::NEG_INFINITY;
    let mut min_lon = f64::INFINITY;
    let mut max_lon = f64::NEG_INFINITY;

    {
        let write_tx = staging_db.begin_write().unwrap();
        {
            let mut nodes_table = write_tx.open_table(OVERPASS_NODES).unwrap();
            let mut ways_table = write_tx.open_table(OVERPASS_WAYS).unwrap();

            for element in overpass_resp.elements.iter() {
                if element.osm_type == "node" {
                    if let (Some(lat), Some(lon)) = (element.lat, element.lon) {
                        if lat > max_lat { max_lat = lat; }
                        if lat < min_lat { min_lat = lat; }
                        if lon > max_lon { max_lon = lon; }
                        if lon < min_lon { min_lon = lon; }

                        // Encode: 8 bytes lat LE | 8 bytes lon LE
                        let mut bytes = [0u8; 16];
                        bytes[..8].copy_from_slice(&lat.to_le_bytes());
                        bytes[8..].copy_from_slice(&lon.to_le_bytes());
                        nodes_table.insert(element.id as u64, bytes.as_ref()).unwrap();
                    }
                } else if element.osm_type == "way" {
                    // Filter private / no-access roads at ingest time.
                    if let Some(tags) = &element.tags {
                        if let Some(access) = tags.get("access") {
                            if access == "private" || access == "no" {
                                continue;
                            }
                        }
                    }

                    if let Some(nodes) = &element.nodes {
                        let is_oneway = element.tags.as_ref()
                            .and_then(|t| t.get("oneway"))
                            .map(|v| v == "yes")
                            .unwrap_or(false);

                        // Encode: 1 byte oneway flag | N × 8 bytes (i64 node IDs LE)
                        let mut bytes = vec![0u8; 1 + nodes.len() * 8];
                        bytes[0] = if is_oneway { 1 } else { 0 };
                        for (i, &node_id) in nodes.iter().enumerate() {
                            let offset = 1 + i * 8;
                            bytes[offset..offset + 8].copy_from_slice(&node_id.to_le_bytes());
                        }
                        ways_table.insert(element.id as u64, bytes.as_slice()).unwrap();
                    }
                }
            }
        }
        write_tx.commit().unwrap();
    }

    drop(overpass_resp);

    let bbox = BoundingBox { max_lat, min_lat, max_lon, min_lon };


    let mut graph: _Graph<DiskGraph> = _Graph::default();

    let mut osm_to_netfog: HashMap<i64, String> = HashMap::new();
    let mut current_internal_id: usize = 0;

    {
        let read_tx = staging_db.begin_read().unwrap();

        {
            let nodes_table = read_tx.open_table(OVERPASS_NODES).unwrap();
            for entry in nodes_table.iter().unwrap() {
                let (k, v) = entry.unwrap();
                let osm_id = k.value() as i64;
                let bytes = v.value();

                let lat = f64::from_le_bytes(bytes[..8].try_into().unwrap());
                let lon = f64::from_le_bytes(bytes[8..].try_into().unwrap());
                let [x, y] = bbox.lat_lon_to_screen(lat, lon);

                let label = current_internal_id.to_string();
                graph.add_node_with_pos(label.clone(), x, y);
                osm_to_netfog.insert(osm_id, label);
                current_internal_id += 1;
            }
        }

        // 2b – Replay ways to create edges.
        {
            let ways_table = read_tx.open_table(OVERPASS_WAYS).unwrap();
            for entry in ways_table.iter().unwrap() {
                let (_, v) = entry.unwrap();
                let bytes = v.value();

                if bytes.is_empty() {
                    continue;
                }

                let is_oneway = bytes[0] != 0;
                let node_count = (bytes.len() - 1) / 8;

                let mut node_ids: Vec<i64> = Vec::with_capacity(node_count);
                for i in 0..node_count {
                    let offset = 1 + i * 8;
                    let id = i64::from_le_bytes(
                        bytes[offset..offset + 8].try_into().unwrap(),
                    );
                    node_ids.push(id);
                }

                for window in node_ids.windows(2) {
                    let osm_from = window[0];
                    let osm_to_node = window[1];

                    if let (Some(netfog_from), Some(netfog_to)) = (
                        osm_to_netfog.get(&osm_from),
                        osm_to_netfog.get(&osm_to_node),
                    ) {
                        graph.create_connection(
                            netfog_from.clone(),
                            netfog_to.clone(),
                            1.0,
                            Some(is_oneway),
                        );
                    }
                }
            }
        }
    }


    drop(staging_db);
    let _ = std::fs::remove_file(&staging_path);

    graph.structure.flush_to_disk();

    return graph;
}
