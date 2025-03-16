use squareknot_graph::AbstractGraph;

use std::collections::VecDeque;

use crate::{recycle::DFSResources, TraversalNode};

pub struct DFSTraversal<'a, G: AbstractGraph<VertexLabel = usize>> {
    pub(crate) graph: &'a G,
    pub(crate) visited: Vec<bool>,
    pub(crate) queue: VecDeque<TraversalNode>,
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> DFSTraversal<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Self {
        let mut visited = vec![false; graph.num_v_labels()];
        visited[root] = true;
        Self {
            graph,
            visited,
            queue: VecDeque::from(vec![TraversalNode {
                vertex: root,
                depth: 0,
            }]),
        }
    }

    /// Extracts [`DFSResources`] which may be recycled into a new [`DFSTraversal`] or [`super::DFSFullTraversal`]
    pub fn extract_resources(self) -> DFSResources {
        DFSResources {
            visited: Some(self.visited),
            queue: Some(self.queue)
        }
    }

    pub fn restart_at(&mut self, root: usize) {
        self.visited.fill(false);

        self.queue.clear();
        self.queue.push_back(TraversalNode {
            vertex: root,
            depth: 0
        });
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

impl<'a, G: AbstractGraph<VertexLabel = usize>> Iterator for DFSTraversal<'a, G> {
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
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
            Some(current_node)
        } else {
            None
        }
    }
}
