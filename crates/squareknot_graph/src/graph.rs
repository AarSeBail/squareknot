use std::hash::Hash;

/// Abstract trait for labeled graphs
/// A few things to note about the behavior of `AbstractGraph`
///
/// - Both edges and vertices have labels.
/// - Removal of a vertex preserves the labels of all other vertices.
/// - Removal of an edge preserves the labels of all other edges.
/// - This trait is designed with multigraphs in mind, however the implementations provided strictly support simple un/directed graphs.
pub trait AbstractGraph: Sized {
    /// A label for vertices.
    /// It is probably a good idea for distinct vertices (in the context of the represented graph) to have distinct labels.
    type VertexLabel: Copy + Hash;

    // Constructors
    /// Constructs a graph on `nv` vertices with no edges.
    fn empty(nv: usize) -> Self;

    // Attributes
    /// Returns the number of vertices (not labels) in a graph.
    fn order(&self) -> usize;
    /// Returns the number of edges in a graph.
    fn size(&self) -> usize;
    /// Returns the number of vertex labels in a graph.
    fn num_v_labels(&self) -> usize;
    /*/// Returns the number of edge labels in a graph.
    fn num_e_labels(&self) -> usize;*/

    // Vertex Modifiers
    /// Add a vertex to the graph and return its label
    fn add_vertex(&mut self) -> Self::VertexLabel;
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
    fn add_edge(&mut self, u: Self::VertexLabel, v: Self::VertexLabel) -> bool;
    /// Remove an edge based on its label and return `true` if it is successful.
    fn rem_edge(&mut self, u: Self::VertexLabel, v: Self::VertexLabel) -> bool;

    // Accessors
    /// Return true if and only if the graph contains the specified vertex label.
    fn has_vertex(&self, label: Self::VertexLabel) -> bool;
    /// Return true if and only if the graph contains the specified edge label.
    fn has_edge(&self, u: Self::VertexLabel, v: Self::VertexLabel) -> bool;


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
