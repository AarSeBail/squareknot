use squareknot_graph::AbstractGraph;

pub trait DFSGraph: AbstractGraph<VertexLabel = usize> {
    fn dfs<'a>(&'a self, root: usize) -> DFSTraversal<'a, Self> {
        DFSTraversal::from_root(self, root)
    }
    fn full_dfs<'a>(&'a self) -> DFSTraversal<'a, Self> {
        DFSTraversal::full_traversal(self)
    }
}

impl<G: AbstractGraph<VertexLabel = usize>> DFSGraph for G {}

#[derive(Debug)]
pub struct DFSNode {
    pub vertex: usize,
    pub depth: usize,
}

/// Iterate over vertices using depth first search.
pub struct DFSTraversal<'a, G: AbstractGraph<VertexLabel = usize>> {
    graph: &'a G,
    parents: Vec<usize>,
    stack: Vec<DFSNode>,
    rooted: bool,
    next_vertex: usize,
    started: bool,
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> DFSTraversal<'a, G> {
    /// Traverse starting from root.
    pub fn from_root(graph: &'a G, root: usize) -> Self {
        let mut parents = vec![usize::MAX; graph.size()];
        parents[root] = root;
        Self {
            graph,
            parents,
            stack: vec![DFSNode {
                vertex: root,
                depth: 0,
            }],
            rooted: true,
            next_vertex: 0,
            started: false,
        }
    }

    /// Iterate over `0..nv` and traverse starting from any unvisited vertices.
    pub fn full_traversal(graph: &'a G) -> Self {
        Self {
            graph,
            parents: vec![usize::MAX; graph.size()],
            stack: Vec::new(),
            rooted: false,
            next_vertex: 0,
            started: false,
        }
    }

    /// Extracts the path to a vertex.
    /// If the iterator has not yet reached the vertex, no path will return.
    /// Once a path has been returned for a vertex, the result of this function will not change
    /// for the vertex.
    pub fn extract_path(&self, mut vertex: usize) -> Option<Vec<usize>> {
        if self.parents[vertex] == usize::MAX {
            None
        } else {
            let mut path = Vec::new();
            path.push(vertex);

            while self.parents[vertex] != vertex {
                vertex = self.parents[vertex];
                path.push(vertex);
            }

            Some(path.into_iter().rev().collect())
        }
    }

    pub fn restart_at(&mut self, root: usize) {
        self.parents.iter_mut().for_each(|x| *x = usize::MAX);
        self.rooted = true;
        self.parents[root] = root;
        self.next_vertex = 0;
        self.started = false;
        self.stack.clear();
        self.stack.push(DFSNode {
            vertex: root,
            depth: 0,
        });
    }

    /// Return the vector of parent maps.
    pub fn extract_parents(self) -> Vec<usize> {
        self.parents
    }
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> Iterator for DFSTraversal<'a, G> {
    type Item = DFSNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.started = true;

        loop {
            if let Some(current_node) = self.stack.pop() {
                // Accept both types of Neighbors and create a slice from them.
                let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();
                /*let refd = match &neighbors {
                    Neighbors::Owned(v) => v,
                    Neighbors::Referenced(v) => *v,
                };*/
                // Push neighbors in reverse order
                // This ensures that traversal order matches the recursive order
                for neighbor in neighbors {
                    if self.parents[neighbor] == usize::MAX {
                        self.parents[neighbor] = current_node.vertex;
                        self.stack.push(DFSNode {
                            vertex: neighbor,
                            depth: current_node.depth + 1,
                        });
                    }
                }
                return Some(current_node);
            } else if self.rooted {
                return None; // Only traverse root componentited
            } else {
                // Find next unvisited vertex for full traversal
                while self.next_vertex < self.graph.size() {
                    let v = self.next_vertex;
                    self.next_vertex += 1;
                    if self.parents[v] == usize::MAX {
                        self.parents[v] = v;
                        self.stack.push(DFSNode {
                            vertex: v,
                            depth: 0,
                        });
                        break;
                    }
                }
                if self.stack.is_empty() {
                    return None;
                }
            }
        }
    }
}
