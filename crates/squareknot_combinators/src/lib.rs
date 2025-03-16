//! This crate adds the [`GraphLike`] trait and several implementations.

mod conversion;
pub mod vertex_filter;
pub mod vertex_map;
pub mod living;

use std::hash::Hash;

use squareknot_graph::AbstractGraph;
use vertex_filter::VertexFilter;
use vertex_map::VertexMap;
pub use living::*;


pub trait GraphLike {
    type VertexLabel: Copy + Hash;

    /// Iterates over vertices by label.
    /// Consumers of this should expect to handle duplicate vertices.
    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a;

    /// Iterates over edges by label.
    /// This should not produce self-loops.
    /// Consumers of this should expect to handle duplicate edges.
    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a;

    /// Filters the vertices of a [`GraphLike`] object.
    fn filter_vertices<F: Fn(Self::VertexLabel) -> bool>(&self, f: F) -> VertexFilter<'_, Self, F> where Self: Sized {
        VertexFilter {
            preimage: self,
            f
        }
    }

    /// Maps (relabels) the vertices of a [`GraphLike`] object.
    fn map_vertices<V: Copy + Hash + Eq, F: Fn(Self::VertexLabel) -> V>(&self, f: F) -> VertexMap<'_, Self, V, F> where Self: Sized {
        VertexMap {
            preimage: self,
            f
        }
    }
}

impl<G: AbstractGraph> GraphLike for G {
    type VertexLabel = G::VertexLabel;

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.vertex_iterator()
    }

    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.edge_iterator()
    }
}