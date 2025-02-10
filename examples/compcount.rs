use std::{fs::File, io::{BufRead, BufReader}};

use itertools::Itertools;
use squareknot::{graph::{AbstractGraph, SimpleGraph}, prelude::depth_first::DFSTraversal};

// Iterator over pairs of numbers
struct PairIterator<B: BufRead> {
    reader: B,
    buffer: String
}

impl<B: std::io::BufRead> Iterator for PairIterator<B> {
    type Item = (usize, usize);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        if let Ok(_) = self.reader.read_line(&mut self.buffer) {
            self.buffer
                .split_whitespace()
                .map(|s| s.parse::<usize>().ok())
                .take(2).filter_map(|n| n).collect_tuple::<(usize, usize)>()
        }else {
            None
        }
    }
}

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

    let mut g = SimpleGraph::with_vertices(nv);

    for (u, v) in pairs {
        g.add_edge(u, v);
    }

    let dfs = DFSTraversal::full_traversal(&g);

    println!("Num Components: {}", dfs.filter(|node| node.depth == 0).count());
}