//! Abstractions over graph metadata

use crate::graph::AbstractGraph;

/// An trait for structures which represent metadata about a graph
/// While these most often implement `VertexAttribute` or `EdgeAttribute`,
/// they are not required to
pub trait Attribute {
    fn add_edge(&mut self, u: usize, v: usize, id: usize);
}

pub trait VertexAttribute: Attribute {}

pub trait EdgeAttribute: Attribute {}

pub struct MetaGraph<A: Attribute, G: AbstractGraph> {
    _graph: G,
    _attribute: A,
}

impl<A: Attribute, G: AbstractGraph> MetaGraph<A, G> {}
