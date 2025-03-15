use std::marker::PhantomData;

use squareknot_graph::{AbstractGraph, FastGraph};

use super::{pair_iter::PairIterator, GraphFormat};

/// This is not real DIMACS
/// This is a modified DIMACS format
pub struct FakeDimacs<G: AbstractGraph> {
    _p: PhantomData<G>,
}

impl<G: FastGraph> GraphFormat<G> for FakeDimacs<G> {
    fn parse_graph<R: std::io::BufRead>(reader: R) -> Result<G, ()> {
        let mut pairs = PairIterator::new(reader);
        let (nv, ne) = pairs.next().ok_or(())?;

        let mut graph = G::empty(nv);

        for (u, v) in pairs.take(ne) {
            unsafe {
                graph.add_edge_unchecked(u, v);
            }
        }
        Ok(graph)
    }

    // TODO: Implement this
    fn write_graph<W: std::io::Write>(_writer: W) -> Result<(), ()> {
        todo!()
    }
}
