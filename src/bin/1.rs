use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    for i in 0..t {
        handle.read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<usize>().unwrap();
        buf.clear();
        let mut matrix = Vec::with_capacity(n);
        for _ in 0..n {
            handle.read_line(&mut buf).unwrap();
            matrix.push(
                buf.trim()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            );
            buf.clear();
        }

        let mut trace = 0;
        let mut r = 0;
        let mut c = 0;
        for j in 0..n {
            trace += matrix[j][j];

            let mut r_set = HashSet::new();
            let mut c_set = HashSet::new();

            for k in 0..n {
                r_set.insert(matrix[j][k]);
                c_set.insert(matrix[k][j]);
            }

            if r_set.len() != n {
                r += 1;
            }
            if c_set.len() != n {
                c += 1;
            }
        }

        println!("Case #{}: {} {} {}", i + 1, trace, r, c);
    }
}
