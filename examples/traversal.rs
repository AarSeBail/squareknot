use squareknot::{
    io::{dimacs::FakeDimacs, GraphFormat},
    prelude::*,
};

fn main() {
    let g = "10 7\n0 1\n1 2\n0 3\n3 4\n2 4\n4 5\n5 6";
    // &[u8] implements BufRead, which is required by parse_graph()
    let graph: SimpleGraph = FakeDimacs::parse_graph(g.as_bytes()).expect("Could not parse graph.");

    let view = graph.view();
    println!("BFS");
    for node in view.bfs(0) {
        println!("vertex: {}, depth: {}", node.vertex, node.depth);
    }

    println!("DFS Pre-Order");
    for node in view.dfs(0) {
        println!("vertex: {}, depth: {}", node.vertex, node.depth);
    }

    println!("DFS Post-Order");
    for node in view.dfs_post_order(0) {
        println!("vertex: {}, depth: {}", node.vertex, node.depth);
    }

    println!("Full DFS Pre-Order");
    for node in view.full_dfs() {
        println!("vertex: {}, depth: {}", node.vertex, node.depth);
    }

    println!("Full DFS Post-Order");
    for node in view.full_dfs_post_order() {
        println!("vertex: {}, depth: {}", node.vertex, node.depth);
    }
}
