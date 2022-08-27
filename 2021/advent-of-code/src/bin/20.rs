// SPDX-License-Identifier: MIT

use std::collections::HashSet;
// use rustc_hash::FxHashSet;
use std::io::BufRead;

#[allow(non_camel_case_types)]
type int = i16;

fn load<R: std::io::Read>(contents: R) -> Life {
    let mut lines = std::io::BufReader::new(contents).lines();
    let rule = lines.next().unwrap().unwrap().chars().map(|ch| if ch == '#' { 1 } else { 0 }).collect();
    lines.next().unwrap().unwrap();
    let mut map = HashSet::new();
//     let mut map = FxHashSet::default();
    let lines: Vec<_> = lines.collect();
    let size = (lines.len()/2 + (lines.len() % 2)) as int;
    for (y, line) in lines.into_iter().enumerate() {
        assert!(y < 2 * (size as usize));
        for (x, ch) in line.unwrap().chars().enumerate() {
            assert!(x < 2 * (size as usize));
            if ch == '#' { map.insert(((x as int) - size, (y as int) - size)); }
        }
    }
    return Life { rule, size, map, border: 0 };
}

#[derive(Default, Debug, Clone)]
struct Life {
    rule: Vec<u8>,
    border: usize,
    size: int,
    map: HashSet<(int,int)>,
//     map: FxHashSet<(int,int)>,
}

impl Life {
    fn step(&mut self) {
        let size = self.size+1;
        let mut map = HashSet::with_capacity(self.map.len() + 4*(size as usize));
//         let mut map = FxHashSet::default();
        for x in -size..size+1 {
            for y in -size..size+1 {
                if self.next(x, y) == 1 { map.insert((x, y)); }
            }
        }
        let borderval = if self.border == 0 { 0 } else { 511 };
        self.border = self.rule[borderval] as usize;
        self.size = size;
        self.map = map;
    }

    fn state(&self, x: int, y: int) -> usize {
        if x < -self.size || x > self.size || y < -self.size || y > self.size { self.border }
        else if self.map.contains(&(x, y)) { 1 }
        else { 0 }
    }

    fn next(&self, i: int, j: int) -> u8 {
        let mut v = 0;
        for dy in &[-1, 0, 1] {
            for dx in &[-1, 0, 1] {
                v <<= 1;
                v += self.state(i+dx, j+dy);
            }
        }
        self.rule[v]
    }
}

impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in -self.size..self.size+1 {
            for x in -self.size..self.size+1 {
                if self.map.contains(&(x, y)) { write!(f, "#")?; } else { write!(f, ".")?; }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("20.in"));
    let mut life = load(std::fs::File::open(fname).unwrap());

    life.step();
    life.step();
    println!("Part 1: {}", life.map.len());

    for _ in 2..50 { life.step(); }
    println!("Part 2: {}", life.map.len());
}
