use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max, min};

static ALPHABET: usize = 256;

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

fn get_period(s: &[u8]) -> usize {
    let result = z_function(s);
    let n = s.len();
    for i in 1..n {
        if n % i == 0 && result[i] + i == n { ;
            return i;
        }
    }
    s.len()
}

fn build_suffix_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let max_len = max(n, ALPHABET);
    let mut p = vec![0; n];
    let mut cnt = vec![0; max_len];
    let mut c = vec![0; max_len];

    for &i in s {
        cnt[i as usize] += 1;
    }
    for i in 1..ALPHABET {
        cnt[i] += cnt[i - 1];
    }
    for i in 0..n {
        cnt[s[i] as usize] -= 1;
        p[cnt[s[i] as usize]] = i;
    }
    c[p[0]] = 0;
    let mut classes = 1usize;
    for i in 1..n {
        if s[p[i]] != s[p[i - 1]] {
            classes += 1;
        }
        c[p[i]] = classes - 1;
    }

    let mut pn = vec![0; max_len];
    let mut cn = vec![0; max_len];
    let mut h = 0;
    while (1 << h) < n {
        for i in 0..n {
            pn[i] = {
                let mut l = p[i] as i32 - (1 << h) as i32;
                if l < 0 {
                    l += n as i32;
                }
                l as usize
            };
        }
        let mut cnt = vec![0; classes];
        for i in 0..n {
            cnt[c[pn[i]]] += 1;
        }
        for i in 1..classes {
            cnt[i] += cnt[i - 1];
        }
        for i in (0..n).rev() {
            cnt[c[pn[i]]] -= 1;
            p[cnt[c[pn[i]]]] = pn[i];
        }
        cn[p[0]] = 0;
        classes = 1;
        for i in 1..n {
            let mid1 = (p[i] + (1 << h)) % n;
            let mid2 = (p[i - 1] + (1 << h)) % n;
            if c[p[i]] != c[p[i - 1]] || c[mid1] != c[mid2] {
                classes += 1;
            }
            cn[p[i]] = classes - 1;
        }
        c = cn.clone();
        h += 1;
    }
    p
}

fn main() {
    let fin = File::open("shifts.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let mut fout = File::create("shifts.out").expect("Failed to open output file");

    let mut s = String::new();
    reader.read_line(&mut s).expect("Failed to read the string");
    {
        let s_len = s.trim_right().len();
        s.truncate(s_len);
    }
    let k = parse_line!(reader, usize).0;

    let period = get_period(s.as_bytes());
    if k > period {
        writeln!(fout, "IMPOSSIBLE").expect("Cannot write to file");
    } else {
        let suffix_array = build_suffix_array(&s[..period].as_bytes());
        let index = suffix_array[k - 1];
        write!(fout, "{}", &s[index..]).expect("Cannot write to file");
        write!(fout, "{}", &s[..index]).expect("Cannot write to file");
    }
}
