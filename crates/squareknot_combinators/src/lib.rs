//! This crate adds the [`GraphCombinator`] trait and implementations.
//! It's important to remember that graph combinators do not necessarily implement [`squareknot_graph::AbstractGraph`].

mod conversion;
pub mod vertex_map;
pub mod vertex_filter;

pub trait GraphLike {
    fn num_v_labels(&self) -> usize;
    fn has_vertex(&self) -> usize;
    fn has_edge(&self) -> usize;
}

pub trait GraphCombinator: GraphLike {

}