use squareknot_graph::{ExactCombinator, ViewCombinator};

use crate::{recycle::DFSResources, TraversalNode};

pub struct DFSFullTraversal<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> {
    pub(crate) graph: &'a G,
    pub(crate) visited: Vec<usize>,
    pub(crate) stack: Vec<TraversalNode>,
    pub(crate) vertex_order: Box<dyn Iterator<Item = usize> + 'a>,
}

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> DFSFullTraversal<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            visited: vec![usize::MAX; graph.num_v_labels()],
            stack: Vec::new(),
            vertex_order: Box::new(graph.vertex_iterator()),
        }
    }

    /// Extracts [`DFSResources`] which may be recycled into a new [`super::DFSTraversal`] or [`DFSFullTraversal`]
    pub fn extract_resources(self) -> DFSResources {
        DFSResources {
            visited: Some(self.visited),
            stack: Some(self.stack),
        }
    }

    /// Iterate over the vertices in the graph which have been traversed
    pub fn traversed_iter<'b>(&'b self) -> impl Iterator<Item = usize> + 'b {
        self.visited
            .iter()
            .enumerate()
            .filter(|(_, &x)| x != usize::MAX)
            .map(|(i, _)| i)
    }
}

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> Iterator for DFSFullTraversal<'a, G> {
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_node) = self.stack.pop() {
                // Accept both types of Neighbors and create a slice from them.
                let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();
                for neighbor in neighbors {
                    if self.visited[neighbor] == usize::MAX {
                        self.visited[neighbor] = current_node.vertex;
                        self.stack.push(TraversalNode {
                            vertex: neighbor,
                            depth: current_node.depth + 1,
                        });
                    }
                }
                return Some(current_node);
            } else {
                // Find next unvisited vertex for full traversal
                while let Some(v) = self.vertex_order.next() {
                    if self.visited[v] != usize::MAX {
                        continue;
                    }
                    self.visited[v] = v;
                    self.stack.push(TraversalNode {
                        vertex: v,
                        depth: 0,
                    });
                    break;
                }
                if self.stack.is_empty() {
                    return None;
                }
            }
        }
    }
}
