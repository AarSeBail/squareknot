use squareknot::prelude::*;

// This example demonstrates some of the core features of the library
fn main() {
    // Graph construction
    let n = 7;
    let mut graph: SimpleGraph = SimpleGraph::empty(n);

    for u in 1..n {
        for v in 0..u {
            graph.add_edge(u, v);
        }
    }

    // This creates a `GraphView`, which is essentially an immutable reference to a graph that implements `ViewCombinator`
    let view = graph.view();

    // ViewCombinators may be used to create new view combinators, which may be seen as modifications of a view of a graph
    // This is an example of a view into the subgraph with even numbered vertices
    // Doing this consumes the view. A .by_ref() method may help with this in the future
    let even_subgraph = view.filter_vertices(|v| v & 1 == 0);

    // Views of graphs may be traversed
    // The .bfs/.dfs/.full_bfs/.full_dfs methods provide iterators over the graph
    // These use preorder traversal at the moment
    // Note: This currently panics when the root vertex is not contained in the graph.
    let bfs = even_subgraph.bfs(0);

    // These iterators yield a node type, `TraversalNode`, with depth and vertex fields
    let comp_size = bfs.count();

    println!("Even Component Size: {comp_size}");

    // Edges may also be filtered
    // The following creates a ViewCombinator in which edges only exist between vertices with the same parity
    let split_subgraph = graph.view().filter_edges(|(u, v)| u & 1 == v & 1);

    // The .full_bfs/.full_dfs methods are methods which restart at the next unvisited node after finishing
    // traversing a component.
    // The node depth is 0 if and only if a new component has started to be traversed
    let comp_count = split_subgraph.full_bfs().filter(|node| node.depth == 0).count();
    println!("Even/Odd Component Count: {comp_count}");

    
}