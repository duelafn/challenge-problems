
extern crate clap;
extern crate intcode;

// Time Start: Sun, 15 Dec 2019 00:56:30 -0500
// Time Finish 1: Sun, 15 Dec 2019 03:10:37 -0500 (2 hours, 14 minutes, 7 seconds)
// Time Finish 2:
// Time Total:

use clap::{Arg, App};

use intcode::Intcode;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;


#[derive(Copy, Clone)]
struct BBox(i64,i64,i64,i64);// xmin, xmax, ymin, ymax
impl BBox {
    pub fn new() -> BBox {
        BBox(0,0,0,0)
    }

    pub fn xmin(&self) -> i64 { self.0 }
    pub fn xmax(&self) -> i64 { self.1 }
    pub fn ymin(&self) -> i64 { self.2 }
    pub fn ymax(&self) -> i64 { self.3 }

    pub fn update(&mut self, x: i64, y: i64) {
        if x < self.0 { self.0 = x; }
        if x > self.1 { self.1 = x; }
        if y < self.2 { self.2 = y; }
        if y > self.3 { self.3 = y; }
    }
}
impl fmt::Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BBox: {}, {}, {}, {}", self.0, self.1, self.2, self.3)
    }
}


struct Droid {
    pub pos: (i64, i64),
    pub map: HashMap<(i64, i64), char>,
    pub bbox: BBox,
    ctrl: Intcode,
}
impl Droid {
    pub fn new(ctrl: Intcode) -> Droid {
        let mut map = HashMap::new();
        map.insert((0,0), '.');
        Droid {
            pos: (0, 0),
            map: map,
            bbox: BBox::new(),
            ctrl: ctrl,
        }
    }

    pub fn item_at(&self, x: i64, y: i64) -> char {
        match self.map.get(&(x,y)) {
            Some(x) => *x,
            None    => ' ',
        }
    }

    pub fn put(&mut self, x: i64, y: i64, obj: char) {
        self.bbox.update(x, y);
        self.map.insert((x, y), obj);
    }

    pub fn follow_path(&mut self, path: &Vec<u8>) -> Option<char> {
        for (i, dir) in path.iter().enumerate() {
            let (x, y) = match dir {
                1 => (self.pos.0, self.pos.1 + 1), // North
                2 => (self.pos.0, self.pos.1 - 1), // South
                3 => (self.pos.0 - 1, self.pos.1), // West
                4 => (self.pos.0 + 1, self.pos.1), // East
                _ => unreachable!("Invalid dir {}", dir),
            };
            self.ctrl.pipe(*dir as i64);
            while self.ctrl.output_len() < 1 && self.ctrl.step() { }
            if self.ctrl.is_halted() { return None; }
            match self.ctrl.shift_output() {
                Some(0) => {
                    self.put(x, y, '#');
                    if i == path.len()-1 { break } else { return None };
                },
                Some(1) => { self.put(x, y, '.'); },
                Some(2) => { self.put(x, y, 'O'); },
                Some(x) => unreachable!("Unexpected output: {}", x),
                _ => unreachable!("Unexpected output: None"),
            };
            self.pos.0 = x;
            self.pos.1 = y;
        }
        return Some(self.item_at(self.pos.0, self.pos.1));
    }

    pub fn search_for_oxygen_system(&mut self) -> Option<Vec<u8>> {
        // Want to stay close to 0, 0 so we find the shortest path.
        // Therefore, we always start our search for a new tile at out origin
        let (x0, y0) = self.pos;
        loop {
            let (x, y, path0) = self.find_nearest(x0, y0, |_x, _y, ch| ch == ' ')?; // position closest to start
            let path = self.find_path(self.pos.0, self.pos.1, x, y)?; // shortest path from here to there
            let obj = self.follow_path(&path)?;
            if obj == 'O' { // Found it!
                return Some(path0);
            }
        }
    }

    // Find a path through the known universe between the two given points.
    pub fn find_path(&self, x0: i64, y0: i64, x1: i64, y1: i64) -> Option<Vec<u8>> {
        let (_, _, path) = self.find_nearest(x0, y0, |x, y, _ch| x == x1 && y == y1)?;
        return Some(path);
    }

    // Search known universe for the coordinate of `obj` nearest to the given start coordinate.
    pub fn find_nearest(&self, x: i64, y: i64, test: impl Fn(i64, i64, char) -> bool) -> Option<(i64, i64, Vec<u8>)> {
        let mut todo: VecDeque<(i64, i64, Vec<u8>)> = VecDeque::new();
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut step: Vec<(i64, i64, u8)> = Vec::new();
        step.push((0,  1, 1)); // North
        step.push((0, -1, 2)); // South
        step.push((-1, 0, 3)); // West
        step.push(( 1, 0, 4)); // East

        todo.push_back((x, y, Vec::new()));
        loop {
            let (x, y, path) = todo.pop_front()?; // or return None
            if test(x, y, self.item_at(x, y)) {
                return Some((x, y, path));
            }
            if '.' == self.item_at(x, y) {
                for i in 0..4 {
                    let (a, b, dir) = step[i];
                    if !seen.contains(&(x+a, y+b)) {
                        let mut newpath = path.clone();
                        newpath.push(dir);
                        todo.push_back((x+a, y+b, newpath));
                    }
                }
            }
            seen.insert((x, y));
        };
    }
}
impl fmt::Display for Droid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (self.bbox.ymin()..=self.bbox.ymax()).rev() {
            for x in self.bbox.xmin()..=self.bbox.xmax() {
                if let Err(err) = write!(f, "{}", self.item_at(x, y)) {
                    return Err(err);
                }
            }
            if let Err(err) = write!(f, "{}", "\n") {
                return Err(err);
            }
        }
        Ok(())
    }
}



fn main() {
    let matches = App::new("Advent of Code 2019, Day 15")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("15.in"));

    let mut droid = Droid::new(Intcode::load(&fname));
    if let Some(path) = droid.search_for_oxygen_system() {
        println!("{}", droid);
        println!("Found path length {}", path.len());
    } else {
        println!("Bummer");
    }
}
