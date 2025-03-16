//! This crate bundles the functionality of all internal crates

pub use graph::*;
pub use squareknot_graph as graph;

#[cfg(feature = "traversal")]
pub use squareknot_traversal as traversal;
#[cfg(feature = "traversal")]
pub use traversal::*;

#[cfg(feature = "io")]
pub use squareknot_io as io;

#[cfg(feature = "metadata")]
pub use squareknot_metadata as metadata;

#[cfg(feature = "pathing")]
pub use squareknot_pathing as pathing;

#[cfg(feature = "planarity")]
pub use squareknot_planarity as planarity;

#[cfg(feature = "combinators")]
pub use squareknot_combinators as combinators;
