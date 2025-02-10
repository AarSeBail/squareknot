use super::{storage::Storage, AbstractGraph, UnGraph};

impl<S: Storage> UnGraph<S> {
    pub fn empty() -> Self {
        Self {
            storage: S::empty()
        }
    }

    pub fn with_vertices(vertex_count: usize) -> Self {
        Self {
            storage: S::with_capacity(vertex_count)
        }
    }
    
    pub fn complete_graph(vertex_count: usize) -> Self {
        Self {
            storage: S::complete_graph(vertex_count)
        }
    }
}

impl<S: Storage> AbstractGraph for UnGraph<S> {
    fn size(&self) -> usize {
        self.storage.size()
    }

    fn order(&self) -> usize {
        self.storage.order() / 2
    }
    
    fn add_edge(&mut self, u: usize, v: usize) {
        self.storage.add_undirected_edge(u, v);
    }
    
    fn has_edge(&self, u: usize, v: usize) -> bool {
        self.storage.has_edge(u, v)
    }
}