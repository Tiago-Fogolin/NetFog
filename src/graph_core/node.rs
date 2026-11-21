use std::rc::Rc;

#[derive(Clone)]
pub struct _Node {
    pub label: Rc<str>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub index: Option<usize>
}
