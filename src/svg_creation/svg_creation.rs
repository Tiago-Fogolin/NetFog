use pyo3::exceptions::PyAttributeError;
use pyo3::ffi::Py_Finalize;

use crate::_Node;
use crate::graph_core::graph::{ConnectionsList, ConnectionProperty};
use crate::graph_core::node;
use crate::layout::layout::{Layout, get_layout_function};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
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
        element_pos: ElementPostion,
        from_index: usize,
        to_index: usize

    ) {
        let mut new_line = Element::new("line".to_string());

        let atributes: Vec<(String, AtributeValue)> = vec![
          ("x1".to_string(), AtributeValue::Decimal(element_pos.x1)),
          ("y1".to_string(), AtributeValue::Decimal(element_pos.y1)),
          ("x2".to_string(), AtributeValue::Decimal(element_pos.x2)),
          ("y2".to_string(), AtributeValue::Decimal(element_pos.y2)),
          ("stroke".to_string(), AtributeValue::Text("black".to_string())),
          ("stroke_width".to_string(), AtributeValue::Decimal(1.)),
          ("class".to_string(), AtributeValue::Text(format!("{}line{}", from_index, to_index)))
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
        nodes: &Vec<Rc<RefCell<_Node>>>
    ) {
        for n in nodes {
            let node = n.borrow();
            self.add_circle(&node);
            self.add_label(&node);
        }
    }

    fn draw_lines(
        &mut self,
        connections: &ConnectionsList,
        node_map: HashMap<String, Rc<RefCell<_Node>>>
    ) {
        for conn in connections {
            let mut from_name = None;
            let mut to_name = None;

            for property in conn.values() {
                match property {
                    ConnectionProperty::From(name) => from_name = Some(name),
                    ConnectionProperty::To(name) => to_name = Some(name),
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

            self.add_line(line_pos, from.index.unwrap(), to.index.unwrap());
        }

    }

    fn draw_graph(
        &mut self,
        nodes: &Vec<Rc<RefCell<_Node>>>,
        connections: &ConnectionsList,
        layout: Layout,
        positions_set: bool,
        override_positions: bool,
        node_map: HashMap<String, Rc<RefCell<_Node>>>
    ) {
        if !positions_set || override_positions {
            let layout_func = get_layout_function(layout);
            layout_func(&nodes);
        }

        self.add_arrow_def();
        self.draw_lines(connections, node_map);
        self.draw_nodes(nodes);

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
        override_positions: bool
    ) -> String {
        // TODO -> Remove this workaround
        // For now, we store the x,y by label
        // But in future this will be changed, so we get a better performance
        let node_map = create_node_hashmap(nodes);

        self.draw_graph(nodes, connections, layout, positions_set, override_positions, node_map);

        let svg = self.write_svg();

        return svg;
    }

}
