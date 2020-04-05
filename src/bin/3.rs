use std::cmp::Ordering;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<usize>().unwrap();
    buf.clear();

    'case: for i in 0..t {
        handle.read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<usize>().unwrap();
        buf.clear();
        let mut acts = Vec::with_capacity(n);
        for j in 0..n {
            handle.read_line(&mut buf).unwrap();
            {
                let mut it = buf.trim().split_whitespace();
                let start = it.next().unwrap().parse::<usize>().unwrap();
                let end = it.next().unwrap().parse::<usize>().unwrap();
                acts.push((start, end, j));
            }
            buf.clear();
        }

        acts.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            otherwise => otherwise,
        });

        let mut ans = vec!['?'; n];
        let mut c_end = 0;
        let mut j_end = 0;
        for (s, e, idx) in acts {
            if s >= c_end {
                ans[idx] = 'C';
                c_end = e;
            } else if s >= j_end {
                ans[idx] = 'J';
                j_end = e;
            } else {
                println!("Case #{}: {}", i + 1, "IMPOSSIBLE");
                continue 'case;
            }
        }
        println!("Case #{}: {}", i + 1, ans.into_iter().collect::<String>());
    }
}
