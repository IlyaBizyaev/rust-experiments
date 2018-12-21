use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use std::cmp::{max, min};

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

/* Automaton state */
#[derive(Clone)]
struct State {
    len: usize,
    link: i32,
    is_clone: bool,
    first_pos: usize,
    next: BTreeMap<u8, usize>,
    inv_link: Vec<i32>
}

impl State {
    fn new() -> Self {
        Self {
            len: 0,
            link: 0,
            is_clone: false,
            first_pos: 0,
            next: BTreeMap::new(),
            inv_link: Vec::new()
        }
    }
}

/* Automaton */
struct SuffixAutomaton {
    states: Vec<State>,
    size: usize,
    last: usize
}

impl SuffixAutomaton {
    fn new(s: &str) -> Self {
        let mut automaton = Self {
            last: 0,
            size: 1,
            states: vec![State::new(); s.len() * 2]
        };
        automaton.states[0] = State {
            len: 0, link: -1, is_clone: false, first_pos: 0,
            next: BTreeMap::new(), inv_link: Vec::new()
        };
        for &i in s.as_bytes() {
            automaton.add_character(i);
        }
        automaton.calculate_inverse_links();
        automaton
    }

    fn add_character(&mut self, c: u8) {
        let cur = self.size;
        self.size += 1;
        self.states[cur].len = &self.states[self.last].len + 1;
        self.states[cur].first_pos = self.states[cur].len -  1;
        let mut p = self.last as i32;
        while p != -1 && !self.states[p as usize].next.contains_key(&c) {
            *self.states[p as usize].next.entry(c).or_default() = cur;
            p = self.states[p as usize].link;
        }

        if p != -1 {
            let q = self.states[p as usize].next[&c];
            if self.states[p as usize].len + 1 == self.states[q].len {
                self.states[cur].link = q as i32;
            } else {
                let clone = self.size;
                self.size += 1;
                self.states[clone] = State {
                    len: self.states[p as usize].len + 1,
                    is_clone: true,
                    first_pos: self.states[q].first_pos,
                    next: self.states[q].next.clone(),
                    link: self.states[q].link,
                    inv_link: self.states[q].inv_link.clone()
                };
                while p != -1 && self.states[p as usize].next[&c] == q {
                    *self.states[p as usize].next.entry(c).or_default() = clone;
                    p = self.states[p as usize].link;
                }
                self.states[q].link = clone as i32;
                self.states[cur].link = clone as i32;
            }
        } else {
            self.states[cur].link = 0;
        }

        self.last = cur;
    }

    fn calculate_inverse_links(&mut self) {
        for i in 1..self.size {
            let link = self.states[i].link as usize;
            self.states[link].inv_link.push(i as i32);
        }
    }

    fn find_substring(&self, s: &str) -> i32 {
        let mut cur = 0;
        for i in s.as_bytes() {
            if !self.states[cur].next.contains_key(i) {
                return -1;
            }
            cur = self.states[cur].next[i];
        }
        cur as i32
    }

    fn find_border_locations(&self, t_len: usize, s: &str) -> (i32, i32) {
        let mut leftmost: i32 = -1; let mut rightmost: i32 = -1;
        if t_len < s.len() {
            return (-1, -1);
        }

        let v = self.find_substring(&s);
        let mut queue = vec![v];

        while !queue.is_empty() {
            let x = queue.pop().unwrap();
            if x == -1 {
                continue;
            }
            queue.extend(self.states[x as usize].inv_link.iter());
            let pos = self.states[x as usize].first_pos + 1 - s.len();
            if leftmost == -1 {
                leftmost = pos as i32;
            }
            leftmost = min(leftmost, pos as i32);
            rightmost = max(rightmost, pos as i32);
        }

        (leftmost, rightmost)
    }
}

fn main() {
    let fin = File::open("search6.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let fout = File::create("search6.out").expect("Failed to open output file");
    let mut writer = io::BufWriter::new(fout);

    let n = parse_line!(reader, usize).0;
    let mut queries = Vec::new();

    for _ in 0..n {
        let mut s = String::new();
        reader.read_line(&mut s).expect("Failed to read a query");
        {
            let s_len = s.trim_right().len();
            s.truncate(s_len);
        }
        queries.push(s);
    }
    let mut t = String::new();
    reader.read_line(&mut t).expect("Failed to read the string");
    {
        let t_len = t.trim_right().len();
        t.truncate(t_len);
    }
    let automaton = SuffixAutomaton::new(&t);

    for s in &queries {
        let (l, r) = automaton.find_border_locations(t.len(), s);
        writeln!(writer, "{} {}", l, r).expect("Cannot write to file");
    }
}
