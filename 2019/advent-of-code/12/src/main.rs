
extern crate clap;
extern crate itertools;
extern crate num_integer;
extern crate regex;

// Time Start: Thu, 12 Dec 2019 12:27:13 -0500
// Time Finish 1: Thu, 12 Dec 2019 15:55:35 -0500 (3 hours, 28 minutes, 22 seconds)
// Time Finish 2: Thu, 12 Dec 2019 16:56:18 -0500 (1 hour, 0 minutes, 43 seconds)
// Time Total: 4 hours, 29 minutes, 5 seconds

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::str::FromStr;

use clap::{Arg, App};
use itertools::Itertools;
use num_integer::Integer;
use regex::Regex;


struct Moon {
    pub pos: [i32; 3],
    pub vel: [i32; 3],
}
impl Moon {
    pub fn parse(line: &String) -> Moon {
        // <x=-8, y=-18, z=6>  :: Blah, too lazy to fix the unwraps.
        let re = Regex::new(r"^<x=([-\d]+), y=([-\d]+), z=([-\d]+)>").unwrap_or_else(|err| panic!("regex error: {}", err));
        if let Some(cap) = re.captures(line) {
            let pos: Vec<i32> = cap.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i32>().unwrap()).collect();
            return Moon {
                pos: [ pos[0], pos[1], pos[2] ],
                vel: [ 0, 0, 0 ],
            }
        }
        panic!("Invalid input: {}", line);
    }

    pub fn gravitate(&mut self, other: &Moon) {
        for i in 0..3 {
            self.vel[i] += match self.pos[i].cmp(&other.pos[i]) {
                Ordering::Less => 1,
                Ordering::Greater => -1,
                Ordering::Equal => 0,
            };
        }
    }

    pub fn step(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }

    pub fn energy(&self) -> i32 {
        let potential = self.pos.iter().map(|v| v.abs()).fold(0_i32, |a, b| a.checked_add(b).unwrap_or_else(|| panic!("Numeric overflow in potential energy")));
        let kinetic   = self.vel.iter().map(|v| v.abs()).fold(0_i32, |a, b| a.checked_add(b).unwrap_or_else(|| panic!("Numeric overflow in kinetic energy")));
        return potential * kinetic;
    }
}
impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>", self.pos[0], self.pos[1], self.pos[2], self.vel[0], self.vel[1], self.vel[2])
    }
}
impl FromStr for Moon {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Moon::parse(&String::from(s)))
    }
}

fn step(moons: &mut Vec<Moon>) {
    // This is stupid!
    for pair in (0..moons.len()).combinations(2) { // .combinations() Clones iterator elements
        let (i, j) = if pair[0] < pair[1] { (pair[0], pair[1]) } else { (pair[1], pair[0]) }; // Probably ordered but just in case
        let (a, b) = moons.split_at_mut(j); // WTF Rust!
        a[i].gravitate(&b[0]);
        b[0].gravitate(&a[i]);
    }
    for m in moons.iter_mut() {
        m.step();
    }
}

fn fingerprint(i: usize, moons: &Vec<Moon>) -> Vec<i32> {
    let mut v = Vec::new();
    for m in moons {
        v.push(m.pos[i]);
        v.push(m.vel[i]);
    }
    return v;
}

fn periods(mut moons: &mut Vec<Moon>) -> Vec<u64> {
    let mut done   = vec![false, false, false];
    let mut period = vec![0, 0, 0];
    let mut seen   = vec![HashSet::new(), HashSet::new(), HashSet::new()];

    loop {
        let mut todo = false;
        for i in 0..3 {
            if !done[i] {
                let fprint = fingerprint(i, moons);
                if seen[i].insert(fprint) {
                    period[i] += 1;
                    todo = true;
                } else {
                    done[i] = true;
                    println!("Axis {} has period {}", i, period[i]);
                }
            }
        }
        if !todo { break; }
        step(&mut moons);
    }
    return period;
}

fn load_moons(fname: &String) -> Vec<Moon> {
    let contents = fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.trim().split('\n').map(|l| { l.parse::<Moon>().unwrap_or_else(|err| panic!("Bummer: {}", err)) }).collect();
}

fn main() {
    let matches = App::new("Advent of Code 2019, Day 12")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("12.in"));

    // Step 1
    let num_iter = 1000;
    let mut moons = load_moons(&fname);
    for _ in 1..=num_iter { step(&mut moons); }

    let mut energy = 0;
    for m in &moons {
        println!("{}  : energy {}", m, m.energy());
        energy += m.energy();
    }
    println!("Total energy after {} steps: {}\n", num_iter, energy);

    // Step 2
    let mut moons = load_moons(&fname);
    let periods = periods(&mut moons);
    println!("Period of whole system: {}", periods.iter().fold(1, |a, b| a.lcm(b)));
}
