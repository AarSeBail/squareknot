use storage::{adjacency_list::AdjacencyList, Storage};

pub mod storage;
pub mod undirected;

// Fundamental types
pub struct UnGraph<S: Storage> {
    pub storage: S
}

pub type SimpleGraph = UnGraph<AdjacencyList>;

/* pub struct Digraph<S: Storage> {
    storage: S
}

pub struct Quiver<S: Storage> {
    storage: S
}*/

pub trait AbstractGraph {
    fn size(&self) -> usize;
    fn order(&self) -> usize;
    /// Test if the graph contains the edge `(u, v)`.
    fn add_edge(&mut self, u: usize, v: usize);
    /// Add the edge `(u, v)` to the graph.
    fn has_edge(&self, u: usize, v: usize) -> bool;
    // fn neighbors<'a>(&'a self, vertex: usize) -> Neighbors<'a>;
}