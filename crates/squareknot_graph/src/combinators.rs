// Issues with dynamic return typing right now
// pub mod vertex_map;
// pub use vertex_map::*;

pub mod vertex_filter;
pub use vertex_filter::*;

pub mod vertex_compactor;
pub use vertex_compactor::*;

pub mod edge_filter;
pub use edge_filter::*;

pub mod graph_view;
pub use graph_view::*;

use std::hash::Hash;

/// View Combinators
pub trait ViewCombinator: Sized {
    type VertexLabel: Copy + Hash + Eq;

    /// Iterates over vertices by label.
    /// Consumers of this should expect to handle duplicate vertices.
    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a;

    /// Iterates over edges by label.
    /// This should not produce self-loops.
    /// Consumers of this should expect to handle duplicate edges.
    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a;

    fn neighbor_iterator<'a>(
        &'a self,
        vertex: Self::VertexLabel
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a>;

    fn filter_edges<F: Fn(&(Self::VertexLabel, Self::VertexLabel)) -> bool>(self, f: F) -> EdgeFilter<Self, F> {
        EdgeFilter::build(self, f)
    }

    fn filter_vertices<F: Fn(&Self::VertexLabel) -> bool>(self, f: F) -> VertexFilter<Self, F> {
        VertexFilter::build(self, f)
    }

    /* fn map_vertices<V: Copy + Hash + Eq, F: Fn(&Self::VertexLabel) -> V>(self, f: F) -> VertexMap<Self, V, F> {
        VertexMap::build(self, f)
    }*/ 

    fn compact(self) -> VertexCompactor<Self> {
        VertexCompactor::build(self)
    }
}

pub trait ExactCombinator: ViewCombinator {
    fn num_v_labels(&self) -> usize;
}