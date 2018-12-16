use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

static ALPHABET: usize = 256;

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
        c.copy_from_slice(&cn);
        h += 1;
    }
    p
}

fn kasai(s: &[u8], suffix_array: &[usize]) -> Vec<usize> {
    let n = suffix_array.len();
    let mut lcp = vec![0; n];
    let mut inv_suffix = vec![0; n];

    for i in 0..n {
        inv_suffix[suffix_array[i]] = i;
    }

    let mut k = 0;

    for i in 0..n {
        if inv_suffix[i] == n - 1 {
            k = 0;
            continue;
        }

        let j = suffix_array[inv_suffix[i] + 1];

        while i + k < n && j + k < n && s[i + k] == s[j + k] {
            k += 1;
        }
        lcp[inv_suffix[i]] = k;
        if k > 0 {
            k -= 1;
        }
    }
    lcp
}

fn main() {
    let fin = File::open("array.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let fout = File::create("array.out").expect("Failed to open output file");
    let mut writer = io::BufWriter::new(fout);

    let mut s = String::new();
    reader.read_line(&mut s).expect("Failed to read the string");
    {
        let s_len = s.trim_right().len();
        s.truncate(s_len);
        s += "$";
    }

    let suffix_array = build_suffix_array(&s.as_bytes());
    let lcp = kasai(&s[..s.len()-1].as_bytes(), &suffix_array[1..]);
    for &x in &suffix_array[1..] {
        write!(writer, "{} ", x + 1).expect("Cannot write to file");
    }
    writeln!(writer).unwrap();
    for i in 0..lcp.len() - 1 {
        write!(writer, "{} ", lcp[i]).expect("Cannot write to file");
    }
    writeln!(writer).unwrap();
}
