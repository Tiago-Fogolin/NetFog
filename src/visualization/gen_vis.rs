use crate::_Node;

pub fn generate_visualization(nodes: &[_Node], all_edges: impl Iterator<Item = (usize, usize, f32, bool)>) -> String {
    let mut json = String::new();
    json.push_str("{\"nodes\":[");

    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&format!(
            "{{\"id\":{},\"label\":{},\"x\":{},\"y\":{}}}",
            i,
            serde_json::to_string(&node.label).unwrap(),
            node.x.unwrap_or(0.0),
            node.y.unwrap_or(0.0)
        ));
    }

    json.push_str("],\"edges\":[");

    let mut arcs_buf: Vec<(usize, usize, f32)> = Vec::new();
    let mut first_edge = true;

    for (from, to, weight, directed) in all_edges {
        if directed {
            arcs_buf.push((from, to, weight));
        } else {
            if !first_edge {
                json.push(',');
            }
            json.push_str(&format!(
                "{{\"source\":{},\"target\":{},\"weight\":{}}}",
                from, to, weight
            ));
            first_edge = false;
        }
    }

    json.push_str("],\"arcs\":[");

    for (i, (from, to, weight)) in arcs_buf.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&format!(
            "{{\"source\":{},\"target\":{},\"weight\":{}}}",
            from, to, weight
        ));
    }

    json.push_str("]}");
    json
}
