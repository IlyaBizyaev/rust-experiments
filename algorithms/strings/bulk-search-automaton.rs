use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

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
}

impl State {
    fn new() -> Self {
        Self {
            len: 0,
            link: 0,
            is_clone: false,
            first_pos: 0,
            next: BTreeMap::new()
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
            next: BTreeMap::new()
        };
        for &i in s.as_bytes() {
            automaton.add_character(i);
        }
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

    fn find_substring(&self, s: &str) -> bool {
        let mut cur = 0;
        for i in s.as_bytes() {
            if !self.states[cur].next.contains_key(i) {
                return false;
            }
            cur = self.states[cur].next[i];
        }
        true
    }
}

fn main() {
    let fin = File::open("search4.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let fout = File::create("search4.out").expect("Failed to open output file");
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
        let result = automaton.find_substring(s);
        writeln!(writer, "{}", if result { "YES"} else {"NO"}).expect("Cannot write to file");
    }
}
