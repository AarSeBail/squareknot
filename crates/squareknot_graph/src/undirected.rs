use crate::{AbstractGraph, FastGraph, Storage};

pub struct UnGraph<S: Storage> {
    order: usize,
    size: usize,
    storage: S,
}

impl<S: Storage<VertexLabel: Ord>> AbstractGraph for UnGraph<S> {
    /// A label for vertices.
    ///
    /// It is probably a good idea for distinct vertices (in the context of the represented graph) to have distinct labels.
    type VertexLabel = S::VertexLabel;
    /// A label for edges.
    type EdgeLabel = (S::VertexLabel, S::VertexLabel);
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
    /// Currently this does not return the labels, however this may be subject to change.
    fn add_vertices(&mut self, count: usize) {
        for _ in 0..count {
            self.add_vertex();
        }
    }
    /// Remove a vertex by its label.
    fn rem_vertex(&mut self, label: Self::VertexLabel) {
        if self.storage.has_vertex(label) {
            self.storage.rem_vertex(label);
        }
    }

    // Edge Modifiers
    /// Add an edge to the graph and return `Some(label)` if it is successful.
    fn add_edge(&mut self, u: Self::VertexLabel, v: Self::VertexLabel) -> Option<Self::EdgeLabel> {
        if self.storage.has_edge(u, v) {
            None
        } else {
            unsafe {
                self.storage.add_edge(u, v);
                self.storage.add_edge(v, u);
            }
            Some((u.min(v), u.max(v)))
        }
    }
    /// Remove an edge based on its label and return `true` if it is successful.
    fn rem_edge(&mut self, label: Self::EdgeLabel) -> bool {
        if self.storage.has_edge(label.0, label.1) {
            self.storage.rem_undirected_edge(label.0, label.1);
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
    fn has_edge(&self, label: Self::EdgeLabel) -> bool {
        self.storage.has_edge(label.0, label.1)
    }
    fn endpoints(&self, label: Self::EdgeLabel) -> (Self::VertexLabel, Self::VertexLabel) {
        label
    }

    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.storage.vertex_iterator()
    }
    fn edge_iterator<'a>(&'a self) -> impl Iterator<Item = Self::EdgeLabel> + 'a {
        self.storage.edge_iterator()
    }
    fn neighbor_iterator<'a>(
        &'a self,
        vertex: usize,
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a> {
        self.storage.neighbor_iterator(vertex)
    }
}

impl<S: Storage<VertexLabel = usize>> FastGraph for UnGraph<S> {
    unsafe fn add_edge_unchecked(&mut self, u: usize, v: usize) {
        self.storage.add_edge(u, v);
        self.storage.add_edge(v, u);
    }
}
