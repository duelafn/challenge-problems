
extern crate clap;
extern crate intcode;

// Time Start: Sun, 22 Dec 2019 09:23:38 -0500
// Time Finish 1: Sun, 22 Dec 2019 17:56:37 -0500 (8 hours, 32 minutes, 59 seconds)
// Time Finish 2:
// Time Total:

use clap::{Arg, App};

use intcode::util::{BBox,Direction,Direction::*};

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::{thread, time};


pub struct Portal {
    a: (i64, i64), a_dir: Direction,
    b: (i64, i64), b_dir: Direction,
}



pub struct Chart {
    pub map: HashMap<(i64, i64), char>,
    pub bbox: BBox,
    pub portals: HashMap<(i64, i64), (i64, i64)>,
}
impl Chart {
    pub fn new() -> Chart {
        Chart {
            map: HashMap::new(),
            bbox: BBox::new(),
            portals: HashMap::new(),
        }
    }

    pub fn load(fname: &str) -> Chart {
        let mut chart = Chart::new();
        let contents = fs::read_to_string(fname)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

        for (j, line) in contents.trim_end_matches('\n').split('\n').enumerate() {
            for (i, ch) in line.chars().enumerate() {
                chart.put(i as i64, j as i64, ch);
            }
        }
        return chart;
    }

    pub fn item_at(&self, x: i64, y: i64) -> char {
        match self.map.get(&(x,y)) {
            Some(x) => *x,
            None    => ' ',
        }
    }

    pub fn locate(&self, ch: char) -> Option<(i64, i64)> {
        for (key, val) in self.map.iter() {
            if *val == ch {
                return Some(*key);
            }
        }
        return None;
    }

    pub fn follow_portal(&self, x0: i64, y0: i64) -> (i64, i64) {
        match self.portals.get(&(x0,y0)) {
            Some(&(x, y)) => (x,y),
            None    => panic!("Tried to follow non-existant portal at ({}, {})", x0, y0),
        }
    }
    pub fn add_portal(&mut self, nest: bool, x0: i64, y0: i64, x1: i64, y1: i64) {
        let sym = if nest {
            if self.near_border(x0, y0, 4) { 'v' } else { '^' }
        } else { '=' };
        self.put(x0, y0, sym);
        self.portals.insert((x0, y0), (x1, y1));
    }

    fn near_border(&self, x: i64, y: i64, dist: i64) -> bool {
        ((x-self.bbox.xmin()).abs() <= dist || (x-self.bbox.xmax()).abs() <= dist)
            ||
        ((y-self.bbox.ymin()).abs() <= dist || (y-self.bbox.ymax()).abs() <= dist)
    }

    pub fn get(&self, x: i64, y: i64) -> Option<&char> { self.map.get(&(x,y)) }
    pub fn put(&mut self, x: i64, y: i64, obj: char) {
        self.bbox.update(x, y);
        self.map.insert((x, y), obj);
    }

    pub fn shortest_path(&self, x: i64, y: i64, valid: impl Fn(i64, i64, i64, char) -> bool, wanted: impl Fn(i64, i64, i64, char) -> bool) -> Option<(i64, i64, Vec<Direction>)> {
        let mut todo = VecDeque::new();
        let mut seen = HashSet::new();

        todo.push_back((x, y, 0, Vec::new()));
        loop {
            let (x, y, level, path) = todo.pop_front()?; // or return None
            let spot = self.item_at(x, y);
            if wanted(x, y, level, spot) {
                return Some((x, y, path));
            }
            if valid(x, y, level, spot) {
                for dir in Direction::each() {
                    let (mut a, mut b) = dir.step(x, y);
                    let mut lvl = match self.item_at(a, b) {
                        '^' =>  1,
                        'v' => -1,
                        '=' => 99,
                         _  =>  0,
                    };
                    if lvl != 0 {
                        let (aa, bb) = self.follow_portal(a, b);
                        a = aa; b = bb;
                        if lvl == 99 { lvl = 0; }
                    }
                    if !seen.contains(&(a, b, level+lvl)) {
                        let mut newpath = path.clone();
                        newpath.push(*dir);
                        todo.push_back((a, b, level+lvl, newpath));
                    }
                }
            }
            seen.insert((x, y, level));
        };
    }

    fn map_portals(&mut self, nest: bool) {
        fn update_portal(pmap: &mut HashMap<String, Portal>, name: &String, x: i64, y: i64, dir: Direction) {
            match pmap.get_mut(name) {
                Some(port) => {
                    port.b = (x, y);
                    port.b_dir = dir;
                },
                None => {
                    pmap.insert(name.clone(), Portal {
                        a: (x, y), a_dir: dir,
                        b: (0, 0), b_dir: North,
                    });
                }
            }
        }

        let mut stuff = HashMap::new();
        for ((x, y), ch) in self.map.iter() {
            match *ch {
                'A'..='Z' => {
                    let mut name = String::new();
                    name.push(*ch);
                    // These run left-to-right or top-to-bottom, only pay
                    // attention when we find when we find the left or top
                    // letter ().
                    let ch2 = self.item_at(*x+1, *y);// LR
                    if 65 <= ch2 as u8 && ch2 as u8 <= 90 {
                        name.push(ch2);
                        match self.item_at(*x+2, *y) {
                            '.' => update_portal(&mut stuff, &name, *x+1, *y, East), // XX.
                             _  => update_portal(&mut stuff, &name,   *x, *y, West), // .XX
                        }
                    }
                    let ch2 = self.item_at(*x, *y+1);// TB
                    if 65 <= ch2 as u8 && ch2 as u8 <= 90 {
                        name.push(ch2);
                        match self.item_at(*x, *y+2) {
                            '.' => update_portal(&mut stuff, &name, *x, *y+1, North), // Y/Y/.
                             _  => update_portal(&mut stuff, &name, *x,   *y, South), // ./Y/Y
                        }
                    }
                },
                _ => (),
            }
        }

        for (name, port) in stuff.iter() {
            match name.as_str() {
                "AA" | "ZZ" => { // Remove the duplicate, leave just one symbol
                    let (x, y) = port.a;
                    let (a, b) = port.a_dir.rev().step(x, y);
                    self.put(a, b, ' ');
                },
                _ => {
                    let (xa, ya) = port.a;
                    let (xb, yb) = port.b;
                    // erase second char
                    let (a, b) = port.a_dir.rev().step(xa, ya); self.put(a, b, ' ');
                    let (a, b) = port.b_dir.rev().step(xb, yb); self.put(a, b, ' ');
                    // Portal a -> b
                    let (a, b) = port.b_dir.step(xb, yb);
                    self.add_portal(nest, xa, ya, a, b);
                    // Portal b -> a
                    let (a, b) = port.a_dir.step(xa, ya);
                    self.add_portal(nest, xb, yb, a, b);
                },
            }
        }
    }
}
impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (self.bbox.ymin()..=self.bbox.ymax()).rev() {
            for x in self.bbox.xmin()..=self.bbox.xmax() {
                write!(f, "{}", self.item_at(x, y))?;
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}

fn walk(chart: &mut Chart, mut x: i64, mut y: i64, path: &Vec<Direction>) {
    let mut level = 0;
    let mut was;
    let mut portal;
    for dir in path {
        let (a, b) = dir.step(x, y);
        x = a; y = b;
        was = chart.item_at(x, y);

        portal = false;
        if was == '^' { level += 1; portal = true; }
        if was == 'v' { level -= 1; portal = true; }
        if portal {
            let (a, b) = chart.follow_portal(x, y);
            x = a; y = b;
        }

        was = chart.item_at(x, y);
        chart.put(x, y, '@');
        println!("\x1B[H\x1B[2J{}\n\n Level: {}", chart, level);
        thread::sleep(time::Duration::from_millis(30));
        chart.put(x, y, was);
    }
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 20")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("20.in");

    let mut chart = Chart::load(fname);
    chart.map_portals(false);
    let (mut x0, mut y0) = chart.locate('A').unwrap_or_else(|| panic!("Can't find start square"));
    for dir in Direction::each() {
        let (x, y) = dir.step(x0, y0);
        if '.' == chart.item_at(x, y) {
            x0 = x; y0 = y; break;
        }
    }

    match chart.shortest_path(x0, y0, |_,_,_,ch| ch == '.' || ch == '=', |_,_,_,ch| ch == 'Z') {
        Some((x, y, path)) => println!("Part 1: Shortest path from ({}, {}) to ({}, {}) = {}", x0, y0, x, y, path.len()-1),
        None => println!("Bummer"),
    }

    let mut chart = Chart::load(fname);
    chart.map_portals(true);
    match chart.shortest_path(x0, y0, |_,_,l,ch| l >= 0 && (ch == '.' || ch == '^' || ch == 'v'), |_,_,l,ch| ch == 'Z' && l == 0) {
        Some((x, y, path)) => {
            // walk(&mut chart, x0, y0, &path);
            println!("Part 2: Shortest path from ({}, {}) to ({}, {}) = {}", x0, y0, x, y, path.len()-1);
        },
        None => println!("Bummer"),
    }
}
