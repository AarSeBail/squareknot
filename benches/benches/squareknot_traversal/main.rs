use criterion::criterion_main;

mod bfs;
mod dfs;

criterion_main!(
    bfs::benches,
    dfs::benches
);
