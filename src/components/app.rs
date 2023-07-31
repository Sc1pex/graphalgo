use super::{Canvas, Input};
use crate::{algs::Dfs, graph::Graph};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // let (graph, set_graph) = create_signal(cx, default_graph());
    let (dfs, set_dfs) = create_signal(cx, Dfs::new(default_graph()));
    provide_context(cx, (dfs, set_dfs));

    let step = move |_| {
        set_dfs.update(|dfs| {
            log!("Updating");
            dfs.step();
        });
    };

    view! {cx,
    <div class="flex justify-center items-center flex-col gap-4">
        <p>Graph Algorithm Visualizer</p>
        <div class="flex gap-2">
            <div>
                { Dfs::input(cx) }
                <button on:click=step class="mt-8 border p-2 rounded">"Next step"</button>
            </div>
            <Canvas />
            { Dfs::output(cx) }
        </div>
        <div class="flex gap-4">
        </div>
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
