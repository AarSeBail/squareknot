//! Abstractions over graph metadata

use std::marker::PhantomData;

use crate::graph::AbstractGraph;

pub mod edge_weights;
pub mod vertex_weights;
pub mod vertex_degree;

/// An trait for structures which represent metadata about a graph
/// While these most often implement `VertexAttribute` or `EdgeAttribute`,
/// they are not required to
pub trait Attribute {

}

pub trait VertexAttribute: Attribute {

}

pub trait EdgeAttribute: Attribute {

}

pub struct MetaGraph<T: Attribute, G: AbstractGraph> {
    _p: PhantomData<(T, G)>
}
