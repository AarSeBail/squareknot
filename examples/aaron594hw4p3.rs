use std::{fs::File, io::BufReader};

use rand::seq::SliceRandom;
use squareknot::{
    algorithms::coloring::greedy_coloring,
    graph::{AbstractGraph, SimpleGraph},
    io::{dimacs::DimacsFormat, GraphFormat},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        std::process::exit(1);
    }

    let f = File::open(&args[1]).expect("Could not open input file");
    let reader = BufReader::with_capacity(1024 * 64, f);

    let graph: SimpleGraph = DimacsFormat::parse_graph(reader).expect("Could not parse graph.");

    println!("Loaded Graph.");

    let mut ord: Vec<_> = (0..graph.order()).map(|v| (graph.degree(v), v)).collect();
    ord.sort_by_key(|&(d, _)| d);
    let mut order = ord.iter().map(|&(_, v)| v).collect::<Vec<_>>();

    let (c, _) = greedy_coloring(&graph, order.iter());
    println!("Highest Degree Coloring: {c}");

    order.reverse();

    let (c, _) = greedy_coloring(&graph, order.iter());
    println!("Lowest Degree Coloring: {c}");

    let mut rng = rand::rng();

    order.shuffle(&mut rng);

    let (c, _) = greedy_coloring(&graph, order.iter());
    println!("Pseudo-Random Coloring: {c}");
}
