use crate::{graph::AbstractGraph, prelude::breadth_first::BFSTraversal};

pub struct APSPIterator<'a, G: AbstractGraph> {
    bfs: BFSTraversal<'a, G>,
    nv: usize,
    u: usize,
    v: usize
}

impl<'a, G: AbstractGraph> APSPIterator<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        let mut bfs = graph.bfs(0);
        bfs.by_ref().for_each(|_| {});
        Self {
            bfs,
            nv: graph.order(),
            u: 0,
            v: 0
        }
    }
}

impl<'a, G: AbstractGraph> Iterator for APSPIterator<'a, G> {
    type Item = (usize, usize, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.u >= self.nv {
                break
            }
            if self.v >= self.u {
                self.v = 0;
                self.u += 1;
                if self.u >= self.nv {
                    break;
                }
                self.bfs.restart_at(self.u);
                self.bfs.by_ref().for_each(|_| {});
                continue;
            }
            self.v += 1;
            if let Some(path) = self.bfs.extract_path(self.v - 1) {
                return Some((self.u, self.v - 1, path));
            }
        }
        None
    }
}