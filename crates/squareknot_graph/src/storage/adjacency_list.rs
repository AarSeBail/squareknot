use super::Storage;

#[derive(Clone)]
struct AdjacencyNode {
    neighbors: Vec<usize>,
}

pub struct AdjacencyList {
    list: Vec<AdjacencyNode>,
}

impl Storage for AdjacencyList {
    // Constructors

    /// Construct storage for a graph on `nv` vertices with no edges.
    fn empty(nv: usize) -> Self {
        Self {
            list: (0..nv)
                .map(|_x| AdjacencyNode { neighbors: vec![] })
                .collect(),
        }
    }

    // Attributes

    /// Return the number of vertex labels in storage.
    fn num_v_labels(&self) -> usize {
        self.list.len()
    }

    // Vertex modifiers

    /// Add a vertex to storage and return its label.
    fn add_vertex(&mut self) -> usize {
        self.list.push(AdjacencyNode { neighbors: vec![] });
        self.list.len() - 1
    }

    /// Add `count` vertices to storage.
    fn add_vertices(&mut self, count: usize) {
        for _ in 0..count {
            self.add_vertex();
        }
    }

    // Edge Modifiers

    /// Add an edge. This is unchecked.
    unsafe fn add_edge(&mut self, from: usize, to: usize) {
        self.list[from].neighbors.push(to)
    }

    /// Remove an edge based on its label.
    fn rem_edge(&mut self, from: usize, to: usize) {
        if from < self.list.len() && to < self.list.len() {
            if let Some(&i) = self.list[from].neighbors.iter().find(|&&x| x == to) {
                self.list[from].neighbors.remove(i);
            }
        }
    }

    /// Remove an undirected edge.
    fn rem_undirected_edge(&mut self, u: usize, v: usize) {
        self.rem_edge(u, v);
        self.rem_edge(v, u);
    }

    // Accessors

    /// Return true if and only if the graph contains the specified vertex label.
    fn has_vertex(&self, label: usize) -> bool {
        label < self.list.len()
    }

    /// Return true if and only if the graph contains the specified edge label.
    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.list[from].neighbors.contains(&to)
    }

    // Basic Iterators

    /// Iterate over vertices by label.
    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        0..self.list.len()
    }

    /// Iterate over edges by label.
    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.list
            .iter()
            .enumerate()
            .map(|(u, x)| x.neighbors.iter().map(move |&v| (u, v)))
            .flatten()
    }

    /// Iterate over neighbors of `vertex` by label.
    fn neighbor_iterator<'a>(
        &'a self,
        vertex: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        self.list[vertex].neighbors.iter().cloned()
    }
}