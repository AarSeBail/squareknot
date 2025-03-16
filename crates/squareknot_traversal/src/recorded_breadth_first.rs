use squareknot_graph::AbstractGraph;

use crate::{TraversalNode, recycle::RBFSResources};

pub struct RBFSTraversal<'a, G: AbstractGraph<VertexLabel = usize>> {
    pub(crate) graph: &'a G,
    pub(crate) parents: Vec<usize>,
    pub(crate) queue: Vec<TraversalNode>,
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> RBFSTraversal<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Self {
        Self {
            graph,
            parents: vec![usize::MAX; graph.num_v_labels()],
            queue: Vec::from(vec![TraversalNode {
                vertex: root,
                depth: 0,
            }]),
        }
    }

    /// Extracts [`RBFSResources`] which may be recycled into a new [`RBFSTraversal`]
    pub fn extract_resources(self) -> RBFSResources {
        RBFSResources {
            parents: Some(self.parents),
            queue: Some(self.queue),
        }
    }

    pub fn restart_at(&mut self, root: usize) {
        self.parents.fill(usize::MAX);

        self.queue.clear();
        self.queue.push(TraversalNode {
            vertex: root,
            depth: 0,
        });
    }

    /// Iterate over the vertices in the graph which have been traversed
    pub fn traversed_iter<'b>(&'b self) -> impl Iterator<Item = usize> + 'b {
        self.parents
            .iter()
            .enumerate()
            .filter(|(_, &x)| x != usize::MAX)
            .map(|(i, _)| i)
    }
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> Iterator for RBFSTraversal<'a, G> {
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_node) = self.queue.pop() {
            // Accept both types of Neighbors and create a slice from them.
            let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();
            for neighbor in neighbors {
                if self.parents[neighbor] == usize::MAX {
                    self.parents[neighbor] = current_node.vertex;
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
