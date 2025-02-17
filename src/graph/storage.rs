pub mod adjacency_list;

pub enum Neighbors<'a> {
    Owned(Vec<usize>),
    Referenced(&'a [usize]),
}

impl<'a> Neighbors<'a> {
    pub fn as_slice<'b>(&'b self) -> &'a [usize]
    where
        'b: 'a,
    {
        match self {
            Self::Owned(v) => &v,
            Self::Referenced(x) => x,
        }
    }
}

/// The backing storage structure for a graph.
/// All implementations must support directed multigraphs.
pub trait Storage: Sized {
    /// Returns an instance with no vertices.
    fn empty() -> Self;

    /// Returns an instance with `nv` isolated vertices.
    fn with_capacity(nv: usize) -> Self;

    /// Returns `true` if the graph contains the edge `from -> to`.
    fn has_edge(&self, from: usize, to: usize) -> bool;

    /// Adds the (directed) edge `from -> to`.
    fn add_edge(&mut self, from: usize, to: usize);

    /* fn rem_edge(&mut self, from: usize, to: usize);

    fn add_vertex(&mut self) -> usize; */

    /// Returns the number of edges into `vertex`.
    fn in_degree(&self, vertex: usize) -> usize;

    /// Returns the number of edges out of `vertex`.
    /// This is sometimes faster than in_degree. Prefer this if ``in_degree = out_degree``.
    fn out_degree(&self, vertex: usize) -> usize;

    /// Returns the neighbors of `vertex`
    /// i.e. all vertices `u` such that the storage contains the edge `vertex -> u`
    fn neighbors<'a>(&'a self, vertex: usize) -> Neighbors<'a>;
    fn neighbors_owned<'a>(&'a self, vertex: usize) -> Vec<usize> {
        match self.neighbors(vertex) {
            Neighbors::Owned(ret) => ret,
            Neighbors::Referenced(ret) => ret.to_vec(),
        }
    }

    fn size(&self) -> usize;
    fn order(&self) -> usize;

    // Blanket implementations
    /// Returns `true` if the storage contains the edge `u -> v` or `v -> u`.
    fn has_undirected_edge(&mut self, u: usize, v: usize) -> bool {
        self.has_edge(u, v) || self.has_edge(v, u)
    }

    /// Adds the edges `u -> v` and `v -> u`
    fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    /// Returns an instance with `nv` vertices and all possible edges.
    fn complete_graph(nv: usize) -> Self {
        let mut ret = Self::with_capacity(nv);
        for u in 1..nv {
            for v in 0..u {
                ret.add_edge(u, v);
            }
        }
        ret
    }
}
