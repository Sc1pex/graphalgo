use leptos::{html::Textarea, *};

use crate::{
    algs::{Algorithm, Dfs},
    graph::Graph,
};

#[component]
pub fn Input(cx: Scope) -> impl IntoView {
    let graph_input_ref = create_node_ref::<Textarea>(cx);
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

    let update_graph = move |_| {
        let input = graph_input_ref.get().unwrap().value();

        let new_graph: Graph = input.into();
        set_dfs.update(|g| {
            *g = Box::new(Dfs::new(new_graph));
        })
    };

    view! {cx,
    <div class="flex items-center gap-4">
        <div class="flex flex-col items-center">
            <p>{"Graph input"}</p>
            <button class="border rounded p-2" on:click=update_graph>
                "Update graph"
            </button>
        </div>
        <textarea _ref=graph_input_ref rows="7" cols="20" class="border border-black font-mono p-2 bg-slate-600" />
    </div>
    }
}
