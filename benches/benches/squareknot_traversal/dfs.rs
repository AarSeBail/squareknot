use std::time::{Duration, Instant};

use benches::bench;
use criterion::{Criterion, criterion_group};
use rand::Rng;
use squareknot_graph::{AbstractGraph, SimpleGraph};
use squareknot_traversal::TraversalView;

criterion_group!(benches, traverse_component_1000);

fn gen_graph(nv: usize, p: f32) -> SimpleGraph {
    let mut g = SimpleGraph::empty(nv);
    for u in 0..nv {
        for v in 0..u {
            if rand::rng().random::<f32>() < p {
                g.add_edge(u, v);
            }
        }
    }
    g
}

fn traverse_component_1000(c: &mut Criterion) {
    let d = [0.1, 0.2, 0.4, 0.8];
    for p in d {
        let s = format!("traverse_component_1000_{p}p");
        c.bench_function(&bench!(s), |b| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _i in 0..iters {
                    let g = gen_graph(1000, p);
                    let start = Instant::now();
                    g.view().dfs(0).count();
                    total += start.elapsed();
                }
                total
            });
        });
    }
}
