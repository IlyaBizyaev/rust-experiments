use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

/* Automaton state */
#[derive(Clone)]
struct State {
    len: usize,
    link: i32,
    next: BTreeMap<u8, usize>
}

impl State {
    fn new() -> Self {
        Self {
            len: 0,
            link: 0,
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
        automaton.states[0] = State {len: 0, link: -1, next: BTreeMap::new()};
        for &i in s.as_bytes() {
            automaton.add_character(i);
        }
        automaton
    }

    fn add_character(&mut self, c: u8) {
        let cur = self.size;
        self.size += 1;
        self.states[cur].len = &self.states[self.last].len + 1;
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
                    next: self.states[q].next.clone(),
                    link: self.states[q].link
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

    fn count_substrings(&self, d: &mut [u64]) -> u64 {
        let mut stack = Vec::new();
        stack.push(0usize);

        while !stack.is_empty() {
            let x = stack.last().unwrap().clone();
            let mut uncalced = false;
            for &state in self.states[x].next.values() {
                if d[state] == 0 {
                    uncalced = true;
                    stack.push(state);
                }
            }
            if !uncalced {
                stack.pop();
                d[x] = 1;
                for &state in self.states[x].next.values() {
                    d[x] += d[state]
                }
            }
        }
        d[0]
    }
}

fn main() {
    let fin = File::open("count.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let mut fout = File::create("count.out").expect("Failed to open output file");

    let mut s = String::new();
    reader.read_line(&mut s).expect("Failed to read the string");
    {
        let s_len = s.trim_right().len();
        s.truncate(s_len);
    }

    let automaton = SuffixAutomaton::new(&s);
    let mut d = vec![0u64; s.len() * 2];
    let result = automaton.count_substrings(&mut d) - 1;
    writeln!(fout, "{}", result).expect("Cannot write to file");
}
