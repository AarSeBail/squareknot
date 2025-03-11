use crate::{AbstractGraph, Storage};

pub struct UnGraph<S: Storage> {
    pub storage: S,
}

impl<S: Storage> UnGraph<S> {
    pub fn with_vertices(vertex_count: usize) -> Self {
        Self {
            storage: S::with_capacity(vertex_count),
        }
    }

    pub fn complete_graph(vertex_count: usize) -> Self {
        Self {
            storage: S::complete_graph(vertex_count),
        }
    }

    pub fn degree(&self, vertex: usize) -> usize {
        self.storage.out_degree(vertex)
    }
}

impl<S: Storage> AbstractGraph for UnGraph<S> {
    fn empty(num_verticies: usize) -> Self {
        Self::with_vertices(num_verticies)
    }

    fn size(&self) -> usize {
        self.storage.size() / 2
    }

    fn order(&self) -> usize {
        self.storage.order()
    }

    // Return a bool?
    fn add_edge(&mut self, u: usize, v: usize) -> bool {
        if !self.storage.has_edge(u, v) {
            self.storage.add_undirected_edge(u, v);
            true
        } else {
            false
        }
    }

    #[inline]
    fn add_edge_unchecked(&mut self, u: usize, v: usize) {
        self.storage.add_undirected_edge(u, v);
    }

    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.storage.has_edge(u, v)
    }

    fn neighbors<'a>(&'a self, vertex: usize) -> impl Iterator<Item = usize> + 'a {
        self.storage.neighbors(vertex)
    }
}
