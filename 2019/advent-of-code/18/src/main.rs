
extern crate clap;
extern crate intcode;

// Time Start: Sat, 21 Dec 2019 08:43:08 -0500
// Time Finish 1: Sat, 21 Dec 2019 11:12:30 -0500 (2 hours, 29 minutes, 22 seconds)
// Time Finish 2: Sat, 21 Dec 2019 11:24:25 -0500 (11 minutes, 55 seconds)
// Time Total: 2 hours, 41 minutes, 17 seconds

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

use clap::{Arg, App};

use intcode::util::Chart;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Keys(u32);
impl Keys {
    pub fn new(n: u32) -> Keys {
        if n > 32 { panic!("Too big") }
        let mut v = 0_u32;
        for i in 0..n { v += 1 << i; }
        return Keys(v);
    }

    pub fn is_locked(&self, n: u32) -> bool {
        match n {
            0..=31 => 1 == (1 & (self.0 >> n)),
            _      => panic!("Too big"),
        }
    }

    pub fn have_key(&self, ch: char) -> bool {
        !self.is_locked((ch.to_ascii_lowercase() as u8 - 97) as u32)
    }

    pub fn add_key(&mut self, ch: char) {
        self.unlock((ch.to_ascii_lowercase() as u8 - 97) as u32);
    }

    pub fn lock(&mut self, n: u32) {
        match n {
            0..=31 => { self.0 = self.0 | (1 << n) },
            _      => panic!("Too big"),
        }
    }
    pub fn unlock(&mut self, n: u32) {
        match n {
            0..=31 => { self.0 = self.0 & !(1 << n) },
            _      => panic!("Too big"),
        }
    }
}

pub trait Day18Chart {
    fn load(fname: &str) -> (i64, i64, Chart, u32);
    fn reachable(&self, x: i64, y: i64, keys: Keys) -> Vec<(i64, i64, char, u64)>;
    fn catch_em_all(&self, x: i64, y: i64, keys: Keys) -> u64;
    fn catch_em_all_4(&self, x: i64, y: i64, keys: Keys) -> u64;
}

impl Day18Chart for Chart {
    fn load(fname: &str) -> (i64, i64, Chart, u32) {
        let mut num = 0;
        let mut chart = Chart::new();

        let contents = fs::read_to_string(fname)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

        let (mut x, mut y) = (0, 0);
        for (j, line) in contents.trim().split('\n').enumerate() {
            for (i, ch) in line.chars().enumerate() {
                match ch {
                    '.' => (),  // No need to store open spaces
                    '@' => { x = i as i64; y = j as i64; },
                    '#' => chart.put(i as i64, j as i64, '#'),
                    ch  => {
                        if ch.to_ascii_lowercase() as u8 - 97 > num { num = ch.to_ascii_lowercase() as u8 - 97; }
                        chart.put(i as i64, j as i64, ch);
                    },
                }
            }
        }
        return (x, y, chart, num as u32 + 1);
    }

    // Given a list of keys (lower), return a vec of all reachable new keys
    // and their positions.
    fn reachable(&self, x: i64, y: i64, keys: Keys) -> Vec<(i64, i64, char, u64)> {
        let mut seen = HashSet::new();
        let mut todo = VecDeque::new();
        let mut rv = Vec::new();

        todo.push_back((x, y, 0));
        while todo.len() > 0 {
            if let Some((x, y, d)) = todo.pop_front() {
                if !seen.contains(&(x, y)) {
                    seen.insert((x, y));
                    for (dx, dy) in &[(0,1), (0,-1), (1,0), (-1,0)] {
                        let a = x + dx;
                        let b = y + dy;
                        let ch = self.item_at(a, b);
                        match ch {
                            '#' => (),
                            ' ' => { todo.push_back((a, b, d+1)); },
                            'a'..='z' => if keys.have_key(ch) { todo.push_back((a, b, d+1)); } // Already have it, pass over...
                                         else { rv.push((a, b, ch, d+1)); }, // new key, return it as a destination
                            'A'..='Z' => if keys.have_key(ch) { todo.push_back((a, b, d+1)); },
                             _  => panic!("Unexpected tile: {}", ch),
                        }
                    }
                }
            }
        }
        return rv;
    }

    fn catch_em_all(&self, x: i64, y: i64, keys: Keys) -> u64 {
        let mut todo = VecDeque::new();
        let mut best = std::u64::MAX;
        let mut seen = HashMap::new();

        todo.push_back((x, y, keys, 0_u64));
        while let Some((x, y, s, d)) = todo.pop_front() {
            if d >= best { continue; }
            if let Some(l) = seen.get(&(x, y, s)) { // Can already get here with this key set
                if d >= *l { continue; }
            }
            seen.insert((x, y, s), d);

            // Keep cheapest if done
            if s.0 == 0 && d < best {
                best = d; println!("Found length {}", d);
                continue;
            }

            for (a, b, ch, d2) in self.reachable(x, y, s) {
                let mut s2 = s;
                s2.add_key(ch);
                todo.push_back((a, b, s2, d+d2));
            }
        }
        return best;
    }

    fn catch_em_all_4(&self, x: i64, y: i64, keys: Keys) -> u64 {
        let mut todo = VecDeque::new();
        let mut best = std::u64::MAX;
        let mut seen = HashMap::new();

        todo.push_back((x+1, y+1, x+1, y-1, x-1, y+1, x-1, y-1, keys, 0_u64));
        while let Some((x0, y0, x1, y1, x2, y2, x3, y3, s, d)) = todo.pop_front() {
            if d >= best { continue; }
            if let Some(l) = seen.get(&(x0, y0, x1, y1, x2, y2, x3, y3, s)) { // Can already get here with this key set
                if d >= *l { continue; }
            }
            seen.insert((x0, y0, x1, y1, x2, y2, x3, y3, s), d);

            // Keep cheapest if done
            if s.0 == 0 && d < best {
                best = d; println!("Found length {}", d);
                continue;
            }

            for (a, b, ch, d2) in self.reachable(x0, y0, s) {
                let mut s2 = s; s2.add_key(ch);
                todo.push_back((a, b, x1, y1, x2, y2, x3, y3, s2, d+d2));
            }

            for (a, b, ch, d2) in self.reachable(x1, y1, s) {
                let mut s2 = s; s2.add_key(ch);
                todo.push_back((x0, y0, a, b, x2, y2, x3, y3, s2, d+d2));
            }

            for (a, b, ch, d2) in self.reachable(x2, y2, s) {
                let mut s2 = s; s2.add_key(ch);
                todo.push_back((x0, y0, x1, y1, a, b, x3, y3, s2, d+d2));
            }

            for (a, b, ch, d2) in self.reachable(x3, y3, s) {
                let mut s2 = s; s2.add_key(ch);
                todo.push_back((x0, y0, x1, y1, x2, y2, a, b, s2, d+d2));
            }
        }
        return best;
    }

}

fn main() {
    let matches = App::new("Advent of Code 2019, Day 18")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("18.in");

    let (x, y, chart, num) = Chart::load(&fname);
    for (x, y, ch, d) in chart.reachable(x, y, Keys::new(num)) {
        println!("Initially reachable: {} of {} at {},{} (distance {})", ch, num, x, y, d);
    }

    let dist = chart.catch_em_all(x, y, Keys::new(num));
    println!("Part 1: length {}", dist);

    let (x, y, mut chart, num) = Chart::load(&fname);
    for (dx, dy) in &[(0,1), (0,-1), (1,0), (-1,0)] {
        chart.put(x+dx, y+dy, '#');
    }

    let dist = chart.catch_em_all_4(x, y, Keys::new(num));
    println!("Part 2: length {}", dist);
}
