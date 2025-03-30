//! This crate contains the core graph and storage types of `squareknot`.
//! Additional graph and storage types may be defined outside of this crate.

pub mod combinators;
pub mod fast;
pub mod graph;
pub mod storage;
pub mod undirected;

pub use combinators::*;
pub use fast::*;
pub use graph::*;
pub use storage::*;
pub use undirected::*;

/// An alias for an undirected graph with an adjacency list.
pub type SimpleGraph = UnGraph<AdjacencyList>;
