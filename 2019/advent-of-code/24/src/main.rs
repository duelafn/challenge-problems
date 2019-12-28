
extern crate clap;
extern crate intcode;

// Time Start: Thu, 26 Dec 2019 16:04:06 -0500
// Time Finish 1: Thu, 26 Dec 2019 16:55:31 -0500 (51 minutes, 25 seconds)
// Time Finish 2:
// Time Total:

use std::collections::HashSet;

use clap::{Arg, App};

use intcode::util::{Chart,Direction,Direction::*};

trait Life {
    fn step(&mut self);
    fn fingerprint(&self) -> u32;
}
impl Life for Chart {
    fn fingerprint(&self) -> u32 {
        let mut finger = 0;
        for y in (self.bbox.ymin()..=self.bbox.ymax()).rev() {
            for x in (self.bbox.xmin()..=self.bbox.xmax()).rev() {
                finger <<= 1;
                if '#' == self.item_at(x, y) {
                    finger |= 1;
                }
            }
        }
        return finger;
    }

    fn step(&mut self) {
        let orig = self.clone();
        for x in self.bbox.xmin()..=self.bbox.xmax() {
            for y in self.bbox.ymin()..=self.bbox.ymax() {
                let mut adjacent = 0;
                for dir in &[North, South, East, West] {
                    let (a, b) = dir.step(x, y);
                    if '#' == orig.item_at(a, b) { adjacent += 1; }
                }
                if adjacent == 1 {
                    self.put(x, y, '#');
                } else if adjacent == 2 {
                    if '#' == orig.item_at(x, y) {
                        self.put(x, y, '.');
                    } else {
                        self.put(x, y, '#');
                    }
                } else {
                    self.put(x, y, '.');
                }
            }
        }
    }
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 24")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("24.in");

    let mut eris = Chart::load(&fname);
    eris.print_flipped = true;
    let mut seen = HashSet::new();
    while seen.insert(eris.fingerprint()) {
        if seen.len() < 4 {
            println!("{}:\n{}", seen.len()-1, eris);
        }
        eris.step();
    }

    println!("{}\nRating: {}", eris, eris.fingerprint());
}
