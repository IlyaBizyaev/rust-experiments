use std::io;
use std::fs::File;
use std::io::prelude::*;

const ALPHABET: usize = 26;

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

/* Trie node */
#[derive(Clone)]
struct Node {
    next: [i32; ALPHABET],
    auto_move: [i32; ALPHABET],
    link: i32,
    fast_link: i32,
    pattern_indexes: Vec<usize>,
    parent: usize,
    is_leaf: bool,
    character: u8
}

impl Node {
    fn new(c: u8, p: usize) -> Self {
        Self {
            next: [-1; ALPHABET],
            auto_move: [-1; ALPHABET],
            link: -1,
            fast_link: -1,
            pattern_indexes: Vec::new(),
            parent: p,
            is_leaf: false,
            character: c
        }
    }
}

/* Trie */
struct Trie {
    states: Vec<Node>,
    patterns: Vec<String>,
    positions: Vec<i32>
}

impl Trie {
    fn new(n: usize) -> Self {
        Self {
            states: vec![Node::new(b'$', 0); 1],
            patterns: Vec::new(),
            positions: vec![-1; n]
        }
    }

    fn add_string(&mut self, s: &str) {
        let mut v = 0;
        for i in 0..s.len() {
            let ch = s.as_bytes()[i] - b'a';
            if self.states[v].next[ch as usize] == -1 {
                self.states.push(Node::new(ch, v));
                self.states[v].next[ch as usize] = (self.states.len() - 1) as i32;
            }
            v = self.states[v].next[ch as usize] as usize;
        }
        self.states[v].is_leaf = true;
        self.patterns.push(s.to_owned());
        self.states[v].pattern_indexes.push(self.patterns.len() - 1);
    }

    fn add_string_rev(&mut self, s: &str, i: usize) {
        let mut v = 0;
        for i in (0..s.len()).rev() {
            let ch = s.as_bytes()[i] - b'a';
            if self.states[v].next[ch as usize] == -1 {
                self.states.push(Node::new(ch, v));
                self.states[v].next[ch as usize] = (self.states.len() - 1) as i32;
            }
            v = self.states[v].next[ch as usize] as usize;
        }
        self.states[v].is_leaf = true;
        self.states[v].pattern_indexes.push(i);
    }

    fn go(&mut self, v: usize, c: u8) -> i32 {
        if self.states[v].auto_move[c as usize] == -1 {
            if self.states[v].next[c as usize] != -1 {
                self.states[v].auto_move[c as usize] = self.states[v].next[c as usize];
            } else if v == 0 {
                self.states[v].auto_move[c as usize] = 0;
            } else {
                let link = self.get_link(v) as usize;
                let mv = self.go(link, c);
                self.states[v].auto_move[c as usize] = mv;
            }
        }
        self.states[v].auto_move[c as usize]
    }

    fn get_link(&mut self, v: usize) -> i32 {
        if self.states[v].link == -1 {
            if v == 0 || self.states[v].parent == 0 {
                self.states[v].link = 0;
            } else {
                let parent = self.states[v].parent;
                let link = self.get_link(parent) as usize;
                let character = self.states[v].character;
                self.states[v].link = self.go(link, character);
            }
        }
        self.states[v].link
    }

    fn get_fast_link(&mut self, v: usize) -> i32 {
        if self.states[v].fast_link == -1 {
            let u = self.get_link(v);
            if u == 0 {
                self.states[v].fast_link = 0;
            } else {
                self.states[v].fast_link = if self.states[u as usize].is_leaf {
                    u
                } else {
                    self.get_fast_link(u as usize)
                };
            }
        }
        self.states[v].fast_link
    }

    fn check(&mut self, v: usize, i: usize) {
        let mut u = v;
        while u != 0 {
            if self.states[u].is_leaf {
                while !self.states[u].pattern_indexes.is_empty() {
                    let last_pattern = *self.states[u].pattern_indexes.last().unwrap();
                    self.positions[last_pattern] = (i - self.patterns[last_pattern].len()) as i32;
                    self.states[u].pattern_indexes.pop();
                }
            }
            u = self.get_fast_link(u) as usize;
        }
    }

    fn find_first_positions(&mut self, s: &str) {
        let mut u = 0usize;
        for i in 0..s.len() {
            u = self.go(u, s.as_bytes()[i] - b'a') as usize;
            self.check(u, i + 1);
        }
    }

    fn find_first_positions_rev(&mut self, s: &str) {
        let mut u = 0usize;
        for i in (0..s.len()).rev() {
            u = self.go(u, s.as_bytes()[i] - b'a') as usize;
            self.check(u, s.len() - i);
        }
    }
}

fn main() {
    let fin = File::open("search6.in").expect("Failed to open input file");
    let mut reader = io::BufReader::new(fin);
    let fout = File::create("search6.out").expect("Failed to open output file");
    let mut writer = io::BufWriter::new(fout);

    let n = parse_line!(reader, usize).0;
    let mut trie = Trie::new(n);

    let mut queries: Vec<String> = reader.lines().take(n + 1).map(|x| x.unwrap()).collect();
    let t = queries.pop().unwrap();
    for s in &queries {
        trie.add_string(s);
    }

    trie.find_first_positions(&t);
    let first_positions = trie.positions;
    trie.positions = vec![-1; n];
    trie.states = vec![Node::new(b'$', 0); 1];

    for (i, q) in queries.iter().enumerate() {
        trie.add_string_rev(q, i);
    }
    trie.find_first_positions_rev(&t);

    for i in 0..n {
        write!(writer, "{} ", first_positions[i]).unwrap();
        if trie.positions[i] == -1 {
            writeln!(writer, "-1").unwrap();
        } else {
            writeln!(writer, "{}", t.len() - trie.positions[i] as usize - trie.patterns[i].len()).unwrap();
        }
    }
}