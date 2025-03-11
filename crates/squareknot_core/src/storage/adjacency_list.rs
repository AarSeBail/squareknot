use super::Storage;

pub struct AdjacencyNode {
    pub edges: Vec<usize>,
    pub out_degree: usize,
    pub in_degree: usize,
}

impl AdjacencyNode {
    fn empty() -> Self {
        Self {
            edges: vec![],
            out_degree: 0,
            in_degree: 0,
        }
    }
}

pub struct AdjacencyList {
    pub size: usize,
    pub order: usize,
    pub lists: Vec<AdjacencyNode>,
}

impl Storage for AdjacencyList {
    fn empty() -> Self {
        AdjacencyList {
            size: 0,
            order: 0,
            lists: vec![],
        }
    }

    fn with_capacity(nv: usize) -> Self {
        AdjacencyList {
            size: 0,
            order: nv,
            lists: (0..nv).map(|_| AdjacencyNode::empty()).collect(),
        }
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.lists[from].edges.contains(&to)
    }

    #[inline]
    fn add_edge(&mut self, from: usize, to: usize) {
        self.lists[from].edges.push(to);
        self.lists[from].out_degree += 1;
        self.lists[to].in_degree += 1;
        self.size += 1;
    }

    fn in_degree(&self, vertex: usize) -> usize {
        self.lists[vertex].in_degree
    }

    fn out_degree(&self, vertex: usize) -> usize {
        self.lists[vertex].out_degree
    }

    fn neighbors<'a>(&'a self, vertex: usize) -> impl Iterator<Item = usize> + 'a {
        self.lists[vertex].edges.iter().cloned()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn order(&self) -> usize {
        self.order
    }

    fn complete_graph(nv: usize) -> Self {
        AdjacencyList {
            size: nv * (nv - 1) / 2,
            order: nv,
            lists: (0..nv)
                .map(|i| (0..i).chain(i + 1..nv).collect())
                .map(|e| AdjacencyNode {
                    edges: e,
                    in_degree: nv - 1,
                    out_degree: nv - 1,
                })
                .collect(),
        }
    }
}
