use std::hash::Hash;

use crate::AbstractGraph;

use crate::{ExactCombinator, ViewCombinator};

pub struct GraphView<'g, G: AbstractGraph> {
    graph: &'g G
}

impl<'g, G: AbstractGraph> GraphView<'g, G> {
    pub fn build(graph: &'g G) -> Self {
        Self {
            graph
        }
    }
}


impl<G: AbstractGraph<VertexLabel = V>, V: Copy + Hash + Eq> ViewCombinator for GraphView<'_, G> {
    type VertexLabel = V;

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.graph.vertex_iterator()
    }

    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.graph.edge_iterator()
    }
    
    fn neighbor_iterator<'a>(
        &'a self,
        vertex: Self::VertexLabel
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a> {
        self.graph.neighbor_iterator(vertex)
    }
}

impl<G: AbstractGraph<VertexLabel = V>, V: Copy + Hash + Eq> ExactCombinator for GraphView<'_, G> {
    fn num_v_labels(&self) -> usize {
        self.graph.num_v_labels()
    }
}
