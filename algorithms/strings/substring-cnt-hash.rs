use std::io;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

static P: u64 = 31;

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

fn count_substrings(s_hashes: &[u64], p: &[u64]) -> usize {
    let mut result = 0;
    for len in 1..=s_hashes.len() {
        let mut hashes = HashSet::new();

        for j in 0..=(s_hashes.len() - len) {
            hashes.insert(get_hash(s_hashes, j, j + len - 1, p));
        }
        result += hashes.len();
    }
    result
}

fn main() {
    let fin = File::open("count.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let mut fout = File::create("count.out").expect("Failed to open output file");

    let mut s = String::new();
    reader.read_line(&mut s).expect("Failed to read the string");
    s = s.trim_right().to_string();
    let powers = calculate_powers(s.len());
    let s_hashes = calculate_hashes(s.as_bytes());
    let result = count_substrings(&s_hashes, &powers);
    writeln!(fout, "{}", result).expect("Cannot write to file");
}
