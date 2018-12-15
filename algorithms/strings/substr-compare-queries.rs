use std::io;

static P: u64 = 31;

macro_rules! parse_line {
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )+
        )
    })
}

fn calculate_powers(n: usize) -> Vec<u64> {
    let mut result: Vec<u64> = vec![0; n];
    result[0] = 1;
    for i in 1..n {
        result[i] = result[i - 1].wrapping_mul(P);
    }
    result
}

fn calculate_hashes(s: &[u8]) -> Vec<u64> {
    let n = s.len();
    let mut result = vec![0; n];
    for i in 0..n {
        result[i] = u64::from(s[i] - ('a' as u8) + 1);
        if i != 0 {
            result[i] = result[i].wrapping_add(result[i - 1].wrapping_mul(P));
        }
    }
    result
}

fn get_hash(hashes: &[u64], l: usize, r: usize, p: &[u64]) -> u64 {
    let mut result = hashes[r];
    if l != 0 {
        result = result.wrapping_sub(hashes[l - 1].wrapping_mul(p[r - l + 1]))
    }
    result
}

fn check_equal(s: &str, s_hashes: &[u64], a: usize, b: usize, c: usize, d: usize, p: &[u64]) -> bool {
    if get_hash(&s_hashes, a, b, &p) != get_hash(&s_hashes, c, d, &p) {
        return false;
    }
    s.as_bytes()[a] == s.as_bytes()[c]
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read string");
    s = s.trim_right().to_string();
    let powers = calculate_powers(s.len());
    let s_hashes = calculate_hashes(s.as_bytes());

    let m = parse_line!(u32).0;
    for _ in 0..m {
        let (a, b, c, d) = parse_line!(usize, usize, usize, usize);
        if check_equal(&s, &s_hashes, a - 1, b - 1, c - 1, d - 1, &powers) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
