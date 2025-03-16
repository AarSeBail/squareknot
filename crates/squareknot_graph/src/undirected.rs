use crate::{AbstractGraph, FastGraph, Storage};

pub struct UnGraph<S: Storage> {
    order: usize,
    size: usize,
    storage: S,
}

impl<S: Storage<VertexLabel: Ord + Copy>> AbstractGraph for UnGraph<S> {
    /// A label for vertices.
    type VertexLabel = S::VertexLabel;

    // Constructors
    /// Constructs a graph on `nv` vertices with no edges.
    fn empty(nv: usize) -> Self {
        Self {
            order: nv,
            size: 0,
            storage: S::empty(nv),
        }
    }

    // Attributes
    /// Returns the number of vertices (not labels) in a graph.
    fn order(&self) -> usize {
        self.order
    }
    /// Returns the number of edges in a graph.
    fn size(&self) -> usize {
        self.size
    }
    /// Returns the number of vertex labels in a graph.
    fn num_v_labels(&self) -> usize {
        self.storage.num_v_labels()
    }
    /*/// Returns the number of edge labels in a graph.
    fn num_e_labels(&self) -> usize {
        self.storage.num_e_labels()
    }*/

    // Vertex Modifiers
    /// Add a vertex to the graph and return its label
    fn add_vertex(&mut self) -> Self::VertexLabel {
        self.order += 1;
        self.storage.add_vertex()
    }
    /// Add `count` vertices to the graph.
    ///
    /// Currently, this does not return the labels, however this may be subject to change.
    fn add_vertices(&mut self, count: usize) {
        for _ in 0..count {
            self.add_vertex();
        }
    }

    // Edge Modifiers
    /// Add an edge to the graph and return `Some(label)` if it is successful.
    fn add_edge(&mut self, u: Self::VertexLabel, v: Self::VertexLabel) -> bool {
        if self.storage.has_edge(u, v) {
            false
        } else {
            unsafe {
                self.storage.add_edge(u, v);
                self.storage.add_edge(v, u);
            }
            true
        }
    }
    /// Remove an edge based on its label and return `true` if it is successful.
    fn rem_edge(&mut self, u: Self::VertexLabel, v: Self::VertexLabel) -> bool {
        if self.storage.has_edge(u, v) {
            self.storage.rem_undirected_edge(u, v);
            self.size -= 1;
            true
        } else {
            false
        }
    }

    // Accessors
    /// Return true if and only if the graph contains the specified vertex label.
    fn has_vertex(&self, label: Self::VertexLabel) -> bool {
        self.storage.has_vertex(label)
    }
    /// Return true if and only if the graph contains the specified edge label.
    fn has_edge(&self, u: Self::VertexLabel, v: Self::VertexLabel) -> bool {
        self.storage.has_edge(u, v)
    }

    // Basic Iterators
    /// Iterate over vertices by label.
    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.storage.vertex_iterator()
    }
    /// Iterate over edges by label.
    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.storage.edge_iterator()
    }
    /// Iterate over neighbors of `vertex` by label.
    fn neighbor_iterator<'a>(
        &'a self,
        vertex: usize,
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a> {
        Some(self.storage.neighbor_iterator(vertex))
    }
}

impl<S: Storage<VertexLabel = usize>> FastGraph for UnGraph<S> {
    unsafe fn add_edge_unchecked(&mut self, u: usize, v: usize) {
        self.storage.add_edge(u, v);
        self.storage.add_edge(v, u);
    }
}
