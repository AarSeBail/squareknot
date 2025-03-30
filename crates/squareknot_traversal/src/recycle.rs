use std::collections::VecDeque;

use crate::{BFSFullTraversal, BFSTraversal, DFSFullTraversal, DFSTraversal, TraversalNode, TraversalView};

/*use crate::{
    BFSFullTraversal, BFSTraversal, DFSFullTraversal, DFSTraversal, TraversalView as TraversalView,
    TraversalNode,
};*/

pub struct BFSResources {
    pub(crate) parents: Option<Vec<usize>>,
    pub(crate) queue: Option<VecDeque<TraversalNode>>,
}

impl BFSResources {
    fn reset(mut self, new_size: usize, new_root: usize) -> (Vec<usize>, VecDeque<TraversalNode>) {
        let mut v = self.parents.take().unwrap();
        let mut q = self.queue.take().unwrap();
        v.fill(usize::MAX);
        v.resize(new_size, usize::MAX);

        q.clear();
        q.push_back(TraversalNode {
            vertex: new_root,
            depth: 0,
            parent: new_root
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
    pub(crate) parents: Option<Vec<usize>>,
    pub(crate) stack: Option<Vec<TraversalNode>>,
    pub(crate) output: Option<Vec<bool>>
}

impl DFSResources {
    fn reset(mut self, new_size: usize, new_root: usize) -> (Vec<usize>, Vec<TraversalNode>, Vec<bool>) {
        let mut v = self.parents.take().unwrap();
        let mut q = self.stack.take().unwrap();
        let mut o = self.output.take().unwrap();
        v.fill(usize::MAX);
        v.resize(new_size, usize::MAX);

        q.clear();
        q.push(TraversalNode {
            vertex: new_root,
            depth: 0,
            parent: new_root
        });

        o.fill(false);
        (v, q, o)
    }

    pub fn dfs<G: TraversalView>(self, graph: &G, root: usize) -> DFSTraversal<G, false> {
        let (v, q, o) = self.reset(graph.num_v_labels(), root);

        DFSTraversal {
            graph,
            parents: v,
            stack: q,
            output: o
        }
    }

    pub fn dfs_post_order<G: TraversalView>(self, graph: &G, root: usize) -> DFSTraversal<G, true> {
        let (v, q, mut o) = self.reset(graph.num_v_labels(), root);

        o.resize(graph.num_v_labels(), false);

        DFSTraversal {
            graph,
            parents: v,
            stack: q,
            output: o
        }
    }

    pub fn full_dfs<G: TraversalView>(self, graph: &G, root: usize) -> DFSFullTraversal<G, false> {
        let (v, q, o) = self.reset(graph.num_v_labels(), root);

        DFSFullTraversal {
            graph,
            parents: v,
            stack: q,
            vertex_order: Box::new(graph.vertex_iterator()),
            output: o
        }
    }

    pub fn full_dfs_post_order<G: TraversalView>(self, graph: &G, root: usize) -> DFSFullTraversal<G, true> {
        let (v, q, mut o) = self.reset(graph.num_v_labels(), root);

        o.resize(graph.num_v_labels(), false);

        DFSFullTraversal {
            graph,
            parents: v,
            stack: q,
            vertex_order: Box::new(graph.vertex_iterator()),
            output: o
        }
    }
}