use std::collections::VecDeque;

use squareknot_graph::{ExactCombinator, ViewCombinator};

use crate::{recycle::BFSResources, TraversalNode};

pub struct BFSFullTraversal<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> {
    pub(crate) graph: &'a G,
    pub(crate) parents: Vec<usize>,
    pub(crate) queue: VecDeque<TraversalNode>,
    pub(crate) vertex_order: Box<dyn Iterator<Item = usize> + 'a>,
}

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> BFSFullTraversal<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            parents: vec![usize::MAX; graph.num_v_labels()],
            queue: VecDeque::new(),
            vertex_order: Box::new(graph.vertex_iterator()),
        }
    }

    /// Extracts [`BFSResources`] which may be recycled into a new [`super::BFSTraversal`] or [`BFSFullTraversal`]
    pub fn extract_resources(self) -> BFSResources {
        BFSResources {
            parents: Some(self.parents),
            queue: Some(self.queue),
        }
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
    for BFSFullTraversal<'a, G>
{
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_node) = self.queue.pop_front() {
                // Accept both types of Neighbors and create a slice from them.
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
                return Some(current_node);
            } else {
                while let Some(v) = self.vertex_order.next() {
                    if self.parents[v] != usize::MAX {
                        continue;
                    }
                    self.parents[v] = v;
                    self.queue.push_back(TraversalNode {
                        vertex: v,
                        depth: 0,
                        parent: v,
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
