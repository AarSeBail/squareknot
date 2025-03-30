//! This crate contains functionality related to graph traversal.
pub mod breadth_first;
pub use breadth_first::*;

pub mod full_breadth_first;
pub use full_breadth_first::*;

pub mod depth_first;
pub use depth_first::*;

pub mod full_depth_first;
pub use full_depth_first::*;

pub mod recycle;

use squareknot_graph::{ExactCombinator, ViewCombinator};

#[derive(Clone, Copy, Hash, Debug)]
pub struct TraversalNode {
    pub vertex: usize,
    pub depth: usize,
    pub parent: usize,
}

/// Trait implementing traversal methods on graphs with `usize` vertices
pub trait TraversalView: ViewCombinator<VertexLabel = usize> + ExactCombinator {
    /// Traverses the graph starting from `root`.
    fn bfs(&self, root: usize) -> BFSTraversal<Self> {
        BFSTraversal::new(self, root)
    }

    /// Traverses the graph starting from `root`.
    fn dfs(&self, root: usize) -> DFSTraversal<Self, false> {
        DFSTraversal::new(self, root)
    }

    /// Traverses the graph starting from `root`.
    fn dfs_post_order(&self, root: usize) -> DFSTraversal<Self, true> {
        DFSTraversal::new(self, root)
    }

    /// Creates a full traversal of the graph starting from first vertex from the graph (See [`AbstractGraph::vertex_iterator`]).
    fn full_bfs(&self) -> BFSFullTraversal<Self> {
        BFSFullTraversal::new(self)
    }

    fn full_dfs(&self) -> DFSFullTraversal<Self, false> {
        DFSFullTraversal::new(self)
    }

    fn full_dfs_post_order(&self) -> DFSFullTraversal<Self, true> {
        DFSFullTraversal::new(self)
    }

    fn component_count(&self) -> usize {
        self.full_dfs().filter(|n| n.depth == 0).count()
    }
}

impl<G: ViewCombinator<VertexLabel = usize> + ExactCombinator> TraversalView for G {}
