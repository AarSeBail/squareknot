use crate::GraphLike;

pub struct VertexFilter<'g, G: GraphLike, F: Fn(G::VertexLabel) -> bool> {
    pub(crate) preimage: &'g G,
    pub(crate) f: F,
}

impl<'g, G: GraphLike, F: Fn(G::VertexLabel) -> bool> GraphLike
    for VertexFilter<'g, G, F>
{
    type VertexLabel = G::VertexLabel;

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.preimage.vertex_iterator()
            .filter(|&v| (self.f)(v))
    }

    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.preimage.edge_iterator()
            .filter(|&(u, v)| (self.f)(u) && (self.f)(v))
    }
}
