// This example is to illustrate how view combinators and dfs may be used to find Kempe chains.

use std::io::{stdout, Write};

use rand::{seq::IndexedRandom, Rng};
use squareknot::{
    graph::{AbstractGraph, SimpleGraph, ViewCombinator},
    prelude::TraversalView,
};

const N: usize = 40;

fn find_kempe_chains(graph: &SimpleGraph, coloring: &[u8], colors: &[u8]) {
    let mut lock = stdout().lock();
    for a in 1..colors.len() {
        for b in 0..a {
            let x = colors[a];
            let y = colors[b];
            println!("Kempe chains with colors {y} and {x}");
            let subgraph = graph
                .view()
                .filter_vertices(|&v| coloring[v] == x || coloring[v] == y);

            let mut started: bool = false;

            for node in subgraph.full_dfs() {
                if node.depth == 0 {
                    if started {
                        writeln!(&mut lock, "]").unwrap();
                    }
                    write!(&mut lock, "[").unwrap();
                }
                started = true;
                write!(&mut lock, "{}, ", node.vertex).unwrap();
            }
            if started {
                writeln!(&mut lock, "]").unwrap();
            }
        }
    }
}

fn main() {
    let mut graph = SimpleGraph::empty(N);

    let mut rng = rand::rng();

    for u in 1..N {
        for v in 0..u {
            if rng.random::<f64>() < 0.2 {
                graph.add_edge(u, v);
            }
        }
    }

    let colors = [1, 2, 3, 4, 5];
    let mut coloring = vec![0; N];

    for i in 0..N {
        coloring[i] = *colors.choose(&mut rng).unwrap();
    }

    find_kempe_chains(&graph, &coloring, &colors);
}
