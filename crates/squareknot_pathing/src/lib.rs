use squareknot_graph::AbstractGraph;
use squareknot_traversal::TraversalGraph;

pub mod apsp;
pub mod shortest_path;

pub trait PathingGraph: AbstractGraph<VertexLabel = usize> {
    fn shortest_path(graph: &Self, u: usize, v: usize) -> Result<Vec<usize>, ()>;
}

impl<G: TraversalGraph> PathingGraph for G {
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
