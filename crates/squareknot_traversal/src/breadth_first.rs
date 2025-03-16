use squareknot_graph::AbstractGraph;

use crate::{recycle::BFSResources, TraversalNode};

pub struct BFSTraversal<'a, G: AbstractGraph<VertexLabel = usize>> {
    pub(crate) graph: &'a G,
    pub(crate) visited: Vec<bool>,
    pub(crate) queue: Vec<TraversalNode>,
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> BFSTraversal<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Self {
        let mut visited = vec![false; graph.num_v_labels()];
        visited[root] = true;
        Self {
            graph,
            visited,
            queue: Vec::from(vec![TraversalNode {
                vertex: root,
                depth: 0,
            }]),
        }
    }

    /// Extracts [`BFSResources`] which may be recycled into a new [`BFSTraversal`] or [`super::BFSFullTraversal`]
    pub fn extract_resources(self) -> BFSResources {
        BFSResources {
            visited: Some(self.visited),
            queue: Some(self.queue)
        }
    }

    pub fn restart_at(&mut self, root: usize) {
        self.visited.fill(false);

        self.queue.clear();
        self.queue.push(TraversalNode {
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

impl<'a, G: AbstractGraph<VertexLabel = usize>> Iterator for BFSTraversal<'a, G> {
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
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
            Some(current_node)
        } else {
            None
        }
    }
}
