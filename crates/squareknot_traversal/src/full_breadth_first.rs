use squareknot_graph::AbstractGraph;

use crate::{recycle::BFSResources, TraversalNode};

pub struct BFSFullTraversal<'a, G: AbstractGraph<VertexLabel = usize>> {
    pub(crate) graph: &'a G,
    pub(crate) visited: Vec<bool>,
    pub(crate) queue: Vec<TraversalNode>,
    pub(crate) vertex_order: Box<dyn Iterator<Item = usize> + 'a>,
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> BFSFullTraversal<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            visited: vec![false; graph.num_v_labels()],
            queue: Vec::new(),
            vertex_order: Box::new(graph.vertex_iterator()),
        }
    }

    /// Extracts [`BFSResources`] which may be recycled into a new [`super::BFSTraversal`] or [`BFSFullTraversal`]
    pub fn extract_resources(self) -> BFSResources {
        BFSResources {
            visited: Some(self.visited),
            queue: Some(self.queue),
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

impl<'a, G: AbstractGraph<VertexLabel = usize>> Iterator for BFSFullTraversal<'a, G> {
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_node) = self.queue.pop() {
                // Accept both types of Neighbors and create a slice from them.
                let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();
                for neighbor in neighbors {
                    if !self.visited[neighbor] {
                        self.visited[neighbor] = true;
                        self.queue.push(TraversalNode {
                            vertex: neighbor,
                            depth: current_node.depth + 1,
                        });
                    }
                }
                return Some(current_node);
            } else {
                while let Some(v) = self.vertex_order.next() {
                    if self.visited[v] {
                        continue;
                    }
                    self.visited[v] = true;
                    self.queue.push(TraversalNode {
                        vertex: v,
                        depth: 0,
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
