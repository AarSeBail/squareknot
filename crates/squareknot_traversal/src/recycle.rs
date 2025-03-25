use std::collections::VecDeque;

use crate::{
    BFSFullTraversal, BFSTraversal, DFSFullTraversal, DFSTraversal, TraversalView as TraversalView,
    TraversalNode,
};

pub struct BFSResources {
    pub(crate) visited: Option<Vec<usize>>,
    pub(crate) queue: Option<VecDeque<TraversalNode>>,
}

impl BFSResources {
    fn reset(mut self, new_size: usize, new_root: usize) -> (Vec<usize>, VecDeque<TraversalNode>) {
        let mut v = self.visited.take().unwrap();
        let mut q = self.queue.take().unwrap();
        v.fill(usize::MAX);
        v.resize(new_size, usize::MAX);

        q.clear();
        q.push_back(TraversalNode {
            vertex: new_root,
            depth: 0,
        });
        (v, q)
    }

    pub fn bfs<G: TraversalView>(self, graph: &G, root: usize) -> BFSTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        BFSTraversal {
            graph,
            parents: v,
            queue: q,
        }
    }

    pub fn full_bfs<G: TraversalView>(self, graph: &G, root: usize) -> BFSFullTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        BFSFullTraversal {
            graph,
            parents: v,
            queue: q,
            vertex_order: Box::new(graph.vertex_iterator()),
        }
    }
}

pub struct DFSResources {
    pub(crate) visited: Option<Vec<usize>>,
    pub(crate) stack: Option<Vec<TraversalNode>>,
}

impl DFSResources {
    fn reset(mut self, new_size: usize, new_root: usize) -> (Vec<usize>, Vec<TraversalNode>) {
        let mut v = self.visited.take().unwrap();
        let mut q = self.stack.take().unwrap();
        v.fill(usize::MAX);
        v.resize(new_size, usize::MAX);

        q.clear();
        q.push(TraversalNode {
            vertex: new_root,
            depth: 0,
        });
        (v, q)
    }

    pub fn bfs<G: TraversalView>(self, graph: &G, root: usize) -> DFSTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        DFSTraversal {
            graph,
            visited: v,
            stack: q,
        }
    }

    pub fn full_bfs<G: TraversalView>(self, graph: &G, root: usize) -> DFSFullTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        DFSFullTraversal {
            graph,
            visited: v,
            stack: q,
            vertex_order: Box::new(graph.vertex_iterator()),
        }
    }
}