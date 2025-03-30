use std::vec;

use squareknot_graph::{ExactCombinator, ViewCombinator};

use crate::{recycle::DFSResources, TraversalNode};

pub struct DFSFullTraversal<
    'a,
    G: ViewCombinator<VertexLabel = usize> + ExactCombinator,
    const POST_ORDER: bool,
> {
    pub(crate) graph: &'a G,
    pub(crate) parents: Vec<usize>,
    pub(crate) stack: Vec<TraversalNode>,
    pub(crate) vertex_order: Box<dyn Iterator<Item = usize> + 'a>,
    pub(crate) output: Vec<bool>
}

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator, const POST_ORDER: bool>
    DFSFullTraversal<'a, G, POST_ORDER>
{
    pub fn new(graph: &'a G) -> Self {
        let output = if POST_ORDER {
            vec![false; graph.num_v_labels()]
        }else {
            vec![]
        };
        Self {
            graph,
            parents: vec![usize::MAX; graph.num_v_labels()],
            stack: Vec::new(),
            vertex_order: Box::new(graph.vertex_iterator()),
            output
        }
    }

    /// Extracts [`DFSResources`] which may be recycled into a new [`DFSFullTraversal`] or [`DFSFullTraversal`]
    pub fn extract_resources(self) -> DFSResources {
        DFSResources {
            parents: Some(self.parents),
            stack: Some(self.stack),
            output: Some(self.output)
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
    for DFSFullTraversal<'a, G, false>
{
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_node) = self.stack.pop() {
                if self.parents[current_node.vertex] != usize::MAX {
                    continue;
                }

                self.parents[current_node.vertex] = current_node.parent;

                let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();

                for neighbor in neighbors {
                    self.stack.push(TraversalNode {
                        vertex: neighbor,
                        depth: current_node.depth + 1,
                        parent: current_node.vertex,
                    });
                }

                return Some(current_node);
            } else {
                // Find next unvisited vertex for full traversal
                while let Some(v) = self.vertex_order.next() {
                    if self.parents[v] != usize::MAX {
                        continue;
                    }
                    self.stack.push(TraversalNode {
                        vertex: v,
                        depth: 0,
                        parent: v,
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

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> Iterator
    for DFSFullTraversal<'a, G, true>
{
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(&current_node) = self.stack.last() {
                // Vertex has already been output
                if self.output[current_node.vertex] {
                    self.stack.pop();
                    continue;
                }

                self.parents[current_node.vertex] = current_node.parent;

                let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();

                let mut count = 0;

                for neighbor in neighbors {
                    if self.parents[neighbor] != usize::MAX {
                        continue;
                    }
                    self.stack.push(TraversalNode {
                        vertex: neighbor,
                        depth: current_node.depth + 1,
                        parent: current_node.vertex,
                    });
                    count += 1;
                }

                if count == 0 {
                    let current_node = self.stack.pop().unwrap();
                    self.output[current_node.vertex] = true;
    
                    return Some(current_node);
                }
            } else {
                // Find next unvisited vertex for full traversal
                while let Some(v) = self.vertex_order.next() {
                    if self.parents[v] != usize::MAX {
                        continue;
                    }
                    self.stack.push(TraversalNode {
                        vertex: v,
                        depth: 0,
                        parent: v,
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
