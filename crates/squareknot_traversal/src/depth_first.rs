use squareknot_graph::{ExactCombinator, ViewCombinator};

use crate::{recycle::DFSResources, TraversalNode};

pub struct DFSTraversal<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> {
    pub(crate) graph: &'a G,
    pub(crate) visited: Vec<usize>,
    pub(crate) stack: Vec<TraversalNode>,
}

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> DFSTraversal<'a, G> {
    pub fn new(graph: &'a G, root: usize) -> Self {
        let mut visited = vec![usize::MAX; graph.num_v_labels()];
        visited[root] = root;
        Self {
            graph,
            visited,
            stack: Vec::from(vec![TraversalNode {
                vertex: root,
                depth: 0,
            }]),
        }
    }

    /// Extracts [`DFSResources`] which may be recycled into a new [`DFSTraversal`] or [`super::DFSFullTraversal`]
    pub fn extract_resources(self) -> DFSResources {
        DFSResources {
            visited: Some(self.visited),
            stack: Some(self.stack),
        }
    }

    pub fn restart_at(&mut self, root: usize) {
        self.visited.fill(usize::MAX);
        self.visited[root] = root;

        self.stack.clear();
        self.stack.push(TraversalNode {
            vertex: root,
            depth: 0,
        });
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

impl<'a, G: ViewCombinator<VertexLabel = usize> + ExactCombinator> Iterator for DFSTraversal<'a, G> {
    type Item = TraversalNode;

    fn next(&mut self) -> Option<Self::Item> {
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
            Some(current_node)
        } else {
            None
        }
    }
}
