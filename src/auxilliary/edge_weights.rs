use std::collections::HashSet;

use super::Attribute;

pub struct EdgeWeight<N: Default> {
    lookup: HashSet<(usize, usize), N>,
}

impl<N: Default> Attribute for EdgeWeight<N> {
    fn add_edge(&mut self, u: usize, v: usize, id: usize) {}
}
