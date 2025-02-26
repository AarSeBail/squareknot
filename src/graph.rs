pub mod storage;
pub mod undirected;

pub use storage::Neighbors;
use storage::{adjacency_list::AdjacencyList, *};

use crate::{prelude::breadth_first::BFSTraversal, DFSTraversal};

// Fundamental types
pub struct UnGraph<S: Storage> {
    pub storage: S,
}

pub type SimpleGraph = UnGraph<AdjacencyList>;

/* pub struct Digraph<S: Storage> {
    storage: S
}

pub struct Quiver<S: Storage> {
    storage: S
}*/

pub trait AbstractGraph: Sized {
    fn empty(num_verticies: usize) -> Self;
    /// Returns the number of edges in a graph.
    fn size(&self) -> usize;
    /// Returns the number of vertices in a graph.
    fn order(&self) -> usize;
    /// Test if the graph contains the edge `(u, v)`.
    fn add_edge(&mut self, u: usize, v: usize) -> bool;
    fn add_edge_unchecked(&mut self, u: usize, v: usize);
    /// Add the edge `(u, v)` to the graph.
    fn has_edge(&self, u: usize, v: usize) -> bool;
    fn neighbors<'a>(&'a self, vertex: usize) -> Neighbors<'a>;
    fn bfs<'a>(&'a self, root: usize) -> BFSTraversal<'a, Self> {
        BFSTraversal::from_root(self, root)
    }
    fn full_bfs<'a>(&'a self) -> BFSTraversal<'a, Self> {
        BFSTraversal::full_traversal(self)
    }
    fn dfs<'a>(&'a self, root: usize) -> DFSTraversal<'a, Self> {
        DFSTraversal::from_root(self, root)
    }
    fn full_dfs<'a>(&'a self) -> DFSTraversal<'a, Self> {
        DFSTraversal::full_traversal(self)
    }
}
