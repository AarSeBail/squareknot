//! This crate contains functionality related to graph paths.

use squareknot_traversal::TraversalView;

pub trait PathingGraph: TraversalView {
    fn shortest_path(graph: &Self, u: usize, v: usize) -> Result<Vec<usize>, ()>;
}

impl<G: TraversalView> PathingGraph for G {
    // TODO: This is no longer the best way to do this
    fn shortest_path(graph: &Self, u: usize, v: usize) -> Result<Vec<usize>, ()> {
        let mut stack = vec![];
        let bfs = graph.bfs(u);
        for n in bfs {
            if n.depth < stack.len() {
                stack[n.depth] = n.vertex;
            } else {
                stack.push(n.vertex);
            }
            if n.vertex == v {
                stack.resize(n.depth + 1, usize::MAX);
                return Ok(stack);
            }
        }
        Err(())
    }
}
