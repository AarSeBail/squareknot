use super::Storage;

#[derive(Clone)]
pub struct AdjacencyNode {
    pub neighbors: Option<Vec<usize>>,
}

pub struct AdjacencyList {
    pub list: Vec<AdjacencyNode>,
}

impl AdjacencyList {
    pub fn out_degree(&self, vertex: usize) -> usize {
        self.list[vertex].neighbors.as_ref().unwrap().len()
    }
}

impl Storage for AdjacencyList {
    type VertexLabel = usize;

    // Constructors
    /// Construct storage for a graph on `nv` vertices with no edges.
    fn empty(nv: usize) -> Self {
        Self {
            list: (0..nv)
                .map(|_x| AdjacencyNode {
                    neighbors: Some(vec![]),
                })
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
    fn add_vertex(&mut self) -> Self::VertexLabel {
        self.list.push(AdjacencyNode {
            neighbors: Some(vec![]),
        });
        self.list.len() - 1
    }
    /// Add `count` vertices to storage.
    fn add_vertices(&mut self, count: usize) {
        for _ in 0..count {
            self.add_vertex();
        }
    }
    /// Remove a vertex by its label.
    fn rem_vertex(&mut self, label: usize) {
        self.list[label].neighbors = None;
    }

    // Edge Modifiers
    /// Add an edge. This is unchecked.
    unsafe fn add_edge(&mut self, from: Self::VertexLabel, to: Self::VertexLabel) {
        if let Some(neighbors) = &mut self.list[from].neighbors {
            neighbors.push(to);
        }
    }
    /// Remove an edge based on its label.
    fn rem_edge(&mut self, from: Self::VertexLabel, to: Self::VertexLabel) {
        if let Some(neighbors) = &mut self.list[from].neighbors {
            if let Some(&i) = neighbors.iter().find(|&&x| x == to) {
                neighbors.remove(i);
            }
        }
    }

    /// Remove an undirected edge.
    fn rem_undirected_edge(&mut self, u: Self::VertexLabel, v: Self::VertexLabel) {
        self.rem_edge(u, v);
        self.rem_edge(v, u);
    }

    // Accessors
    /// Return true if and only if the graph contains the specified vertex label.
    fn has_vertex(&self, label: Self::VertexLabel) -> bool {
        self.list[label].neighbors.is_some()
    }
    /// Return true if and only if the graph contains the specified edge label.
    fn has_edge(&self, from: Self::VertexLabel, to: Self::VertexLabel) -> bool {
        if let Some(neighbors) = &self.list[from].neighbors {
            neighbors.contains(&to)
        } else {
            false
        }
    }

    // Basic Iterators
    /// Iterate over vertices by label.
    fn vertex_iterator<'a>(&'a self) -> impl Iterator<Item = Self::VertexLabel> + 'a {
        self.list
            .iter()
            .enumerate()
            .filter(|(_, x)| x.neighbors.is_some())
            .map(|(i, _)| i)
    }
    /// Iterate over edges by label.
    fn edge_iterator<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Self::VertexLabel, Self::VertexLabel)> + 'a {
        self.list
            .iter()
            .enumerate()
            .filter(|(_, x)| x.neighbors.is_some())
            .map(|(u, x)| x.neighbors.as_ref().unwrap().iter().map(move |&v| (u, v)))
            .flatten()
    }
    /// Iterate over neighbors of `vertex` by label.
    fn neighbor_iterator<'a>(
        &'a self,
        vertex: usize,
    ) -> Option<impl Iterator<Item = Self::VertexLabel> + 'a> {
        self.list[vertex]
            .neighbors
            .as_ref()
            .map(|neighbors| neighbors.iter().map(|&x| x))
    }
}
