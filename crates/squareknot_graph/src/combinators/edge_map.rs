use crate::{ExactCombinator, GraphCombinator};

pub struct EdgeMap<
    'g,
    G: GraphCombinator,
    F: Fn(&(G::VertexLabel, G::VertexLabel)) -> !,
> {
    preimage: &'g G,
    f: F,
}

impl<'g, G: GraphCombinator, F: Fn(&(G::VertexLabel, G::VertexLabel)) -> bool> EdgeFilter<'g, G, F> {
    pub fn build(preimage: &'g G, filter: F) -> Self {
        Self {
            preimage,
            f: filter
        }
    }
}

impl<'g, G: GraphCombinator, F: Fn(&(G::VertexLabel, G::VertexLabel)) -> bool> GraphCombinator
    for EdgeFilter<'g, G, F>
{
    type VertexLabel = G::VertexLabel;

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        /* self.preimage.vertex_iterator()
            .filter(|&v| (self.f)(v))*/
        self.preimage.vertex_iterator()
    }

    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        /* self.preimage
            .edge_iterator()
            .filter(|&(u, v)| (self.f)(u) && (self.f)(v))*/
        self.preimage.edge_iterator()
            .filter(|e| (self.f)(e))
    }
}

impl<'g, G: GraphCombinator<VertexLabel = usize> + ExactCombinator, F: Fn(&(G::VertexLabel, G::VertexLabel)) -> bool> ExactCombinator
    for EdgeFilter<'g, G, F>
{
    fn num_v_labels(&self) -> usize {
        self.preimage.num_v_labels()
    }
}
