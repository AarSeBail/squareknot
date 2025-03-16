use std::collections::VecDeque;

use crate::{
    BFSFullTraversal, BFSTraversal, DFSFullTraversal, DFSTraversal, RBFSTraversal, TraversalGraph, TraversalNode
};

pub struct BFSResources {
    pub(crate) visited: Option<Vec<bool>>,
    pub(crate) queue: Option<Vec<TraversalNode>>,
}

impl BFSResources {
    fn reset(mut self, new_size: usize, new_root: usize) -> (Vec<bool>, Vec<TraversalNode>) {
        let mut v = self.visited.take().unwrap();
        let mut q = self.queue.take().unwrap();
        v.fill(false);
        v.resize(new_size, false);

        q.clear();
        q.push(TraversalNode {
            vertex: new_root,
            depth: 0,
        });
        (v, q)
    }

    pub fn bfs<G: TraversalGraph>(self, graph: &G, root: usize) -> BFSTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        BFSTraversal {
            graph,
            visited: v,
            queue: q,
        }
    }

    pub fn full_bfs<G: TraversalGraph>(self, graph: &G, root: usize) -> BFSFullTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        BFSFullTraversal {
            graph,
            visited: v,
            queue: q,
            vertex_order: Box::new(graph.vertex_iterator()),
        }
    }
}

pub struct DFSResources {
    pub(crate) visited: Option<Vec<bool>>,
    pub(crate) queue: Option<VecDeque<TraversalNode>>,
}

impl DFSResources {
    fn reset(mut self, new_size: usize, new_root: usize) -> (Vec<bool>, VecDeque<TraversalNode>) {
        let mut v = self.visited.take().unwrap();
        let mut q = self.queue.take().unwrap();
        v.fill(false);
        v.resize(new_size, false);

        q.clear();
        q.push_back(TraversalNode {
            vertex: new_root,
            depth: 0,
        });
        (v, q)
    }

    pub fn bfs<G: TraversalGraph>(self, graph: &G, root: usize) -> DFSTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        DFSTraversal {
            graph,
            visited: v,
            queue: q,
        }
    }

    pub fn full_bfs<G: TraversalGraph>(self, graph: &G, root: usize) -> DFSFullTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        DFSFullTraversal {
            graph,
            visited: v,
            queue: q,
            vertex_order: Box::new(graph.vertex_iterator()),
        }
    }
}

pub struct RBFSResources {
    pub(crate) parents: Option<Vec<usize>>,
    pub(crate) queue: Option<Vec<TraversalNode>>,
}

impl RBFSResources {
    fn reset(mut self, new_size: usize, new_root: usize) -> (Vec<usize>, Vec<TraversalNode>) {
        let mut v = self.parents.take().unwrap();
        let mut q = self.queue.take().unwrap();
        v.fill(usize::MAX);
        v.resize(new_size, usize::MAX);

        q.clear();
        q.push(TraversalNode {
            vertex: new_root,
            depth: 0,
        });
        (v, q)
    }

    pub fn bfs<G: TraversalGraph>(self, graph: &G, root: usize) -> RBFSTraversal<G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        RBFSTraversal {
            graph,
            parents: v,
            queue: q,
        }
    }
}