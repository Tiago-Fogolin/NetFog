use pyo3::prelude::*;


#[pyclass(get_all, set_all)]
#[derive(Clone, PartialEq)]
pub struct GraphStyle {
    pub node_color: String,
    pub node_border: String,
    pub node_radius: i32,

    pub marker_svg: String,
    pub marker_fill: String,
    pub marker_width: i32,
    pub marker_height: i32,

    pub line_color: String,
    pub line_min_width: i32,
    pub line_max_width: i32,
    pub dynamic_line_size: bool
}

impl GraphStyle {
    pub fn default() -> Self {
        return GraphStyle {
            node_color: "blue".to_string(),
            node_border: "blue".to_string(),
            node_radius: 20,
            marker_svg: "M0,0 L0,6 L9,3 z".to_string(),
            marker_fill: "black".to_string(),
            marker_width: 30,
            marker_height: 30,
            line_color: "black".to_string(),
            line_min_width: 1,
            line_max_width: 5,
            dynamic_line_size: true
        };
    }
}

#[pymethods]
impl GraphStyle {
    #[new]
    #[pyo3(signature = (
        node_color=None,
        node_border=None,
        node_radius=None,
        marker_svg=None,
        marker_fill=None,
        marker_width=None,
        marker_height=None,
        line_color=None,
        line_min_width=None,
        line_max_width=None,
        dynamic_line_size=None
    ))]
    fn new(
        node_color: Option<String>,
        node_border: Option<String>,
        node_radius: Option<i32>,
        marker_svg: Option<String>,
        marker_fill: Option<String>,
        marker_width: Option<i32>,
        marker_height: Option<i32>,
        line_color: Option<String>,
        line_min_width: Option<i32>,
        line_max_width: Option<i32>,
        dynamic_line_size: Option<bool>,
    ) -> Self {
        let d = Self::default();
        GraphStyle {
            node_color: node_color.unwrap_or(d.node_color),
            node_border: node_border.unwrap_or(d.node_border),
            node_radius: node_radius.unwrap_or(d.node_radius),
            marker_svg: marker_svg.unwrap_or(d.marker_svg),
            marker_fill: marker_fill.unwrap_or(d.marker_fill),
            marker_width: marker_width.unwrap_or(d.marker_width),
            marker_height: marker_height.unwrap_or(d.marker_height),
            line_color: line_color.unwrap_or(d.line_color),
            line_min_width: line_min_width.unwrap_or(d.line_min_width),
            line_max_width: line_max_width.unwrap_or(d.line_max_width),
            dynamic_line_size: dynamic_line_size.unwrap_or(d.dynamic_line_size),
        }
    }
}

pub fn get_line_width(
    weight: f32,
    min_weight: f32,
    max_weight: f32,
    min_width: f32,
    max_width: f32

) -> f64 {
    if max_weight == min_weight {
        return weight as f64
    }

    let normalized_weight = (weight - min_weight) / (max_weight - min_weight);

    let result = min_width + normalized_weight * (max_width - min_width);

    return result as f64;
}
