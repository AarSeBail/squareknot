use squareknot_graph::{ExactCombinator, ViewCombinator};

use crate::{recycle::DFSResources, TraversalNode};

pub struct DFSTraversal<
    'a,
    G: ViewCombinator<VertexLabel = usize> + ExactCombinator,
    const POST_ORDER: bool = false,
> {
    pub(crate) graph: &'a G,
    pub(crate) parents: Vec<usize>,
    pub(crate) stack: Vec<TraversalNode>,
    pub(crate) output: Vec<bool>
}

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator, const POST_ORDER: bool>
    DFSTraversal<'a, G, POST_ORDER>
{
    pub fn new(graph: &'a G, root: usize) -> Self {
        let output = if POST_ORDER {
            vec![false; graph.num_v_labels()]
        }else {
            vec![]
        };
        Self {
            graph,
            parents: vec![usize::MAX; graph.num_v_labels()],
            stack: vec![TraversalNode {
                vertex: root,
                depth: 0,
                parent: root,
            }],
            output
        }
    }

    /// Extracts [`DFSResources`] which may be recycled into a new [`DFSTraversal`] or [`super::DFSFullTraversal`]
    pub fn extract_resources(self) -> DFSResources {
        DFSResources {
            parents: Some(self.parents),
            stack: Some(self.stack),
            output: Some(self.output)
        }
    }

    pub fn restart_at(&mut self, root: usize) {
        self.parents.fill(usize::MAX);
        self.output.fill(false);

        self.stack.clear();
        self.stack.push(TraversalNode {
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
    for DFSTraversal<'a, G, false>
{
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current_node) = self.stack.pop() {
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
        }

        None
    }
}

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> Iterator
    for DFSTraversal<'a, G, true>
{
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&current_node) = self.stack.last() {
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
        }

        None
    }
}
