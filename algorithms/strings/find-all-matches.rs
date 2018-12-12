use std::io;
use std::cmp::min;

fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i <= r {
            z[i] = min(r - i + 1, z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i; r = i + z[i] - 1;
        }
    }
    z
}

fn main() {
    let (mut s, mut t) = (String::new(), String::new());
    io::stdin().read_line(&mut s).expect("Failed to read s");
    io::stdin().read_line(&mut t).expect("Failed to read t");
    let n = s.trim_right().len();
    s.truncate(n);
    let m = t.trim_right().len();
    t.truncate(m);
    s  = s + "$" + &t;
    let z = z_function(s.as_bytes());
    let mut result = Vec::new();
    for (i, &x) in z[1..].iter().enumerate() {
        if x == n {
            result.push(i + 1 - n);
        }
    }
    println!("{}", result.len());
    for x in result {
        print!("{} ", x);
    }
}
