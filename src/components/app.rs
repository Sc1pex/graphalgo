use crate::graph::Graph;

use super::{Canvas, Input};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (graph, set_graph) = create_signal(cx, default_graph());
    provide_context(cx, (graph, set_graph));

    view! {cx,
    <div class="flex justify-center items-center flex-col gap-4">
        <p>Graph Algorithm Visualizer</p>
        <Canvas />
        <Input />
    </div>
    }
}

fn default_graph() -> Graph {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph
}
