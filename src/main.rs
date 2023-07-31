use components::App;
use leptos::*;

mod algs;
mod components;
mod graph;
mod graphstyle;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
