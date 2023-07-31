use leptos::{html::Canvas, *};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

use crate::{
    algs::{Algorithm, Dfs},
    graph::Graph,
};

#[component]
pub fn Canvas(cx: Scope) -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>(cx);
    let (graph, _set_graph) = use_context::<(
        ReadSignal<Box<dyn Algorithm>>,
        WriteSignal<Box<dyn Algorithm>>,
    )>(cx)
    .unwrap();
    canvas_ref.on_load(cx, move |canvas| {
        canvas.set_width(500);
        canvas.set_height(500);

        graph.with_untracked(|g| draw_graph(&g.graph(), &canvas))
    });

    create_effect(cx, move |_| {
        if let Some(canvas) = canvas_ref.get() {
            graph.with(|g| draw_graph(&g.graph(), &canvas))
        }
    });

    view! {cx,
    <div>
        <canvas _ref=canvas_ref class="bg-slate-600 rounded w-[500px] h-[500px]" />
    </div>
    }
}

fn draw_graph(graph: &Graph, canvas: &HtmlElement<Canvas>) {
    let ctx = canvas.get_context("2d").unwrap().unwrap();
    let context = ctx.dyn_into::<CanvasRenderingContext2d>().unwrap();
    context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

    // Draw edges
    for node1 in 0..graph.node_count {
        for node2 in &graph.edges[node1] {
            let (node1, node2) = if node1 < *node2 {
                (node1, *node2)
            } else {
                (*node2, node1)
            };

            let (x1, y1) = get_node_coord(node1, graph, canvas);
            let (x2, y2) = get_node_coord(node2, graph, canvas);

            let edge_style = graph
                .style
                .edges
                .get(&(node1 as u32, node2 as u32))
                .expect(format!("Edge ({}, {}) not found", node1, node2).as_str());
            edge_style.set_style(&context);
            context.begin_path();
            context.move_to(x1, y1);
            context.line_to(x2, y2);
            context.stroke();
        }
    }

    // Draw nodes
    for node in 0..graph.node_count {
        let (x, y) = get_node_coord(node, &graph, canvas);
        let node_style = &graph.style.nodes[node];

        node_style.set_style(&context);
        context.begin_path();
        context
            .arc(x, y, node_style.radius, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        context.stroke();
        context.set_fill_style(&JsValue::from_str("white"));
        context.fill();

        context.set_fill_style(&JsValue::from_str("black"));
        context.set_font("20px sans-serif");
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.fill_text(&node.to_string(), x, y).unwrap();
    }
}

const DRAW_MARGIN: f64 = 0.85;

fn get_node_coord(node: usize, graph: &Graph, canvas: &HtmlElement<Canvas>) -> (f64, f64) {
    use std::f64::consts::PI;

    let angle_increment = 2. * PI / graph.node_count as f64;
    let angle = PI / 2. - angle_increment * node as f64;

    let x = angle.cos() * DRAW_MARGIN;
    let y = angle.sin() * DRAW_MARGIN;
    transform_coord(x, y, canvas)
}

fn transform_coord(x: f64, y: f64, canvas: &HtmlElement<Canvas>) -> (f64, f64) {
    let x = (x + 1.) / 2. * canvas.width() as f64;
    let y = (1. - y) / 2. * canvas.height() as f64;
    (x, y)
}
