use crate::graph::{storage::{Neighbors, Storage}, AbstractGraph, UnGraph};

#[derive(Debug)]
pub struct DFSNode {
    pub vertex: usize,
    pub depth: usize,
}

pub struct DFSTraversal<'a, S: Storage> {
    graph: &'a UnGraph<S>,
    visited: Vec<bool>,
    stack: Vec<DFSNode>,
    rooted: bool,
    next_vertex: usize,
}

impl<'a, S: Storage> DFSTraversal<'a, S> {
    pub fn from_root(graph: &'a UnGraph<S>, root: usize) -> Self {
        let mut visited = vec![false; graph.size()];
        visited[root] = true;
        Self {
            graph,
            visited,
            stack: vec![DFSNode { vertex: root, depth: 0 }],
            rooted: true,
            next_vertex: 0,
        }
    }

    pub fn full_traversal(graph: &'a UnGraph<S>) -> Self {
        Self {
            graph,
            visited: vec![false; graph.size()],
            stack: Vec::new(),
            rooted: false,
            next_vertex: 0,
        }
    }
}

impl<'a, S: Storage> Iterator for DFSTraversal<'a, S> {
    type Item = DFSNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_node) = self.stack.pop() {
                // Accept both types of Neighbors and create a slice from them.
                let neighbors = self.graph.storage.neighbors(current_node.vertex);
                let refd = match &neighbors {
                    Neighbors::Owned(v) => v,
                    Neighbors::Referenced(v) => *v,
                };
                // Push neighbors in reverse order
                // This ensures that traversal order matches the recursive order
                for &neighbor in refd.iter().rev() {
                    if !self.visited[neighbor] {
                        self.visited[neighbor] = true;
                        self.stack.push(DFSNode {
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
                    if !self.visited[v] {
                        self.visited[v] = true;
                        self.stack.push(DFSNode { vertex: v, depth: 0 });
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