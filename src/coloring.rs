use crate::{
    graph::{AbstractGraph, UnGraph},
    prelude::Storage,
};

pub fn greedy_coloring<'a, S: Storage, I: Iterator<Item = &'a usize>>(
    graph: &UnGraph<S>,
    order: I,
) -> (usize, Vec<usize>) {
    let mut coloring = vec![usize::MAX; graph.size()];
    let mut chrom = 0;
    let mut local_colors = vec![];

    for &u in order {
        // Don't drop this.
        let mut neighbors = graph.neighbors(u);
        // This is O(n^2), but it's faster than the O(n) alternative for small n.
        if graph.degree(u) < 500 {
            // If the neighborhood of `u` does not contain some `c` in `0..chrom`,
            // then color `u` as `c`.
            // Otherwise color `u` as `chrom` and increment `chrom`.
            if let Some(c) = (0..chrom)
                .filter(|&c| !neighbors.any(|x| coloring[x] == c))
                .next()
            {
                coloring[u] = c
            } else {
                local_colors.push(false);
                coloring[u] = chrom;
                chrom += 1;
            }
        } else {
            local_colors.iter_mut().for_each(|x| *x = false);
            for v in neighbors.filter(|&v| coloring[v] != usize::MAX) {
                local_colors[coloring[v]] = true;
            }
            if let Some(c) = local_colors.iter().position(|x| !x) {
                coloring[u] = c
            } else {
                local_colors.push(false);
                coloring[u] = chrom;
                chrom += 1;
            }
        }
    }

    (chrom, coloring)
}
