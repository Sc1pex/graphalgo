mod dfs;

use crate::graph::Graph;
use leptos::{Scope, View};

pub use dfs::*;

pub trait Algorithm {
    fn start(&mut self, input: usize);
    fn step(&mut self) -> Option<()>;

    fn input(&self, cx: Scope) -> View;
    fn output(&self, cx: Scope) -> View;

    // TODO: Find a better way to do this
    fn output_text(&self) -> Vec<String>;

    fn graph(&self) -> &Graph;
    fn graph_mut(&mut self) -> &mut Graph;

    fn reset(&mut self) {}
}
