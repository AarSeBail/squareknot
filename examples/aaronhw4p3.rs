use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use rand::seq::SliceRandom;

// Iterator over pairs of numbers
struct PairIterator<B: BufRead> {
    reader: B,
    buffer: String,
}

impl<B: std::io::BufRead> Iterator for PairIterator<B> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        if let Ok(_) = self.reader.read_line(&mut self.buffer) {
            self.buffer
                .split_whitespace()
                .map(|s| s.parse::<usize>().ok())
                .take(2)
                .filter_map(|n| n)
                .collect_tuple::<(usize, usize)>()
        } else {
            None
        }
    }
}

pub fn greedy_coloring<'a, I: Iterator<Item = &'a usize>>(
    graph: &Vec<Vec<usize>>,
    order: I,
) -> (usize, Vec<usize>) {
    let mut coloring = vec![usize::MAX; graph.len()];
    let mut chrom = 0;
    let mut local_colors = vec![];

    for &u in order {
        // Don't drop this.
        let n = &graph[u];
        // This is O(n^2), but it's faster than the O(n) alternative for small n.
        if n.len() < 250 {
            // If the neighborhood of `u` does not contain some `c` in `0..chrom`,
            // then color `u` as `c`.
            // Otherwise color `u` as `chrom` and increment `chrom`.
            if let Some(c) = (0..chrom)
                .filter(|&c| !n.iter().any(|&x| coloring[x] == c))
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
            for &v in n.iter().filter(|&v| coloring[*v] != usize::MAX) {
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

// Counting the number of components using the standard union-find approach
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        std::process::exit(1);
    }

    // Boilerplate to create an iterator over pairs in the graph
    let f = File::open(&args[1]).expect("Could not open input file");
    let reader = BufReader::with_capacity(1024 * 64, f);

    let mut pairs = PairIterator {
        reader,
        buffer: String::with_capacity(20),
    };

    let (nv, _) = pairs.next().unwrap();

    let mut adj = (0..nv).map(|_| Vec::new()).collect::<Vec<_>>();
    let deg = vec![0; nv];

    for (u, v) in pairs {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut ord: Vec<_> = (0..nv).map(|v| (deg[v], v)).collect();
    ord.sort_by_key(|&(d, _)| d);
    let mut order = ord.iter().map(|&(_, v)| v).collect::<Vec<_>>();

    let (c, _) = greedy_coloring(&adj, order.iter());
    println!("Highest Degree Coloring: {c}");

    order.reverse();

    let (c, _) = greedy_coloring(&adj, order.iter());
    println!("Lowest Degree Coloring: {c}");

    let mut rng = rand::rng();

    order.shuffle(&mut rng);

    let (c, _) = greedy_coloring(&adj, order.iter());
    println!("Pseudo-Random Coloring: {c}");
}
