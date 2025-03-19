use crate::AbstractGraph;

/// Adds some performance focused methods
pub trait FastGraph: AbstractGraph<VertexLabel = usize> {
    unsafe fn add_edge_unchecked(&mut self, u: usize, v: usize);
}
