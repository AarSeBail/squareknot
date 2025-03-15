use std::hash::Hash;

pub mod adjacency_list;
pub use adjacency_list::*;

/// A trait which, for the most part, mirrors [`AbstractGraph`]
///
/// This provides an extra layer of abstraction, allowing the reuse ofstorage code for both directed and undirected graphs.
///
/// This also allows graphs using this trait to be backend independent, allowing them to be reusable components as well.
///
/// Accordingly this trait is intended to support all such cases.
///
/// It is possible to support only one case (e.g. a `Storage` implementation for only undirected graphs), however this is undefined behavior and will not be explicitly supported.
pub trait Storage: Sized {
    /// A label for vertices.
    ///
    /// See [`AbstractGraph::VertexLabel`]
    type VertexLabel: Copy + Hash;

    // Constructors
    /// Construct storage for a graph on `nv` vertices with no edges.
    fn empty(nv: usize) -> Self;

    // Attributes
    /// Return the number of vertex labels in storage.
    fn num_v_labels(&self) -> usize;

    // Vertex modifiers
    /// Add a vertex to storage and return its label.
    fn add_vertex(&mut self) -> Self::VertexLabel;
    /// Add `count` vertices to storage.
    fn add_vertices(&mut self, count: usize) {
        for _ in 0..count {
            self.add_vertex();
        }
    }
    /// Remove a vertex by its label.
    fn rem_vertex(&mut self, label: Self::VertexLabel);

    // Edge Modifiers
    /// Add an edge. This is unsafe since it can produce multi-edges.
    unsafe fn add_edge(&mut self, from: Self::VertexLabel, to: Self::VertexLabel);
    /// Remove an edge based on its label.
    fn rem_edge(&mut self, from: Self::VertexLabel, to: Self::VertexLabel);

    /// Remove an undirected edge.
    fn rem_undirected_edge(&mut self, u: Self::VertexLabel, v: Self::VertexLabel);

    // Accessors
    /// Return true if and only if the graph contains the specified vertex label.
    fn has_vertex(&self, label: Self::VertexLabel) -> bool;
    /// Return true if and only if the graph contains the specified edge label.
    fn has_edge(&self, from: Self::VertexLabel, to: Self::VertexLabel) -> bool;

    // Basic Iterators
    /// Iterate over vertices by label.
    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a;
    /// Iterate over edges by label.
    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a;
    /// Iterate over neighbors of `vertex` by label.
    fn neighbor_iterator<'a>(
        &'a self,
        vertex: usize,
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a>;
}
