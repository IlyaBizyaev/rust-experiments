use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::BTreeSet;

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

#[derive(Debug)]
struct Task {deadline: u32, fine: u32}

fn main() {
    let fin = File::open("schedule.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let mut fout = File::create("schedule.out").expect("Failed to open output file");

    let n = parse_line!(reader, u32).0;
    let mut tasks = Vec::new();
    let mut times = BTreeSet::new();
    for i in 0..n {
        let (d, w) = parse_line!(reader, u32, u32);
        tasks.push(Task {deadline: d, fine: w});
        times.insert(i);
    }
    tasks.sort_unstable_by(|a, b| b.fine.cmp(&a.fine));

    let mut result = 0u64;
    for t in tasks.iter() {
        let c = times.range(..t.deadline).next_back().cloned();
        if c != None {
            times.remove(&c.unwrap());
        } else {
            result += t.fine as u64;
        }
    }

    writeln!(fout, "{}", result).expect("Cannot write to file");
}
