use crate::graph::{storage::Storage, UnGraph};

pub struct BFSNode {
    vertex: usize,
    depth: usize
}

pub struct BFSTraversal<'a, S: Storage> {
    graph: &'a UnGraph<S>
}

impl<'a, S: Storage> BFSTraversal<'a, S> {
    fn rooted(graph: &'a UnGraph<S>, root: usize) -> Self {
        Self {
            graph
        }
    }

    fn unrooted(graph: &'a UnGraph<S>) -> Self {
        Self {
            graph
        }
    }
}

impl<'a, S: Storage> Iterator for BFSTraversal<'a, S> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}