use rand::Rng;
use squareknot::prelude::{depth_first::DFSTraversal, *};

fn main() {
    let n = 50000;
    let t = 0.99998;
    // let mut g = SimpleGraph::with_vertices(n);

    use std::time::Instant;
    let now = Instant::now();

    let mut g = SimpleGraph::with_vertices(n);
    
    let mut rng = rand::rng();

    for u in 1..n {
        for v in 0..u {
            if rng.random::<f64>() > t {
                g.add_edge(u, v);
            }
        }
    }

    let dfs = DFSTraversal::from_root(&g, 0);

    println!("Component Size: {}", dfs.count());

    let dfs = DFSTraversal::full_traversal(&g);

    println!("Num Components: {}", dfs.filter(|node| node.depth == 0).count());
}