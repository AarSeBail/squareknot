use std::collections::VecDeque;

use crate::{BFSFullTraversal, BFSTraversal, DFSFullTraversal, DFSTraversal, TraversalGraph, TraversalNode};

pub struct BFSResources {
    pub(crate) visited: Option<Vec<bool>>,
    pub(crate) queue: Option<Vec<TraversalNode>>
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
            depth: 0
        });
        (v, q)
    }

    pub fn bfs<'a, G: TraversalGraph>(self, graph: &'a G, root: usize) -> BFSTraversal<'a, G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        BFSTraversal {
            graph,
            visited: v,
            queue: q
        }
    }

    pub fn full_bfs<'a, G: TraversalGraph>(self, graph: &'a G, root: usize) -> BFSFullTraversal<'a, G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        BFSFullTraversal {
            graph,
            visited: v,
            queue: q,
            vertex_order: Box::new(graph.vertex_iterator())
        }
    }
}

pub struct DFSResources {
    pub(crate) visited: Option<Vec<bool>>,
    pub(crate) queue: Option<VecDeque<TraversalNode>>
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
            depth: 0
        });
        (v, q)
    }

    pub fn bfs<'a, G: TraversalGraph>(self, graph: &'a G, root: usize) -> DFSTraversal<'a, G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        DFSTraversal {
            graph,
            visited: v,
            queue: q
        }
    }

    pub fn full_bfs<'a, G: TraversalGraph>(self, graph: &'a G, root: usize) -> DFSFullTraversal<'a, G> {
        let (v, q) = self.reset(graph.num_v_labels(), root);

        DFSFullTraversal {
            graph,
            visited: v,
            queue: q,
            vertex_order: Box::new(graph.vertex_iterator())
        }
    }
}