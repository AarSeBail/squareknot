pub mod breadth_first;

pub use breadth_first::*;

pub mod full_breadth_first;
pub use full_breadth_first::*;

pub mod depth_first;
pub use depth_first::*;

pub mod full_depth_first;
pub use full_depth_first::*;

pub mod recycle;

use squareknot_graph::AbstractGraph;

pub struct TraversalNode {
    pub vertex: usize,
    pub depth: usize,
}

/// Trait implementing traversal methods on graphs with `usize` vertices
pub trait TraversalGraph: AbstractGraph<VertexLabel = usize> {
    /// Traverses the graph starting from `root`.
    fn bfs<'a>(&'a self, root: usize) -> BFSTraversal<'a, Self> {
        BFSTraversal::new(self, root)
    }

    /// Creates a full traversal of the graph starting from first vertex from the graph (See [`AbstractGraph::vertex_iterator`]).
    fn full_bfs<'a>(&'a self) -> BFSFullTraversal<'a, Self> {
        BFSFullTraversal::new(self)
    }

    /// Traverses the graph starting from `root`.
    fn dfs<'a>(&'a self, root: usize) -> DFSTraversal<'a, Self> {
        DFSTraversal::new(self, root)
    }

    /// Creates a full traversal of the graph starting from first vertex from the graph (See [`AbstractGraph::vertex_iterator`]).
    fn full_dfs<'a>(&'a self) -> DFSFullTraversal<'a, Self> {
        DFSFullTraversal::new(self)
    }

    fn component_count(&self) -> usize {
        self.full_dfs().filter(|n| n.depth == 0).count()
    }
}

impl<G: AbstractGraph<VertexLabel = usize>> TraversalGraph for G {}
