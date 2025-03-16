use squareknot::{prelude::*, io::{dimacs::FakeDimacs, *}};

#[test]
fn quotient_map() {
    let g = "10 5\n0 1\n1 2\n2 3\n3 4\n5 6";
    let graph: SimpleGraph = FakeDimacs::parse_graph(g.as_bytes()).expect("Could not parse graph.");

    let relation = [(0, 1), (2, 3)];

    let mut map = (0..graph.num_v_labels()).collect::<Vec<_>>();

    for (u, v) in relation {
        map[v] = map[u];
    }

    let quotient = graph.map_vertices(|v| map[v]);

    let r = quotient.edge_iterator().collect::<Vec<_>>();
    let v = quotient.vertex_iterator().collect::<Vec<_>>();

    assert!(!v.contains(&1));
    assert!(!v.contains(&3));
    assert!(r.contains(&(0, 2)) || r.contains(&(2, 0)));
    assert!(r.contains(&(2, 4)) || r.contains(&(4, 2)));
    assert!(!r.contains(&(0, 0)));
    assert!(!r.contains(&(2, 2)));
}