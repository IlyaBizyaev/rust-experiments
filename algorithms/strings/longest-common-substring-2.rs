use std::io;
use std::cmp::max;
use std::iter::ExactSizeIterator;
use std::collections::HashSet;
use std::mem::swap;
use std::fs::File;
use std::io::prelude::*;

// static MODULE: u64 = 1000000007;
static P: u64 = 31;

fn binary_search_by<I, F>(iter: I, mut f: F) -> Result<usize, usize>
        where F: FnMut(&<I as Iterator>::Item) -> bool,
              I: ExactSizeIterator + Clone,
{
        let mut size = iter.len();
        if size == 0 {
            return Err(0);
        }
        let mut base: usize = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(&iter.clone().nth(mid).unwrap());
            base = if !cmp { base } else { mid };
            size -= half;
        }
        let cmp = f(&iter.clone().nth(base).unwrap());
        if cmp { Ok(base) } else { Err(base + cmp as usize) }
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

fn check_match(i: usize, s_hashes: &[u64], t_hashes: &[u64], p: &[u64]) -> bool {
    if i == 0 {
        return true;
    }
    let mut hashes = HashSet::new();
    for j in 0..=(s_hashes.len() - i) {
        hashes.insert(get_hash(s_hashes, j, j + i - 1, p));
    }
    for j in 0..=(t_hashes.len() - i) {
        let hash = get_hash(t_hashes, j, j + i - 1, p);
        if hashes.contains(&hash) {
            return true;
        }
    }
    false
}

fn find_lexicographically_min<'a>
(t: &'a str, len: usize, s_hashes: &[u64], t_hashes: &[u64], p: &[u64]) -> &'a str {
    if len == 0 {
        return "";
    }
    let mut hashes = HashSet::new();
    for j in 0..=(s_hashes.len() - len) {
        hashes.insert(get_hash(s_hashes, j, j + len - 1, p));
    }
    let mut result = &t[0..0];
    let mut first = true;
    for j in 0..=(t_hashes.len() - len) {
        let hash = get_hash(t_hashes, j, j + len - 1, p);
        if hashes.contains(&hash) {
            if &t[j..j + len] < result || first {
                result = &t[j..j + len];
                first = false;
            }
        }
    }
    result
}

fn main() {
    let fin = File::open("common.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let mut fout = File::create("common.out").expect("Failed to open output file");

    let (mut s, mut t) = (String::new(), String::new());
    reader.read_line(&mut s).expect("Failed to read s");
    reader.read_line(&mut t).expect("Failed to read t");
    s = s.trim_right().to_string();
    t = t.trim_right().to_string();
    if s.len() > t.len() {
        swap(&mut s, &mut t);
    }
    let powers = calculate_powers(max(s.len(), t.len()));
    let s_hashes = calculate_hashes(s.as_bytes());
    let t_hashes = calculate_hashes(t.as_bytes());
    let index = binary_search_by(0..s.len() + 1,
                                 |x| check_match(*x, &s_hashes, &t_hashes, &powers)).unwrap();
    let result = find_lexicographically_min(&t, index, &s_hashes, &t_hashes, &powers);
    writeln!(fout, "{}", result).expect("Cannot write to file");
}
