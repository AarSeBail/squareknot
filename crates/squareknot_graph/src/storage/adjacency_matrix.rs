use nalgebra::DMatrix;

use super::Storage;

pub struct AdjacencyMatrix {
    matrix: DMatrix<u8>,
}

impl Storage for AdjacencyMatrix {
    // Constructors

    /// Construct storage for a graph on `nv` vertices with no edges.
    fn empty(nv: usize) -> Self {
        Self {
            matrix: DMatrix::from_element(nv, nv, 0),
        }
    }

    // Attributes

    /// Return the number of vertex labels in storage.
    fn num_v_labels(&self) -> usize {
        self.matrix.nrows()
    }

    // Vertex modifiers

    /// Add a vertex to storage and return its label.
    fn add_vertex(&mut self) -> usize {
        let nv = self.matrix.nrows();
        self.matrix.resize_mut(nv + 1, nv + 1, 0);
        nv
    }

    /// Add `count` vertices to storage.
    fn add_vertices(&mut self, count: usize) {
        let nv = self.matrix.nrows();
        self.matrix.resize_mut(nv + count, nv + count, 0);
    }

    // Edge Modifiers

    /// Add an edge. This is unchecked.
    unsafe fn add_edge(&mut self, from: usize, to: usize) {
        self.matrix[(from, to)] = 1;
    }

    /// Remove an edge based on its label.
    fn rem_edge(&mut self, from: usize, to: usize) {
        self.matrix[(from, to)] = 0;
    }

    /// Remove an undirected edge.
    fn rem_undirected_edge(&mut self, u: usize, v: usize) {
        self.matrix[(u, v)] = 0;
        self.matrix[(v, u)] = 0;
    }

    // Accessors

    /// Return true if and only if the graph contains the specified vertex label.
    fn has_vertex(&self, label: usize) -> bool {
        label < self.matrix.nrows()
    }

    /// Return true if and only if the graph contains the specified edge label.
    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.matrix[(from, to)] == 1
    }

    // Basic Iterators

    /// Iterate over vertices by label.
    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        0..self.matrix.nrows()
    }

    /// Iterate over edges by label.
    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let nv = self.matrix.nrows();
        (0..nv)
            .map(move |u| {
                (0..nv)
                    .map(move |v| (u, v))
                    .filter(|&coord| self.matrix[coord] == 1)
            })
            .flatten()
    }
    
    /// Iterate over neighbors of `vertex` by label.
    fn neighbor_iterator<'a>(
        &'a self,
        vertex: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        let nv = self.matrix.nrows();
        (0..nv).filter(move |&v| self.matrix[(vertex, v)] == 1)
    }
}
