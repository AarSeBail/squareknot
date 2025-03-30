use std::collections::HashMap;

use crate::{ExactCombinator, ViewCombinator};

pub struct VertexCompactor<G: ViewCombinator> {
    preimage: G,
    map: HashMap<G::VertexLabel, usize>,
}

impl<'g, G: ViewCombinator> VertexCompactor<G> {
    pub fn build(preimage: G) -> Self {
        let mut map = HashMap::new();

        for v in preimage.vertex_iterator() {
            if !map.contains_key(&v) {
                map.insert(v, map.len());
            }
        }

        Self { preimage, map }
    }
}

impl<G: ViewCombinator> ViewCombinator for VertexCompactor<G> {
    type VertexLabel = usize;

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        0..self.map.len()
    }

    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.preimage
            .edge_iterator()
            .map(|(u, v)| (self.map[&u], self.map[&v]))
    }

    fn neighbor_iterator<'a>(
        &'a self,
        vertex: Self::VertexLabel,
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a> {
        let key = self.map.iter().filter(|&(_, &v)| v == vertex).next();
        if let None = key {
            return None;
        }
        let k = key.unwrap().0;
        self.preimage
            .neighbor_iterator(*k)
            .map(|neigh| neigh.map(|v| self.map[&v]))
    }
}

impl<G: ViewCombinator> ExactCombinator for VertexCompactor<G> {
    fn num_v_labels(&self) -> usize {
        self.map.len()
    }
}
