use std::{
    ffi::OsStr,
    fs::{read_link, File},
    io::{stdout, BufReader, BufWriter, Write},
};

use squareknot::{
    graph::{AbstractGraph, SimpleGraph},
    io::{dimacs::DimacsFormat, GraphFormat},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input> [disable paths]", args[0]);
        std::process::exit(1);
    }

    // Test if stdout is being redirected to `/dev/null`
    let should_print =
        read_link("/dev/fd/1").expect("Could not read stdout") != OsStr::new("/dev/null");
    
    let lock = stdout().lock();
    let mut writer = BufWriter::with_capacity(1024 * 64, lock);

    let f = File::open(&args[1]).expect("Could not open input file");
    let reader = BufReader::with_capacity(1024 * 64, f);

    let graph: SimpleGraph = DimacsFormat::parse_graph(reader).expect("Could not parse graph.");

    writeln!(writer, "Loaded Graph.").expect("Could not print.");

    let nv = graph.order();
    if nv == 0 {
        return;
    }

    let mut distances = vec![0; nv * nv];

    let mut bfs = graph.bfs(0);

    // It's actually faster without the depth map
    // bfs.add_depth_map();

    writeln!(writer, "Shortest Paths:").expect("Could not print.");
    for u in 0..nv {
        bfs.restart_at(u);
        bfs.by_ref().for_each(|_| {});
        for v in 0..=u {
            if let Some(path) = bfs.extract_path(v) {
                if should_print {
                    writeln!(writer, "{u} -> {v}: {:?}", path).expect("Could not print.");
                }
                distances[nv * u + v] = path.len() as i64;
                distances[nv * v + u] = path.len() as i64;
            }
        }
    }

    if should_print {
        writeln!(writer, "Distances:").expect("Could not print.");
        for u in 0..nv {
            for v in 0..nv {
                if distances[nv * u + v] > 0 {
                    write!(writer, "{} ", distances[nv * u + v] - 1).expect("Could not print.");
                } else {
                    write!(writer, "- ").expect("Could not print.");
                }
            }
            write!(writer, "\n").expect("Could not print.");
        }
    }
}

