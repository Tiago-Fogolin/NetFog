use crate::_Node;
use crate::graph_core::graph::{ConnectionsList, ConnectionProperty};
use crate::layout::layout::{Layout, get_layout_function};
use crate::layout::style::{GraphStyle, get_line_width};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::{self, format};

const LABEL_OFFSET: f64 = 35.;


fn create_node_hashmap(nodes: &Vec<Rc<RefCell<_Node>>>) -> HashMap<String, Rc<RefCell<_Node>>> {
    let mut node_map: HashMap<String, Rc<RefCell<_Node>>> = HashMap::new();

    for n in nodes {
        let node_handle = Rc::clone(n);
        let label = node_handle.borrow().label.clone();


        node_map.insert(label, node_handle);
    }


    return node_map;
}

enum AtributeValue {
    Text(String),
    Decimal(f64),
    Integer(i32),
    Boolean(bool)
}

impl fmt::Display for AtributeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AtributeValue::Text(val) => write!(f, "{}", val),
            AtributeValue::Decimal(val) => write!(f, "{}", val),
            AtributeValue::Integer(val) => write!(f, "{}", val),
            AtributeValue::Boolean(val) => write!(f, "{}", val),
        }
    }
}

struct ElementPostion {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64
}

struct Element {
    name: String,
    atributes: Vec<(String, AtributeValue)>,
    children: Vec<Element>,
    simple_text: bool
}

impl Element {
    pub fn new(name: String) -> Self {
        return Element {
            name: name,
            atributes: Vec::new(),
            children: Vec::new(),
            simple_text: false
        };
    }
}

pub struct Svg {
    elements: Vec<Element>
}

impl Svg {
    pub fn new() -> Self {
        return Svg {
            elements: Vec::new()
        };
    }
    fn add_arrow_def(&mut self, marker_svg: String, marker_color: String, marker_width: i32, marker_height: i32) {
        let mut arrow_head = Element::new("path".to_string());
        let arrow_head_atributes = vec![
            ("d".to_string(), AtributeValue::Text(marker_svg)),
            ("fill".to_string(), AtributeValue::Text(marker_color))
        ];

        arrow_head.atributes = arrow_head_atributes;

        let mut arrow_marker = Element::new("marker".to_string());
        let arrow_marker_atributes = vec![
            ("id".to_string(), AtributeValue::Text("marker".to_string())),
            ("markerWidth".to_string(), AtributeValue::Text(marker_width.to_string())),
            ("markerHeight".to_string(), AtributeValue::Text(marker_height.to_string())),
            ("refX".to_string(), AtributeValue::Text("29".to_string())),
            ("refY".to_string(), AtributeValue::Text("3".to_string())),
            ("orient".to_string(), AtributeValue::Text("auto".to_string())),
            ("markerUnits".to_string(), AtributeValue::Text("strokeWidth".to_string())),
        ];
        arrow_marker.atributes = arrow_marker_atributes;
        arrow_marker.children.push(arrow_head);

        let mut defs = Element::new("defs".to_string());
        defs.children.push(arrow_marker);

        self.elements.push(defs);

    }

    fn get_marker_only_line(
        &mut self,
        element_pos: ElementPostion,
        element_class: &String,
        stroke_width: f64
    ) -> Element {
        let mut marker_only_line = Element::new("line".to_string());

        let atributes: Vec<(String, AtributeValue)> = vec![
          ("x1".to_string(), AtributeValue::Decimal(element_pos.x1)),
          ("y1".to_string(), AtributeValue::Decimal(element_pos.y1)),
          ("x2".to_string(), AtributeValue::Decimal(element_pos.x2)),
          ("y2".to_string(), AtributeValue::Decimal(element_pos.y2)),
          ("stroke".to_string(), AtributeValue::Text("none".to_string())),
          ("stroke-width".to_string(), AtributeValue::Decimal(stroke_width)),
          ("marker-end".to_string(), AtributeValue::Text("url(#marker)".to_string())),
          ("class".to_string(), AtributeValue::Text(element_class.clone()))
        ];

        marker_only_line.atributes = atributes;

        return marker_only_line;
    }

    fn add_line(
        &mut self,
        element_pos: ElementPostion,
        from_index: usize,
        to_index: usize,
        directed: bool,
        line_color: &str,
        line_width: f64

    ) -> Option<Element> {
        let mut new_line = Element::new("line".to_string());
        let mut marker_ony_element: Option<Element> = None;
        let stroke_width: f64 = 1.;
        let class_name = format!("{}line{}", from_index, to_index);

        let atributes: Vec<(String, AtributeValue)> = vec![
          ("x1".to_string(), AtributeValue::Decimal(element_pos.x1)),
          ("y1".to_string(), AtributeValue::Decimal(element_pos.y1)),
          ("x2".to_string(), AtributeValue::Decimal(element_pos.x2)),
          ("y2".to_string(), AtributeValue::Decimal(element_pos.y2)),
          ("stroke".to_string(), AtributeValue::Text(line_color.to_string())),
          ("stroke-width".to_string(), AtributeValue::Decimal(line_width)),
          ("class".to_string(), AtributeValue::Text(class_name.clone()))
        ];

        new_line.atributes = atributes;

        self.elements.push(new_line);

        if directed {
            marker_ony_element = Some(self.get_marker_only_line(element_pos, &class_name, stroke_width));
        }

        return marker_ony_element;
    }

    fn add_circle(
        &mut self,
        node: &_Node,
        node_color: &str,
        node_border: &str,
        node_radius: i32
    ) {
        let mut new_circle = Element::new("circle".to_string());

        let atributes: Vec<(String, AtributeValue)> = vec![
            ("cx".to_string(), AtributeValue::Decimal(node.x.unwrap())),
            ("cy".to_string(), AtributeValue::Decimal(node.y.unwrap())),
            ("stroke".to_string(), AtributeValue::Text(node_border.to_string())),
            ("fill".to_string(), AtributeValue::Text(node_color.to_string())),
            ("r".to_string(), AtributeValue::Integer(node_radius)),
            ("class".to_string(), AtributeValue::Text(format!("node{}",node.index.unwrap())))
        ];

        new_circle.atributes = atributes;

        self.elements.push(new_circle);
    }

    fn add_label(
        &mut self,
        node: &_Node
    ) {
        let mut new_label = Element::new("text".to_string());

        let atributes: Vec<(String, AtributeValue)> = vec![
            ("text-anchor".to_string(), AtributeValue::Text("middle".to_string())),
            ("x".to_string(), AtributeValue::Decimal(node.x.unwrap())),
            ("y".to_string(), AtributeValue::Decimal(node.y.unwrap() + LABEL_OFFSET)),
            ("class".to_string(), AtributeValue::Text(format!("label{}",node.index.unwrap()))),
        ];

        let mut simple_text = Element::new(node.label.clone());
        simple_text.simple_text = true;


        new_label.atributes = atributes;
        new_label.children.push(simple_text);

        self.elements.push(new_label);
    }

    fn draw_nodes(
        &mut self,
        nodes: &Vec<Rc<RefCell<_Node>>>,
        node_color: &str,
        node_border: &str,
        node_radius: i32
    ) {
        for n in nodes {
            let node = n.borrow();
            self.add_circle(&node, node_color, node_border, node_radius);
            self.add_label(&node);
        }
    }

    fn draw_lines(
        &mut self,
        connections: &ConnectionsList,
        node_map: HashMap<String, Rc<RefCell<_Node>>>,
        line_color: &str,
        min_weight: f32,
        max_weight: f32,
        min_width: i32,
        max_width: i32,
        dynamic_lines: bool
    ) {
        let mut marker_only_lines: Vec<Element> = Vec::new();

        for conn in connections {
            let mut from_name = None;
            let mut to_name = None;
            let mut directed = false;
            let mut weight = 0.;

            for property in conn.values() {
                match property {
                    ConnectionProperty::From(name) => from_name = Some(name),
                    ConnectionProperty::To(name) => to_name = Some(name),
                    ConnectionProperty::Directed(bool) => directed = *bool,
                    ConnectionProperty::Weight(w) => weight = *w,
                    _ => {}
                }
            }
            let from = node_map[from_name.unwrap()].borrow();
            let to = node_map[to_name.unwrap()].borrow();

            let line_pos = ElementPostion {
                x1: from.x.unwrap(),
                y1: from.y.unwrap(),
                x2: to.x.unwrap(),
                y2: to.y.unwrap()
            };
            let mut line_width = 1.;
            if dynamic_lines {
                line_width = get_line_width(weight, min_weight, max_weight, min_width as f32, max_width as f32);
            }

            let marker_line = self.add_line(line_pos, from.index.unwrap(), to.index.unwrap(), directed, line_color, line_width);
            if !marker_line.is_none() {
                marker_only_lines.push(marker_line.unwrap());
            }
        }

        if !marker_only_lines.is_empty() {
            for el in marker_only_lines {
                self.elements.push(el);
            }
        }

    }

    fn draw_graph(
        &mut self,
        nodes: &Vec<Rc<RefCell<_Node>>>,
        connections: &ConnectionsList,
        layout: Layout,
        positions_set: bool,
        override_positions: bool,
        node_map: HashMap<String, Rc<RefCell<_Node>>>,
        style: GraphStyle
    ) {
        if !positions_set || override_positions {
            let layout_func = get_layout_function(layout);
            layout_func(&nodes);
        }

        let min_weight = 1.;
        let max_weight = connections.iter()
            .filter_map(|conn| conn.get("Weight"))
            .filter_map(|prop| if let ConnectionProperty::Weight(w) = prop { Some(w) } else { None })
            .max_by(|a, b| a.total_cmp(b))
            .copied()
            .unwrap_or(1.0);

        self.add_arrow_def(style.marker_svg, style.marker_fill, style.marker_width, style.marker_height);
        self.draw_lines(connections, node_map, &style.line_color, min_weight, max_weight, style.line_min_width, style.line_max_width, style.dynamic_line_size);
        self.draw_nodes(nodes, &style.node_color, &style.node_border, style.node_radius);

    }

    fn write_element(&self, element: &Element) -> String {
        if element.simple_text {
            return element.name.clone();
        }

        let mut current_element = "<".to_string();
        current_element += &element.name;

        for atribute in &element.atributes {
            current_element += " ";
            current_element += &atribute.0;
            current_element += "=";
            current_element += "\"";
            current_element += &atribute.1.to_string();
            current_element += "\"";
        }

        if element.children.is_empty() {
            current_element += "/>";
            return current_element;
        }

        current_element += ">";

        for child in &element.children {
            current_element += &self.write_element(&child);
        }

        current_element += "</";
        current_element += &element.name;
        current_element += ">";

        return current_element;
    }

    fn write_svg(&self) -> String {
        let mut final_svg: String = String::new();

        final_svg += "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"20 20 1480 680\" width=\"100%\" height=\"100%\">";

        for element in &self.elements {
            final_svg += &self.write_element(&element);
        }

        final_svg += "</svg>";

        return final_svg;
    }

    pub fn get_svg(
        &mut self,
        nodes: &Vec<Rc<RefCell<_Node>>>,
        connections: &ConnectionsList,
        layout: Layout,
        positions_set: bool,
        override_positions: bool,
        style: GraphStyle
    ) -> String {
        // TODO -> Remove this workaround
        // For now, we store the x,y by label
        // But in future this will be changed, so we get a better performance
        let node_map = create_node_hashmap(nodes);

        self.draw_graph(nodes, connections, layout, positions_set, override_positions, node_map, style);

        let svg = self.write_svg();

        return svg;
    }

}
