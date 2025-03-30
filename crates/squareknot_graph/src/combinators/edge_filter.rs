use crate::{ExactCombinator, ViewCombinator};

pub struct EdgeFilter<G: ViewCombinator, F: Fn(&(G::VertexLabel, G::VertexLabel)) -> bool> {
    preimage: G,
    f: F,
}

impl<G: ViewCombinator, F: Fn(&(G::VertexLabel, G::VertexLabel)) -> bool> EdgeFilter<G, F> {
    pub fn build(preimage: G, filter: F) -> Self {
        Self {
            preimage,
            f: filter,
        }
    }
}

impl<G: ViewCombinator, F: Fn(&(G::VertexLabel, G::VertexLabel)) -> bool> ViewCombinator
    for EdgeFilter<G, F>
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
        self.preimage.edge_iterator().filter(|e| (self.f)(e))
    }

    fn neighbor_iterator<'a>(
        &'a self,
        vertex: Self::VertexLabel,
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a> {
        self.preimage
            .neighbor_iterator(vertex)
            .map(move |neigh| neigh.filter(move |&v| (self.f)(&(vertex, v))))
    }
}

impl<
        G: ViewCombinator<VertexLabel = usize> + ExactCombinator,
        F: Fn(&(G::VertexLabel, G::VertexLabel)) -> bool,
    > ExactCombinator for EdgeFilter<G, F>
{
    fn num_v_labels(&self) -> usize {
        self.preimage.num_v_labels()
    }
}
