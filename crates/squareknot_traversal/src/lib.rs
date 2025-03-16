//! This crate contains functionality related to graph traversal.
pub mod breadth_first;

pub use breadth_first::*;

pub mod full_breadth_first;
pub use full_breadth_first::*;

pub mod depth_first;
pub use depth_first::*;

pub mod full_depth_first;
pub use full_depth_first::*;

pub mod recorded_breadth_first;
pub use recorded_breadth_first::*;

pub mod recycle;

use squareknot_graph::AbstractGraph;

pub struct TraversalNode {
    pub vertex: usize,
    pub depth: usize,
}

/// Trait implementing traversal methods on graphs with `usize` vertices
pub trait TraversalGraph: AbstractGraph<VertexLabel = usize> {
    /// Traverses the graph starting from `root`.
    fn bfs(&self, root: usize) -> BFSTraversal<Self> {
        BFSTraversal::new(self, root)
    }

    /// Creates a full traversal of the graph starting from first vertex from the graph (See [`AbstractGraph::vertex_iterator`]).
    fn full_bfs(&self) -> BFSFullTraversal<Self> {
        BFSFullTraversal::new(self)
    }

    /// Traverses the graph starting from `root`.
    fn dfs(&self, root: usize) -> DFSTraversal<Self> {
        DFSTraversal::new(self, root)
    }

    /// Creates a full traversal of the graph starting from first vertex from the graph (See [`AbstractGraph::vertex_iterator`]).
    fn full_dfs(&self) -> DFSFullTraversal<Self> {
        DFSFullTraversal::new(self)
    }

    /// Traverses the graph starting from `root` and recording the traversal as a tree.
    fn recorded_bfs(&self, root: usize) -> RBFSTraversal<Self> {
        RBFSTraversal::new(self, root)
    }

    fn component_count(&self) -> usize {
        self.full_dfs().filter(|n| n.depth == 0).count()
    }
}

impl<G: AbstractGraph<VertexLabel = usize>> TraversalGraph for G {}
