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
        let ch: Vec<char> = buf.trim().chars().collect();
        let mut ans = String::new();
        let mut cur_depth = 0;
        for c in ch {
            let val = c as u32 - '0' as u32;
            while val > cur_depth {
                ans.push('(');
                cur_depth += 1;
            }
            while val < cur_depth {
                ans.push(')');
                cur_depth -= 1;
            }
            ans.push(c);
        }
        for _ in 0..cur_depth {
            ans.push(')');
        }

        println!("Case #{}: {}", i + 1, ans);
        buf.clear();
    }
}
