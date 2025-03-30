use std::hash::Hash;

use crate::{ExactCombinator, ViewCombinator};

pub struct VertexMap<G: ViewCombinator, V: Copy + Hash + Eq, F: Fn(&G::VertexLabel) -> V> {
    preimage: G,
    f: F,
}

impl<'g, G: ViewCombinator, V: Copy + Hash + Eq, F: Fn(&G::VertexLabel) -> V> VertexMap<G, V, F> {
    pub fn build(preimage: G, map: F) -> Self {
        Self { preimage, f: map }
    }
}

impl<G: ViewCombinator, V: Copy + Hash + Eq, F: Fn(&G::VertexLabel) -> V> ViewCombinator
    for VertexMap<G, V, F>
{
    type VertexLabel = V;

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.preimage.vertex_iterator().map(|v| (self.f)(&v))
    }

    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.preimage
            .edge_iterator()
            .map(|(u, v)| ((self.f)(&u), (self.f)(&v)))
            .filter(|&(u, v)| u != v)
    }

    fn neighbor_iterator<'a>(
        &'a self,
        vertex: Self::VertexLabel,
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a> {
        if !self
            .preimage
            .vertex_iterator()
            .any(|u| (self.f)(&u) == vertex)
        {
            return None;
        }
        let upstream = self
            .preimage
            .vertex_iterator()
            .filter(move |u| (self.f)(u) == vertex);
        Some(
            upstream
                .filter_map(|v| self.preimage.neighbor_iterator(v))
                .flatten()
                .map(|v| (self.f)(&v)),
        )
    }
}

impl<'g, G: ViewCombinator + ExactCombinator, F: Fn(&G::VertexLabel) -> usize> ExactCombinator
    for VertexMap<G, usize, F>
{
    fn num_v_labels(&self) -> usize {
        self.preimage.num_v_labels()
    }
}
