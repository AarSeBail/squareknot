use squareknot_core::{AbstractGraph, SimpleGraph};
use squareknot_io::{dimacs::FakeDimacs, GraphFormat};
use squareknot_traversal::BFSGraph;

#[test]
fn comp_count() {
    let g = "10 5\n0 1\n1 2\n2 3\n3 4\n5 6";
    // &[u8] implements BufRead, which is required by parse_graph()
    let graph: SimpleGraph =
        FakeDimacs::parse_graph(g.as_bytes()).expect("Could not parse graph.");

    let dfs = graph.full_bfs();
    let ncomp = dfs
        .map(|x| {
            println!("{}", x.depth);
            x
        })
        .filter(|n| n.depth == 0)
        .count();
    println!("{}", graph.order());
    println!("Component Count: {ncomp}");
    assert_eq!(ncomp, 6);
}
