use squareknot_graph::AbstractGraph;

pub trait BFSGraph: AbstractGraph<VertexLabel = usize> {
    fn bfs<'a>(&'a self, root: usize) -> BFSTraversal<'a, Self> {
        BFSTraversal::from_root(self, root)
    }
    fn full_bfs<'a>(&'a self) -> BFSTraversal<'a, Self> {
        BFSTraversal::full_traversal(self)
    }
}

impl<G: AbstractGraph<VertexLabel = usize>> BFSGraph for G {}

pub struct BFSNode {
    pub vertex: usize,
    pub depth: usize,
}

pub struct BFSTraversal<'a, G: AbstractGraph<VertexLabel = usize>> {
    graph: &'a G,
    visited: Vec<bool>,
    queue: Vec<BFSNode>,
    rooted: bool,
    next_vertex: usize,
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> BFSTraversal<'a, G> {
    pub fn from_root(graph: &'a G, root: usize) -> Self {
        let mut visited = vec![false; graph.num_v_labels()];
        visited[root] = true;
        Self {
            graph,
            visited,
            queue: Vec::from(vec![BFSNode {
                vertex: root,
                depth: 0,
            }]),
            rooted: true,
            next_vertex: 0,
        }
    }

    pub fn full_traversal(graph: &'a G) -> Self {
        Self {
            graph,
            visited: vec![false; graph.num_v_labels()],
            queue: Vec::new(),
            rooted: false,
            next_vertex: 0,
        }
    }
}

impl<'a, G: AbstractGraph<VertexLabel = usize>> Iterator for BFSTraversal<'a, G> {
    type Item = BFSNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_node) = self.queue.pop() {
                // Accept both types of Neighbors and create a slice from them.
                let neighbors = self.graph.neighbor_iterator(current_node.vertex).unwrap();
                for neighbor in neighbors {
                    if !self.visited[neighbor] {
                        self.visited[neighbor] = true;
                        self.queue.push(BFSNode {
                            vertex: neighbor,
                            depth: current_node.depth + 1,
                        });
                    }
                }
                return Some(current_node);
            } else if self.rooted {
                return None; // Only traverse root component
            } else {
                // Find next unvisited vertex for full traversal
                while self.next_vertex < self.graph.num_v_labels() {
                    let v = self.next_vertex;
                    self.next_vertex += 1;
                    if !self.visited[v] {
                        self.visited[v] = true;
                        self.queue.push(BFSNode {
                            vertex: v,
                            depth: 0,
                        });
                        break;
                    }
                }
                if self.queue.is_empty() {
                    return None;
                }
            }
        }
    }
}
