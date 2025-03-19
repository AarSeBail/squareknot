use crate::{ExactCombinator, ViewCombinator};

pub struct VertexFilter<
    G: ViewCombinator,
    F: Fn(&G::VertexLabel) -> bool,
> {
    preimage: G,
    f: F,
}

impl<G: ViewCombinator, F: Fn(&G::VertexLabel) -> bool> VertexFilter<G, F> {
    pub fn build(preimage: G, filter: F) -> Self {
        Self {
            preimage,
            f: filter
        }
    }
}

impl<G: ViewCombinator, F: Fn(&G::VertexLabel) -> bool> ViewCombinator
    for VertexFilter<G, F>
{
    type VertexLabel = G::VertexLabel;

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.preimage.vertex_iterator()
            .filter(|&v| (self.f)(&v))
    }

    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.preimage
            .edge_iterator()
            .filter(|&(u, v)| (self.f)(&u) && (self.f)(&v))
    }
    
    fn neighbor_iterator<'a>(
        &'a self,
        vertex: Self::VertexLabel
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a> {
        if (self.f)(&vertex) {
            self.preimage.neighbor_iterator(vertex)
                .map(|neigh|
                    neigh.filter(|v| (self.f)(v))
                )
        }else {
            None
        }
    }
}

impl<G: ViewCombinator + ExactCombinator, F: Fn(&G::VertexLabel) -> bool> ExactCombinator
    for VertexFilter<G, F>
{
    fn num_v_labels(&self) -> usize {
        self.preimage.num_v_labels()
    }
}
