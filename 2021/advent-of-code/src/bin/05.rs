// SPDX-License-Identifier: MIT

use std::cmp::Ordering;

fn point(txt: &str) -> (i64, i64) {
    let mut iter = txt.split(',').map(|v| v.parse().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn step(cmp: Ordering) -> i64 {
    match cmp {
        Ordering::Less    => -1,
        Ordering::Equal   =>  0,
        Ordering::Greater =>  1,
    }
}

#[derive(Clone, Copy)]
pub struct Data { count: u16 }
impl Data {
    fn new() -> Self { Data { count: 0 } }
    fn incr(&mut self) {
        self.count += 1;
    }
}

pub struct Map {
    map: Vec<Vec<Data>>,
}

impl Map {
    pub fn new(size: usize) -> Map {
        let mut map: Vec<Vec<Data>> = Vec::new();
        for _ in 0..size {
            map.push(vec![Data::new(); size]);
        }
        return Map { map };
    }

    pub fn count<F>(&self, f: F) -> usize where F: Fn(&Data) -> bool {
        let mut count = 0;
        for row in self.map.iter() {
            for cell in row.iter() {
                if f(&cell) { count += 1; }
            }
        }
        return count;
    }

    pub fn load(&mut self, src: &str, all: bool) {
        for expr in src.split('\n') {
            if expr.len() > 0 {
                let mut line = expr.split(" -> ");
                let (mut a1, mut a2) = point(line.next().unwrap());
                let (b1, b2) = point(line.next().unwrap());
                let dx = step(b1.cmp(&a1));
                let dy = step(b2.cmp(&a2));

                if all || dx == 0 || dy == 0 {
                    loop {
                        self.map[a1 as usize][a2 as usize].incr();
                        if a1 == b1 && a2 == b2 { break; }
                        a1 += dx;
                        a2 += dy;
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for item in row.iter() {
                if item.count > 0 { write!(f, "{}", item.count)?; }
                else              { write!(f, "{}", '.')?; }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}



fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("05.in"));
    let contents = std::fs::read_to_string(fname).unwrap();

    let mut map = Map::new(1000);
    map.load(&contents, false);
    println!("Part 1: {}", map.count(|d| d.count >= 2));

    let mut map = Map::new(1000);
    map.load(&contents, true);
    println!("Part 2: {}", map.count(|d| d.count >= 2));
}
