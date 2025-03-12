pub mod fast;
pub mod graph;
pub mod storage;
pub mod undirected;

pub use fast::*;
pub use graph::*;
pub use storage::*;
pub use undirected::*;

pub type SimpleGraph = UnGraph<AdjacencyList>;
