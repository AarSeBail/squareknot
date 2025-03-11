use std::collections::VecDeque;

use squareknot_core::AbstractGraph;

pub trait BFSGraph: AbstractGraph {
    fn bfs<'a>(&'a self, root: usize) -> BFSTraversal<'a, Self> {
        BFSTraversal::from_root(self, root)
    }
    fn full_bfs<'a>(&'a self) -> BFSTraversal<'a, Self> {
        BFSTraversal::full_traversal(self)
    }
}

impl<G: AbstractGraph> BFSGraph for G {}

pub struct BFSNode {
    pub vertex: usize,
    pub depth: usize,
}

/// Iterate over vertices using breadth first search.
pub struct BFSTraversal<'a, G: AbstractGraph> {
    graph: &'a G,
    /// Storing parents allows us to trace paths
    parents: Vec<usize>,
    queue: VecDeque<BFSNode>,
    rooted: bool,
    next_vertex: usize,
    started: bool,
}

impl<'a, G: AbstractGraph> BFSTraversal<'a, G> {
    /// Traverse starting from root.
    pub fn from_root(graph: &'a G, root: usize) -> Self {
        let mut parents = vec![usize::MAX; graph.order()];
        parents[root] = root;
        Self {
            graph,
            parents,
            queue: VecDeque::from(vec![BFSNode {
                vertex: root,
                depth: 0,
            }]),
            rooted: true,
            next_vertex: 0,
            started: false,
        }
    }

    /// Iterate over `0..nv` and traverse starting from any unvisited vertices.
    pub fn full_traversal(graph: &'a G) -> Self {
        Self {
            graph,
            parents: vec![usize::MAX; graph.order()],
            queue: VecDeque::new(),
            rooted: false,
            next_vertex: 0,
            started: false,
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
        self.queue.clear();
        self.queue.push_back(BFSNode {
            vertex: root,
            depth: 0,
        });
    }
}

impl<'a, G: AbstractGraph> Iterator for BFSTraversal<'a, G> {
    type Item = BFSNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.started = true;

        loop {
            if let Some(current_node) = self.queue.pop_front() {
                let neighbors = self.graph.neighbors(current_node.vertex);

                for neighbor in neighbors {
                    if self.parents[neighbor] == usize::MAX {
                        self.parents[neighbor] = current_node.vertex;
                        self.queue.push_back(BFSNode {
                            vertex: neighbor,
                            depth: current_node.depth + 1,
                        });
                    }
                }
                return Some(current_node);
            } else if self.rooted {
                return None; // Only traverse root component
            } else {
                // Find next unvisited vertex for full traversal
                while self.next_vertex < self.graph.size() {
                    let v = self.next_vertex;
                    self.next_vertex += 1;
                    if self.parents[v] == usize::MAX {
                        self.parents[v] = v;
                        self.queue.push_back(BFSNode {
                            vertex: v,
                            depth: 0,
                        });
                        break;
                    }
                }
                if self.queue.is_empty() {
                    return None;
                }
            }
        }
    }
}
