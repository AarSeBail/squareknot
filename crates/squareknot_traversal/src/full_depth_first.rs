use squareknot_graph::AbstractGraph;

use std::collections::VecDeque;

use crate::{recycle::DFSResources, TraversalNode};

pub struct DFSFullTraversal<'a, G: AbstractGraph<VertexLabel = usize>> {
    pub(crate) graph: &'a G,
    pub(crate) visited: Vec<bool>,
    pub(crate) queue: VecDeque<TraversalNode>,
    pub(crate) vertex_order: Box<dyn Iterator<Item = usize> + 'a>,
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> DFSFullTraversal<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            visited: vec![false; graph.num_v_labels()],
            queue: VecDeque::new(),
            vertex_order: Box::new(graph.vertex_iterator())
        }
    }

    /// Extracts [`DFSResources`] which may be recycled into a new [`super::DFSTraversal`] or [`DFSFullTraversal`]
    pub fn extract_resources(self) -> DFSResources {
        DFSResources {
            visited: Some(self.visited),
            queue: Some(self.queue)
        }
    }

    /// Iterate over the vertices in the graph which have been traversed
    pub fn traversed_iter<'b>(&'b self) -> impl Iterator<Item = usize> + 'b {
        self.visited
            .iter()
            .enumerate()
            .filter(|(_, &x)| x)
            .map(|(i, _)| i)
    }
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> Iterator for DFSFullTraversal<'a, G> {
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_node) = self.queue.pop_front() {
                // Accept both types of Neighbors and create a slice from them.
                let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();
                for neighbor in neighbors {
                    if !self.visited[neighbor] {
                        self.visited[neighbor] = true;
                        self.queue.push_back(TraversalNode {
                            vertex: neighbor,
                            depth: current_node.depth + 1,
                        });
                    }
                }
                return Some(current_node);
            } else {
                // Find next unvisited vertex for full traversal
                while let Some(v) = self.vertex_order.next() {
                    if self.visited[v] {
                        continue;
                    }
                    self.visited[v] = true;
                    self.queue.push_back(TraversalNode {
                        vertex: v,
                        depth: 0
                    });
                    break;
                }
                if self.queue.is_empty() {
                    return None;
                }
            }
        }
    }
}
