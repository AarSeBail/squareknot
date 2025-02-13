use super::{Neighbors, Storage};

pub struct AdjacencyList {
    pub size: usize,
    pub order: usize,
    pub lists: Vec<Vec<usize>>,
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
            size: nv,
            order: 0,
            lists: (0..nv).map(|_| vec![]).collect(),
        }
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        self.lists[from].contains(&to)
    }
    
    fn add_edge(&mut self, from: usize, to: usize) {
        self.lists[from].push(to);
        self.order += 1;
    }
    
    fn in_degree(&self, vertex: usize) -> usize {
        self.lists.iter()
            .map(|v| 
                v.iter().filter(|&&x| x == vertex).count()
            ).sum()
    }
    
    fn out_degree(&self, vertex: usize) -> usize {
        self.lists[vertex].len()
    }
    
    fn neighbors<'a>(&'a self, vertex: usize) -> Neighbors<'a> {
        Neighbors::Referenced(&self.lists[vertex])
    }
    
    fn size(&self) -> usize {
        self.size
    }

    fn order(&self) -> usize {
        self.order
    }

    fn complete_graph(nv: usize) -> Self {
        AdjacencyList {
            size: nv,
            order: 0,
            lists: (0..nv).map(|i| 
                (0..i).chain(i + 1..nv).collect()
            ).collect(),
        }
    }
}