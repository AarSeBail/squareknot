use crate::GraphLike;

use std::hash::Hash;

pub struct VertexMap<'g, G: GraphLike, V: Copy + Hash, F: Fn(G::VertexLabel) -> V> {
    pub(crate) preimage: &'g G,
    pub(crate) f: F,
}

impl<'g, G: GraphLike, V: Copy + Hash, F: Fn(G::VertexLabel) -> V> GraphLike
    for VertexMap<'g, G, V, F>
{
    type VertexLabel = V;

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.preimage.vertex_iterator()
            .map(|v| (self.f)(v))
    }

    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.preimage.edge_iterator()
            .map(|(u, v)| ((self.f)(u), (self.f)(v)))
    }
}
