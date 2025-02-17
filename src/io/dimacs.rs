use std::marker::PhantomData;

use crate::graph::AbstractGraph;

use super::{pair_iter::PairIterator, GraphFormat};

/// This is not real DIMACS
/// This is a modified DIMACS format
pub struct DimacsFormat<G: AbstractGraph> {
    _p: PhantomData<G>,
}

impl<G: AbstractGraph> GraphFormat<G> for DimacsFormat<G> {
    fn parse_graph<R: std::io::BufRead>(reader: R) -> Result<G, ()> {
        let mut pairs = PairIterator::new(reader);
        let (nv, ne) = pairs.next().ok_or(())?;

        let mut graph = G::empty(nv);

        for (u, v) in pairs.take(ne) {
            graph.add_edge_unchecked(u, v);
        }
        Ok(graph)
    }

    // TODO: Implement this
    fn write_graph<W: std::io::Write>(_writer: W) -> Result<(), ()> {
        todo!()
    }
}
