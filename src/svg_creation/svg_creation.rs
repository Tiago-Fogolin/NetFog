use pyo3::ffi::Py_Finalize;

use crate::_Node;
use crate::graph_core::graph::ConnectionsList;
use crate::layout::layout::{Layout, get_layout_function};
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use std::fmt;


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
    children: Vec<Element>
}

impl Element {
    pub fn new(name: String) -> Self {
        return Element {
            name: name,
            atributes: Vec::new(),
            children: Vec::new()
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
    fn add_arrow_def(&mut self) {
        let mut arrow_head = Element::new("path".to_string());
        let arrow_head_atributes = vec![
            ("d".to_string(), AtributeValue::Text("M0,0 L0,6 L9,3 z".to_string())),
            ("fill".to_string(), AtributeValue::Text("black".to_string()))
        ];

        arrow_head.atributes = arrow_head_atributes;

        let mut arrow_marker = Element::new("marker".to_string());
        let arrow_marker_atributes = vec![
            ("id".to_string(), AtributeValue::Text("marker".to_string())),
            ("markerWidth".to_string(), AtributeValue::Text("30".to_string())),
            ("markerHeight".to_string(), AtributeValue::Text("30".to_string())),
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

    fn add_line(
        &mut self,
        element_pos: ElementPostion

    ) {
        let mut new_line = Element::new("line".to_string());

        let atributes: Vec<(String, AtributeValue)> = vec![
          ("x1".to_string(), AtributeValue::Decimal(element_pos.x1)),
          ("y1".to_string(), AtributeValue::Decimal(element_pos.y1)),
          ("x2".to_string(), AtributeValue::Decimal(element_pos.x2)),
          ("y2".to_string(), AtributeValue::Decimal(element_pos.y2)),
          ("stroke".to_string(), AtributeValue::Text("blue".to_string())),
          ("stroke_width".to_string(), AtributeValue::Decimal(1.)),
        ];

        new_line.atributes = atributes;

        self.elements.push(new_line);
    }

    fn add_circle(
        &mut self,
        node: &_Node
    ) {
        let mut new_circle = Element::new("circle".to_string());

        let atributes: Vec<(String, AtributeValue)> = vec![
            ("cx".to_string(), AtributeValue::Decimal(node.x.unwrap())),
            ("cy".to_string(), AtributeValue::Decimal(node.y.unwrap())),
            ("stroke".to_string(), AtributeValue::Text("blue".to_string())),
            ("fill".to_string(), AtributeValue::Text("blue".to_string())),
            ("r".to_string(), AtributeValue::Integer(20)),
        ];

        new_circle.atributes = atributes;

        self.elements.push(new_circle);
    }

    fn draw_nodes(
        &mut self,
        nodes: &Vec<Rc<RefCell<_Node>>>
    ) {
        for n in nodes {
            let node = n.borrow();
            self.add_circle(&node);
        }
    }

    fn draw_graph(
        &mut self,
        nodes: &Vec<Rc<RefCell<_Node>>>,
        connections: ConnectionsList,
        layout: Layout,
        positions_set: bool,
        override_positions: bool
    ) {
        if !positions_set || override_positions {
            let layout_func = get_layout_function(layout);
            layout_func(&nodes);
        }

        // self.add_arrow_def();
        self.draw_nodes(nodes);

    }

    fn write_element(&self, element: &Element) -> String {
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

        for child in &element.children {
            self.write_element(&child);
        }

        current_element += "/>";

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
        connections: ConnectionsList,
        layout: Layout,
        positions_set: bool,
        override_positions: bool
    ) -> String {
        self.draw_graph(nodes, connections, layout, positions_set, override_positions);

        let svg = self.write_svg();

        return svg;
    }

}
