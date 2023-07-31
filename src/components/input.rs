use crate::{
    algs::{Algorithm, Bfs, Dfs},
    graph::Graph,
};
use leptos::{
    html::{Select, Textarea},
    *,
};

#[component]
pub fn Input(cx: Scope) -> impl IntoView {
    let graph_input_ref = create_node_ref::<Textarea>(cx);
    let alg_input_ref = create_node_ref::<Select>(cx);
    graph_input_ref.on_load(cx, |text| {
        text.set_value(
            r#"4
0 1
1 2
1 3
2 3
"#,
        );
    });
    let (_dfs, set_dfs) = use_context::<(
        ReadSignal<Box<dyn Algorithm>>,
        WriteSignal<Box<dyn Algorithm>>,
    )>(cx)
    .unwrap();

    let update_graph = move || {
        let input = graph_input_ref.get().unwrap().value();

        let new_graph: Graph = input.into();
        set_dfs.update(|g| {
            *g = match alg_input_ref.get().unwrap().value().as_str() {
                "dfs" => Box::new(Dfs::new(new_graph)),
                "bfs" => Box::new(Bfs::new(new_graph)),
                _ => unreachable!(),
            }
        })
    };

    view! {cx,
    <div class="flex items-center gap-4">
        <div class="flex flex-col items-center">
            <p>"Graph input"</p>

            <div class="m-4">
                <p>"Algorithm"</p>
                <select _ref=alg_input_ref class="bg-slate-600 px-1 py-0.5 rounded" on:change= move |_| update_graph()>
                    <option value="dfs">"DFS"</option>
                    <option value="bfs">"BFS"</option>
                </select>
            </div>

            <button class="border rounded p-2" on:click= move |_| update_graph()>
                "Update graph"
            </button>
        </div>
        <textarea _ref=graph_input_ref rows="7" cols="20" class="border border-black font-mono p-2 bg-slate-600" />
    </div>
    }
}
