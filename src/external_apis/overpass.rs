use crate::Graph;
use crate::external_apis::core::{OverpassResponse, NominatimResponse};
use reqwest::blocking::Client;
use std::error::Error;
use std::collections::HashMap;
use std::f64;
use crate::graph_core::graph_structure_interface::IGraphStructure;
use crate::graph_core::graph::_Graph;
use crate::layout::layout::BoundingBox;

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
