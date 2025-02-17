use crate::graph::AbstractGraph;

pub fn shortest_path<G: AbstractGraph>(g: &G, u: usize, v: usize) -> Option<Vec<usize>> {
    let mut bfs = g.bfs(u);
    // TODO: Test Performance
    // bfs.add_depth_map();
    for node in bfs.by_ref() {
        if node.vertex == v {
            break;
        }
    }

    bfs.extract_path(v)
}
