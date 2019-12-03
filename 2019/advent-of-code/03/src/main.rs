
extern crate clap;

// Time Start: Tue, 03 Dec 2019 06:53:21 -0500
// Time Finish 1: Tue, 03 Dec 2019 09:30:02 -0500 (2 hours, 36 minutes, 41 seconds)
// Time Finish 2: Tue, 03 Dec 2019 09:52:25 -0500 (22 minutes, 23 seconds)
// Time Total: 2 hours, 59 minutes, 4 seconds

use std::boxed::Box;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use clap::{Arg, App};

struct Move {
    dx: i16,
    dy: i16,
    step: i32,
}
impl Move {
    fn new(cmd: String) -> Move {
        let mut me = Move { dx: 0, dy: 0, step: 0 };
        match cmd.get(0..1) {
            Some("U") => { me.dy = 1 },
            Some("D") => { me.dy = -1 },
            Some("L") => { me.dx = -1 },
            Some("R") => { me.dx = 1 },
            Some(x_)  => { panic!("Unexpected move direction '{}'", x_) },
            _ => { panic!("Unexpected move command '{}'", cmd) },
        }

        if let Some(tail) = cmd.get(1..) {
            if let Ok(step) = tail.parse::<i32>() {
                me.step = step;
            } else {
                panic!("Non-numeric move in {}", cmd);
            }
        } else {
            panic!("Invalid move '{}'", cmd);
        }

        return me;
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn norm(&self) -> u32 { self.x.abs() as u32 + self.y.abs() as u32 }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[derive(Copy, Clone)]
struct Space(u8);
impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 != 0 {
            write!(f, "{}", self.0)
        } else {
            write!(f, " ")
        }
    }
}

#[derive(Copy, Clone)]
struct BBox(i32,i32,i32,i32);// xmin, xmax, ymin, ymax
impl BBox {
    pub fn new() -> BBox {
        BBox(0,0,0,0)
    }

    pub fn xmin(&self) -> i32 { self.0 }
    pub fn xmax(&self) -> i32 { self.1 }
    pub fn ymin(&self) -> i32 { self.2 }
    pub fn ymax(&self) -> i32 { self.3 }
//     pub fn width(&self) -> u32 { (self.1 as i64 - self.0 as i64) as u32 }
//     pub fn height(&self) -> u32 { (self.3 as i64 - self.2 as i64) as u32 }

    pub fn update(&mut self, pt: &Point) {
        if pt.x < self.0 { self.0 = pt.x; }
        if pt.x > self.1 { self.1 = pt.x; }
        if pt.y < self.2 { self.2 = pt.y; }
        if pt.y > self.3 { self.3 = pt.y; }
    }
}

struct Map {
    map: HashMap<Point, Space>,
    bbox: BBox,
}
impl Map {
    pub fn new() -> Map {
        Map { map: HashMap::new(), bbox: BBox::new() }
    }

    pub fn add_path<F>(&mut self, bit: u8, path: Vec<Move>, mut isect: F)
        where F: FnMut(u32, Point, Space),
    {
        let mut pos = Point::new(0, 0);
        let mut len: u32 = 0;

        for mv in path {
            for _i in 0..mv.step {
                len += 1;
                pos.x += mv.dx as i32;
                pos.y += mv.dy as i32;
                self.bbox.update(&pos);
                if !self.map.contains_key(&pos) {
                    self.map.insert(pos.clone(), Space(bit));
                }

                if let Some(space) = self.map.get_mut(&pos) {
                    space.0 |= bit;
                    if space.0 != bit {
                        isect(len, pos, *space);
                    }
                }
            }
        }
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pt = Point::new(0, 0);
        let zero = Space(0);
        for j in (self.bbox.ymin()..=self.bbox.ymax()).rev() {
            pt.y = j;
            for i in self.bbox.xmin()..=self.bbox.xmax() {
                if i == 0 && j == 0 {
                    if let Err(err) = write!(f, "*") {
                        return Err(err);
                    }
                } else {
                    pt.x = i;
                    if let Err(err) = write!(f, "{}", self.map.get(&pt).unwrap_or(&zero)) {
                        return Err(err);
                    }
                }
            }
            if let Err(err) = write!(f, "\n") {
                return Err(err);
            }
        }
        Ok(())
    }
}


fn records(fname: &String) -> Box<Iterator<Item=Vec<Move>>> {
    let file = File::open(fname).unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
    let reader = BufReader::new(file);
    let mut lineno = 0;
    let name = fname.clone();

    return Box::new(reader.lines().map(
        move |l| {
            lineno += 1;
            let line = l.unwrap_or_else(|err| panic!("Error reading {}, line {}: {}", name, lineno, err));
            line.trim().split(',').map(|s| Move::new(String::from(s))).collect()
        }
    ));
}

fn main() {
    let matches = App::new("Advent of Code 2019, Day 03")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("03.in"));

    let mut map = Map::new();
    let mut isect = Point::new(std::i32::MAX, std::i32::MAX);
    for (i, path) in records(&fname).enumerate() {
        map.add_path(2u8.pow(i as u32), path, |_len: u32, pos:Point, _space:Space| {
            if isect.norm() > pos.norm() {
                isect = pos;
            }
        });
    }
    // println!("{}", map);
    println!("Nearest intersection {} at a distance of {} from the origin", isect, isect.norm());

    // Part 2: draw over it again. This time, we get the path 1 intersections too
    struct Dist(u32, u32);
    let mut seen = HashMap::new();

    for (i, path) in records(&fname).enumerate() {
        map.add_path(2u8.pow(i as u32), path, |len: u32, pos:Point, _space:Space| {
            if !seen.contains_key(&pos) {
                seen.insert(pos.clone(), Dist(0,0));
            }

            if let Some(dist) = seen.get_mut(&pos) {
                if i == 0 && dist.0 == 0 {
                    dist.0 = len;
                } else if i == 1 && dist.1 == 0 {
                    dist.1 = len;
                }
            }
        });
    }

    let mut closest = Point::new(0,0);
    let mut best_dist = std::u32::MAX;
    for (pt, dist) in seen.iter() {
        if dist.0 + dist.1 < best_dist {
            closest = *pt;
            best_dist = dist.0 + dist.1;
        }
    }

    println!("Shortest path length intersection is at {} at combined distance of {}", closest, best_dist);

}
