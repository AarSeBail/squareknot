pub mod dimacs;
mod pair_iter;

use std::io::{BufRead, Write};

use crate::graph::AbstractGraph;

pub trait GraphFormat<G: AbstractGraph> {
    fn parse_graph<R: BufRead>(reader: R) -> Result<G, ()>;
    fn write_graph<W: Write>(writer: W) -> Result<(), ()>;
}
