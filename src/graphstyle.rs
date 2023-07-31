use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[derive(Debug, Clone)]
pub struct GraphStyle {
    pub nodes: Vec<NodeStyle>,
    pub edges: HashMap<(u32, u32), EdgeStyle>,
}

#[derive(Clone, Debug)]
pub struct NodeStyle {
    pub color: String,
    pub radius: f64,
    pub value: String,
    pub width: f64,
}

#[derive(Clone, Debug)]
pub struct EdgeStyle {
    pub color: String,
    pub width: f64,
    pub value: String,
}

impl GraphStyle {
    pub fn new(node_count: usize) -> Self {
        Self {
            nodes: vec![NodeStyle::default(); node_count],
            edges: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.nodes = vec![NodeStyle::default(); self.nodes.len()];
        let new_edges = self
            .edges
            .keys()
            .map(|key| (*key, EdgeStyle::default()))
            .collect();
        self.edges = new_edges;
    }

    pub fn add_edge(&mut self, mut node1: usize, mut node2: usize) {
        if node1 > node2 {
            std::mem::swap(&mut node1, &mut node2);
        }

        self.edges
            .insert((node1 as u32, node2 as u32), EdgeStyle::default());
    }

    pub fn set_edge_style(&mut self, mut node1: usize, mut node2: usize, style: EdgeStyle) {
        if node1 > node2 {
            std::mem::swap(&mut node1, &mut node2);
        }

        let ok = self.edges.insert((node1 as u32, node2 as u32), style);
        if ok.is_none() {
            // console_log!("Edge ({}, {}) not found", node1, node2);
        }
    }

    pub fn set_node_style(&mut self, node: usize, style: NodeStyle) {
        self.nodes[node] = style;
    }
}

impl NodeStyle {
    pub fn set_style(&self, context: &CanvasRenderingContext2d) {
        context.set_stroke_style(&JsValue::from_str(&self.color));
        // Multiply by 2 because the line width is centered on the path
        context.set_line_width(self.width * 2.);
    }
}

impl EdgeStyle {
    pub fn set_style(&self, context: &CanvasRenderingContext2d) {
        context.set_stroke_style(&JsValue::from_str(&self.color));
        context.set_line_width(self.width);
    }
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            color: String::from("black"),
            radius: 15.,
            value: String::from(""),
            width: 3.,
        }
    }
}

impl Default for EdgeStyle {
    fn default() -> Self {
        Self {
            color: String::from("black"),
            width: 1.,
            value: String::from(""),
        }
    }
}
