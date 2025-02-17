use std::collections::HashSet;

fn main() {
    let n: usize = 1000;
    let p: usize = 5;
    let q: usize = 1000;
    let m: usize = p * n * (n - 1) / (2 * q);

    let mut unused = HashSet::new();
    for u in 0..n {
        for v in 0..u {
            unused.insert((u, v));
        }
    }

    println!("{n} {m}");

    for (u, v) in unused.iter().take(m) {
        println!("{} {}", u, v);
    }
}
