
extern crate clap;
extern crate intcode;

// Time Start: Sun, 22 Dec 2019 09:23:38 -0500
// Time Finish 1: Sun, 22 Dec 2019 17:56:37 -0500 (8 hours, 32 minutes, 59 seconds)
// Time Finish 2:
// Time Total:

use clap::{Arg, App};

use intcode::util::Chart;
use intcode::util::{Direction,Direction::*};

use std::collections::HashMap;


pub struct Portal {
    a: (i64, i64), a_dir: Direction,
    b: (i64, i64), b_dir: Direction,
}

pub trait Day20Chart {
    fn map_portals(&mut self);
}

impl Day20Chart for Chart {
    fn map_portals(&mut self) {
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
                    self.add_portal(xa, ya, a, b);
                    // Portal b -> a
                    let (a, b) = port.a_dir.step(xa, ya);
                    self.add_portal(xb, yb, a, b);
                },
            }
        }
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
//     println!("{}", chart);
    chart.map_portals();
//     println!("{}", chart);
    let (mut x0, mut y0) = chart.locate('A').unwrap_or_else(|| panic!("Can't find start square"));
    for dir in Direction::each() {
        let (x, y) = dir.step(x0, y0);
        if '.' == chart.item_at(x, y) {
            x0 = x; y0 = y; break;
        }
    }
    // let (x1, y1) = chart.locate('Z').unwrap_or_else(|| panic!("Can't find finish square"));

    match chart.shortest_path(x0, y0, |_,_,ch| ch == '.' || ch == '^', |_,_,ch| ch == 'Z') {
        Some((x, y, path)) => println!("Part 1: Shortest path from ({}, {}) to ({}, {}) = {}", x0, y0, x, y, path.len()-1),
        None => println!("Bummer"),
    }
}
