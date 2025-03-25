use squareknot::{prelude::*, io::{dimacs::FakeDimacs, GraphFormat}};

fn main() {
    let g = "10 5\n0 1\n1 2\n0 3\n3 4";
    // &[u8] implements BufRead, which is required by parse_graph()
    let graph: SimpleGraph = FakeDimacs::parse_graph(g.as_bytes()).expect("Could not parse graph.");

    let view = graph.view();
    println!("BFS");
    for node in view.bfs(0) {
        println!("vertex: {}, depth: {}", node.vertex, node.depth);
    }

    println!("DFS");
    for node in view.dfs(0) {
        println!("vertex: {}, depth: {}", node.vertex, node.depth);
    }
}