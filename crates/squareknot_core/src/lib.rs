pub mod storage;
pub mod graph;
pub mod undirected;

pub use storage::*;
pub use graph::*;
pub use undirected::*;

// Fundamental types
pub type SimpleGraph = UnGraph<AdjacencyList>;
