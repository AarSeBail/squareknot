//! This crate adds the [`GraphLike`] trait and several implementations.

mod conversion;
pub mod vertex_filter;
pub mod vertex_map;

use std::hash::Hash;

use squareknot_graph::AbstractGraph;
use vertex_filter::VertexFilter;
use vertex_map::VertexMap;

//
pub trait GraphLike {
    type VertexLabel: Copy + Hash;

    /// Iterate over vertices by label.
    /// This may produce duplicate vertices.
    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a;

    /// Iterate over edges by label.
    /// This may produce duplicates or self-loops.
    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a;

    /// A combinator that filters the vertices of a [`GraphLike`] object.
    fn filter_vertices<F: Fn(Self::VertexLabel) -> bool>(&self, f: F) -> VertexFilter<'_, Self, F> where Self: Sized {
        VertexFilter {
            preimage: self,
            f
        }
    }

    /// A combinator that maps (relabels) the vertices of a [`GraphLike`] object.
    fn map_vertices<V: Copy + Hash, F: Fn(Self::VertexLabel) -> V>(&self, f: F) -> VertexMap<'_, Self, V, F> where Self: Sized {
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