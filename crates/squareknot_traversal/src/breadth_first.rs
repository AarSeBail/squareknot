use std::collections::VecDeque;

use squareknot_graph::{ExactCombinator, ViewCombinator};

use crate::{recycle::BFSResources, TraversalNode};

pub struct BFSTraversal<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> {
    pub(crate) graph: &'a G,
    pub(crate) parents: Vec<usize>,
    pub(crate) queue: VecDeque<TraversalNode>,
}

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> BFSTraversal<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Self {
        let mut parents = vec![usize::MAX; graph.num_v_labels()];
        parents[root] = root;

        Self {
            graph,
            parents,
            queue: VecDeque::from(vec![TraversalNode {
                vertex: root,
                depth: 0,
                parent: root,
            }]),
        }
    }

    /// Extracts [`BFSResources`] which may be recycled into a new [`BFSTraversal`] or [`super::BFSFullTraversal`]
    pub fn extract_resources(self) -> BFSResources {
        BFSResources {
            parents: Some(self.parents),
            queue: Some(self.queue),
        }
    }

    pub fn restart_at(&mut self, root: usize) {
        self.parents.fill(usize::MAX);
        self.parents[root] = root;

        self.queue.clear();
        self.queue.push_back(TraversalNode {
            vertex: root,
            depth: 0,
            parent: root,
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

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> Iterator
    for BFSTraversal<'a, G>
{
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_node) = self.queue.pop_front() {
            let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();
            for neighbor in neighbors {
                if self.parents[neighbor] == usize::MAX {
                    self.parents[neighbor] = current_node.vertex;
                    self.queue.push_back(TraversalNode {
                        vertex: neighbor,
                        depth: current_node.depth + 1,
                        parent: current_node.vertex,
                    });
                }
            }
            Some(current_node)
        } else {
            None
        }
    }
}
