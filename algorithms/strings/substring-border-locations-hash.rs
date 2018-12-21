use std::io;
use std::fs::File;
use std::io::prelude::*;

static P: u64 = 31;

macro_rules! parse_line {
    ($x:ident, $($t: ty),+) => ({
        let mut a_str = String::new();
        $x.read_line(&mut a_str).expect("read error");
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

fn calculate_hash(s: &[u8]) -> u64 {
    let n = s.len();
    let mut prev: u64;
    let mut result = 0;
    for i in 0..n {
        prev = result;
        result = u64::from(s[i] - ('a' as u8) + 1);
        result = result.wrapping_add(prev.wrapping_mul(P));
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

fn main() {
    let fin = File::open("search6.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let mut fout = File::create("search6.out").expect("Failed to open output file");

    let n = parse_line!(reader, u32).0;
    let mut list = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        reader.read_line(&mut s).expect("Failed to read s_i");
        {
            let s_len = s.trim_right().len();
            s.truncate(s_len);
        }
        list.push(s);
    }
    let mut t = String::new();
    reader.read_line(&mut t).expect("Failed to read t");
    {
        let t_len = t.trim_right().len();
        t.truncate(t_len);
    }

    let powers = calculate_powers(t.len());
    let t_hashes = calculate_hashes(t.as_bytes());

    for s in list {
        let s_hash = calculate_hash(s.as_bytes());

        let mut l = t.len() as i32;
        let mut r = -1i32;
        for j in 0..=(t.len() - s.len()) as i32 {
            if get_hash(&t_hashes, j as usize, j as usize + s.len() - 1, &powers) == s_hash {
                if j < l { l = j; }
                if j > r { r = j; }
            }
        }

        if r != -1 {
            write!(fout, "{} {}", l, r).expect("write failed");
        } else {
            write!(fout, "-1 -1").expect("write failed");
        }
        writeln!(fout).expect("write failed");
    }
}
