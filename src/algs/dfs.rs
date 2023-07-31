use crate::{
    graph::Graph,
    graphstyle::{EdgeStyle, NodeStyle},
};
use leptos::{html::Input, *};
use std::collections::VecDeque;

pub struct Dfs {
    visited: Vec<bool>,
    queue: VecDeque<(Option<usize>, usize)>,

    pub graph: Graph,
    output: Vec<String>,
}

impl Dfs {
    pub fn new(graph: Graph) -> Self {
        let mut dfs = Self {
            visited: vec![false; graph.node_count],
            queue: VecDeque::new(),

            graph,
            output: vec![],
        };
        dfs.start(0);
        dfs
    }

    pub fn start(&mut self, input: usize) {
        self.output = vec![];
        self.queue.clear();
        self.visited = vec![false; self.graph.node_count];
        self.graph.style.reset();

        self.queue.push_back((None, input));
    }

    pub fn step(&mut self) -> Option<()> {
        let (last, node) = self.queue.pop_back()?;
        if self.visited[node] {
            return Some(());
        }
        self.visited[node] = true;

        self.output.push(format!("Visiting {}\n", node));
        self.graph.style.set_node_style(
            node,
            NodeStyle {
                color: "green".to_owned(),
                ..Default::default()
            },
        );
        if let Some(l) = last {
            self.graph.style.set_edge_style(
                l,
                node,
                EdgeStyle {
                    color: "green".to_owned(),
                    ..Default::default()
                },
            );
        }

        for neighbor in &self.graph.edges[node] {
            self.queue.push_back((Some(node), *neighbor));
        }

        Some(())
    }

    pub fn input(cx: Scope) -> impl IntoView {
        view! { cx,
            <DfsInput />
        }
    }

    pub fn output(cx: Scope) -> impl IntoView {
        view! { cx,
            <DfsOutput />
        }
    }
}

#[component]
fn dfs_input(cx: Scope) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    let (_dfs, set_dfs) = use_context::<(ReadSignal<Dfs>, WriteSignal<Dfs>)>(cx).unwrap();

    input_ref.on_load(cx, |input| {
        input.set_value("0");
    });

    let update_input = move |_| {
        let input = input_ref.get().unwrap().value();
        let input: usize = input.parse().unwrap();

        set_dfs.update(|dfs| {
            dfs.start(input);
        });
    };

    view! {cx,
        <div>
            <p class="text-xl mb-2">"Algorithm input"</p>
            <div>
                <label>"Starting node"</label>
                <input _ref=input_ref class="bg-slate-600 w-10 text-center ml-5" />
            </div>
            <button on:click=update_input class="border rounded px-1 py-0.5">"Reset"</button>
        </div>
    }
}

#[component]
fn dfs_output(cx: Scope) -> impl IntoView {
    let (dfs, _set_dfs) = use_context::<(ReadSignal<Dfs>, WriteSignal<Dfs>)>(cx).unwrap();

    view! {cx,
    <div class="flex flex-col gap-2">
        <p class="text-xl mb-2">"Algorithm output"</p>
        {
            move || dfs.with(|dfs| {
                dfs.output.iter().map(|line| {
                    view! {cx,
                    <p>{line}</p>
                    }
                }
                ).collect_view(cx)
            })
        }
    </div>
    }
}
