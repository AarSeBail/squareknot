/// A standard implementation of union-find
pub struct UnionFind {
    uf: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            uf: (0..size).collect(),
            rank: vec![1; size],
        }
    }

    fn mut_find(&mut self, mut u: usize) -> usize {
        while self.uf[u] != u {
            // Path Compression
            self.uf[u] = self.uf[self.uf[u]];
            u = self.uf[u];
        }
        u
    }

    pub fn find(&self, u: usize) -> usize {
        self.uf[u]
    }

    pub fn union(&mut self, u: usize, v: usize) {
        let root_u = self.mut_find(u);
        let root_v = self.mut_find(v);
        if root_u != root_v {
            // The root of the component with the largest *order* is the new root
            // Slightly slower in small cases, but significantly faster in large cases
            if self.rank[root_u] < self.rank[root_v] {
                self.uf[root_u] = root_v;
                self.rank[root_v] += self.rank[root_u];
            } else {
                self.uf[root_v] = root_u;
                self.rank[root_u] += self.rank[root_v];
            }
        }
    }
}
