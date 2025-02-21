use crate::graph::{storage::Storage, AbstractGraph, UnGraph};

#[derive(Debug)]
pub struct DFSNode {
    pub vertex: usize,
    pub depth: usize,
}

/// Iterate over vertices using depth first search.
pub struct DFSTraversal<'a, S: Storage> {
    graph: &'a UnGraph<S>,
    parents: Vec<usize>,
    stack: Vec<DFSNode>,
    rooted: bool,
    next_vertex: usize,
    started: bool,
    depth_map: Option<Vec<usize>>,
}

impl<'a, S: Storage> DFSTraversal<'a, S> {
    /// Traverse starting from root.
    pub fn from_root(graph: &'a UnGraph<S>, root: usize) -> Self {
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
            depth_map: None
        }
    }

    /// Iterate over `0..nv` and traverse starting from any unvisited vertices.
    pub fn full_traversal(graph: &'a UnGraph<S>) -> Self {
        Self {
            graph,
            parents: vec![usize::MAX; graph.size()],
            stack: Vec::new(),
            rooted: false,
            next_vertex: 0,
            started: false,
            depth_map: None
        }
    }

    /// Return the vector of parent maps.
    pub fn extract_parents(self) -> Vec<usize> {
        self.parents
    }

    /// Extracts the path to a vertex.
    /// If the iterator has not yet reached the vertex, no path will return.
    /// Once a path has been returned for a vertex, the result of this function will not change
    /// for the vertex.
    pub fn extract_path(&self, mut vertex: usize) -> Option<Vec<usize>> {
        if self.parents[vertex] == usize::MAX {
            None
        } else {
            // TODO: Compare performance of implementations.
            // If depth maps do not help much, consider removing them.
            // Note that this will mostly be called at low depths.
            if let Some(dm) = &self.depth_map {
                let mut path = vec![vertex; dm[vertex] + 1];
                let mut i = path.len() - 2;
                while self.parents[vertex] != vertex {
                    vertex = self.parents[vertex];
                    path[i] = vertex;
                    i -= 1;
                }
                Some(path)
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
        if let Some(dm) = &mut self.depth_map {
            dm[root] = 0;
        }
    }
}

impl<'a, S: Storage> Iterator for DFSTraversal<'a, S> {
    type Item = DFSNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.started = true;

        loop {
            if let Some(current_node) = self.stack.pop() {
                // Accept both types of Neighbors and create a slice from them.
                let neighbors = self.graph.storage.neighbors(current_node.vertex);
                /*let refd = match &neighbors {
                    Neighbors::Owned(v) => v,
                    Neighbors::Referenced(v) => *v,
                };*/
                // Push neighbors in reverse order
                // This ensures that traversal order matches the recursive order
                for &neighbor in neighbors.as_slice().iter().rev() {
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
