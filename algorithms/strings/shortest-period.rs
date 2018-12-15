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
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read a string");
    let n = s.trim_right().len();
    s.truncate(n);
    let result = z_function(s.as_bytes());
    for i in 1..n {
        if n % i == 0 && result[i] + i == n {
            println!("{}", i);
            return;
        }
    }
    println!("{}", n);
}
