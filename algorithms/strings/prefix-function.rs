use std::io;

fn prefix_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut p = vec![0; n];
    for i in 1..n {
        let mut j = p[i - 1];
        while j > 0 && s[i] != s[j] {
            j = p[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        p[i] = j;
    }
    p
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read a string");
    let n = s.trim_right().len();
    s.truncate(n);
    let result = prefix_function(s.as_bytes());
    for x in result {
        print!("{} ", x);
    }
}
