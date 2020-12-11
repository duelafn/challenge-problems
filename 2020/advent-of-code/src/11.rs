// SPDX-License-Identifier: MIT

use std::collections::HashMap;
// use std::convert::TryFrom;

use clap::{Arg, App};

static DIRECTIONS : [(i64,i64); 8] = [
    (-1,-1), ( 0,-1), ( 1,-1),
    (-1, 0),          ( 1, 0),
    (-1, 1), ( 0, 1), ( 1, 1),
];

#[derive(Copy, Clone)]
pub struct BBox(i64,i64,i64,i64);// xmin, xmax, ymin, ymax
impl BBox {
    pub fn new() -> BBox {
        BBox(0,0,0,0)
    }

    #[inline]
    pub fn xmin(&self) -> i64 { self.0 }
    #[inline]
    pub fn xmax(&self) -> i64 { self.1 }
    #[inline]
    pub fn ymin(&self) -> i64 { self.2 }
    #[inline]
    pub fn ymax(&self) -> i64 { self.3 }

    #[inline]
    pub fn in_bounds(&self, x: i64, y: i64) -> bool {
        x >= self.0 && x <= self.1 && y >= self.2 && y <= self.3
    }

    #[inline]
    pub fn update(&mut self, x: i64, y: i64) {
        if x < self.0 { self.0 = x; }
        if x > self.1 { self.1 = x; }
        if y < self.2 { self.2 = y; }
        if y > self.3 { self.3 = y; }
    }
}
impl std::fmt::Display for BBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BBox: {}, {}, {}, {}", self.0, self.1, self.2, self.3)
    }
}

#[derive(Clone)]
pub struct Floorplan {
    pub map: HashMap<(i64, i64), char>,
    pub bbox: BBox,
}
impl Floorplan {
    pub fn new() -> Floorplan {
        Floorplan { map: HashMap::new(), bbox: BBox::new() }
    }

    pub fn load(fname: &str) -> Floorplan {
        let mut plan = Floorplan::new();
        let contents = std::fs::read_to_string(fname)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

        for (j, line) in contents.trim_end_matches('\n').split('\n').enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch != '.' {
                    plan.insert((i as i64, j as i64), ch);
                }
                plan.bbox.update(i as i64, j as i64);
            }
        }
        return plan;
    }

    pub fn step1(&mut self) -> bool {
        let mut add = Vec::new();
        let mut rm  = Vec::new();

        for y in self.bbox.ymin()..=self.bbox.ymax() {
            for x in self.bbox.xmin()..=self.bbox.xmax() {
                match (self.at(x, y), self.count_neighbors(x, y)) {
                    ('L', 0)           => { add.push((x,y)); },
                    ('#', n) if n >= 4 => {  rm.push((x,y)); },
                    _ => { },
                }
            }
        }

        if add.is_empty() && rm.is_empty() { return false; }

        for (i, j) in add { self.insert((i, j), '#'); }
        for (i, j) in rm  { self.insert((i, j), 'L'); }
        return true;
    }

    pub fn step2(&mut self) -> bool {
        let mut add = Vec::new();
        let mut rm  = Vec::new();

        for y in self.bbox.ymin()..=self.bbox.ymax() {
            for x in self.bbox.xmin()..=self.bbox.xmax() {
                match (self.at(x, y), self.count_visible(x, y)) {
                    ('L', 0)           => { add.push((x,y)); },
                    ('#', n) if n >= 5 => {  rm.push((x,y)); },
                    _ => { },
                }
            }
        }

        if add.is_empty() && rm.is_empty() { return false; }

        for (i, j) in add { self.insert((i, j), '#'); }
        for (i, j) in rm  { self.insert((i, j), 'L'); }
        return true;
    }

    #[inline]
    pub fn count_neighbors(&self, x: i64, y: i64) -> u8 {
        let mut seated = 0;
        for (dx, dy) in DIRECTIONS.iter() {
            if '#' == self.at(x+dx, y+dy) { seated += 1; }
        }
        return seated;
    }

    #[inline]
    pub fn count_visible(&self, x: i64, y: i64) -> u8 {
        let mut seated = 0;
        for (dx, dy) in DIRECTIONS.iter() {
            let (mut x, mut y) = (x+dx, y+dy);
            while self.bbox.in_bounds(x, y) {
                match self.at(x, y) {
                    '#' => { seated += 1; break; },
                    'L' => { break; },
                    _   => { },
                }
                x += dx; y += dy;
            }
        }
        return seated;
    }

    #[inline]
    pub fn count(&self, ch: &char) -> u64 {
        let mut n = 0;
        for y in self.bbox.ymin()..=self.bbox.ymax() {
            for x in self.bbox.xmin()..=self.bbox.xmax() {
                match self.map.get(&(x,y)) {
                    Some(x) if x == ch => { n += 1; },
                    _ => { },
                }
            }
        }
        return n;
    }

    #[inline]
    pub fn at(&self, x: i64, y: i64) -> char {
        match self.map.get(&(x,y)) {
            Some(x) => *x,
            None    => '.',
        }
    }
}
impl std::ops::Deref for Floorplan {
    type Target = HashMap<(i64, i64), char>;
    fn deref(&self) -> &Self::Target { &self.map }
}
impl std::ops::DerefMut for Floorplan {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.map }
}
impl std::fmt::Display for Floorplan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.bbox.ymin()..=self.bbox.ymax() {
            for x in self.bbox.xmin()..=self.bbox.xmax() {
                write!(f, "{}", self.at(x, y))?;
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}



fn main() {
    let matches = App::new("Advent of code 2020, Day 11 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("11.in");
    let mut plan = Floorplan::load(fname);
//     println!("{}", plan);
    while plan.step1() { }
    println!("Part 1: {} occupied seats", plan.count(&'#'));
//     println!("{}", plan);

    let mut plan = Floorplan::load(fname);
//     println!("{}", plan);
    while plan.step2() { }
    println!("Part 2: {} occupied seats", plan.count(&'#'));
//     println!("{}", plan);
}
