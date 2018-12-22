use std::io;
use std::collections::BTreeMap;

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

/* Automaton state */
#[derive(Clone)]
struct State {
    len: usize,
    link: i32,
    back_char: u8,
    back_vertex: usize,
    next: BTreeMap<u8, usize>
}

impl State {
    fn new() -> Self {
        Self {
            len: 0,
            link: 0,
            back_char: '0' as u8,
            back_vertex: 0,
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
            len: 0, link: -1, back_char: '0' as u8, back_vertex: 0,
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
        self.states[cur].back_char = c;
        self.states[cur].back_vertex = self.last;
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
                    back_char: self.states[q].back_char,
                    back_vertex: self.states[q].back_vertex,
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

    fn collect_string(&self, v: usize, len: usize) -> String {
        let mut answer = String::new();
        let mut cur = v;
        for _ in 0..len {
            answer.push(self.states[cur].back_char as char);
            cur = self.states[cur].back_vertex;
        }
        answer.chars().rev().collect()
    }

    fn longest_common_substring(&self, perfect_path: usize) -> String {
        let mut special_symbol_path = vec![0usize; self.states.len()];
        let mut used = vec![false; self.states.len()];
        let mut stack = Vec::new();
        stack.push(0usize);
        let mut max_state = 0;
        let mut max_length = 0;
        while !stack.is_empty() {
            let x = stack.last().unwrap().clone();
            used[x] = true;

            let mut uncalced = false;
            for (&first, &second) in &self.states[x].next {
                if first >= '0' as u8 && first <= '9' as u8 {
                    special_symbol_path[x] |= 1 << first - '0' as u8;
                } else if !used[second] {
                    stack.push(second);
                    uncalced = true;
                }
            }

            if !uncalced {
                for &state in self.states[x].next.values() {
                    special_symbol_path[x] |= special_symbol_path[state];
                }
                if special_symbol_path[x] == perfect_path && self.states[x].len > max_length {
                    max_length = self.states[x].len;
                    max_state = x;
                }

                stack.pop();
            }
        }

        self.collect_string(max_state, max_length)
    }
}

fn main() {
    let k = parse_line!(usize).0;

    let mut t = String::new();
    let mut perfect_path = 0usize;
    for i in 0..k {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read a string");

        {
            let s_len = s.trim_right().len();
            s.truncate(s_len);
        }
        t += &(s + &i.to_string());
        perfect_path |= 1 << i;
    }

    let automaton = SuffixAutomaton::new(&t);

    let x = automaton.longest_common_substring(perfect_path);
    println!("{}", x);
}
