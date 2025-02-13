use std::collections::VecDeque;

use crate::{graph::{storage::Storage, AbstractGraph, UnGraph}, prelude::Neighbors};

pub struct BFSNode {
    pub vertex: usize,
    pub depth: usize
}

pub struct BFSTraversal<'a, S: Storage> {
    graph: &'a UnGraph<S>,
    visited: Vec<bool>,
    queue: VecDeque<BFSNode>,
    rooted: bool,
    next_vertex: usize
}

impl<'a, S: Storage> BFSTraversal<'a, S> {
    pub fn from_root(graph: &'a UnGraph<S>, root: usize) -> Self {
        let mut visited = vec![false; graph.size()];
        visited[root] = true;
        Self {
            graph,
            visited,
            queue: VecDeque::from(vec![BFSNode { vertex: root, depth: 0 }]),
            rooted: true,
            next_vertex: 0,
        }
    }

    pub fn full_traversal(graph: &'a UnGraph<S>) -> Self {
        Self {
            graph,
            visited: vec![false; graph.size()],
            queue: VecDeque::new(),
            rooted: false,
            next_vertex: 0,
        }
    }
}

impl<'a, S: Storage> Iterator for BFSTraversal<'a, S> {
    type Item = BFSNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_node) = self.queue.pop_front() {
                // Accept both types of Neighbors and create a slice from them.
                let neighbors = self.graph.storage.neighbors(current_node.vertex);
                let refd = match &neighbors {
                    Neighbors::Owned(v) => v,
                    Neighbors::Referenced(v) => *v,
                };

                for &neighbor in refd.iter() {
                    if !self.visited[neighbor] {
                        self.visited[neighbor] = true;
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
                    if !self.visited[v] {
                        self.visited[v] = true;
                        self.queue.push_back(BFSNode { vertex: v, depth: 0 });
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