// SPDX-License-Identifier: MIT

use std::convert::TryFrom;

use clap::{Arg, App};


#[inline]
fn div_ceil(a: u64, b: u64) -> u64 {
    let (d, r) = (a / b, a % b);
    if r != 0 { d + 1 } else { d }
}

/// Solve: _ y = c + a x  for a, we don't care about _; start at a1 and step by k
#[inline]
fn d_solve(y: u64, c: u64, x: u64, a1: u64, k: u64) -> u64 {
    let mut a = a1;
    loop {
        if 0 == (c + a * x) % y { return a; }
        a += k;
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    pub arrival: u64,
    pub ids: Vec<u64>,
}
impl Schedule {

}
impl std::convert::TryFrom<&str> for Schedule {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut lines = src.lines();
        let arrival = lines.next().ok_or(String::from("Missing line 1"))?
                      .parse().map_err(|err| format!("Number parse error: {}", err))?;
        let ids = lines.next().ok_or(String::from("Missing line 2"))?
                .split(",")
                .map(|x| x.parse().unwrap_or(0))
                .collect();
        Ok(Self { arrival, ids })
    }
}


fn load(fname: &str) -> Schedule {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
    return Schedule::try_from(contents.as_ref()).unwrap_or_else(|err| panic!("Parse error in {}: {}", fname, err));
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 13 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("13.in");

    let schedule = load(fname);
    let (mut bus, mut time) = (0, 0);
    for id in schedule.ids.iter() {
        if *id > 0 {
            let t = id * div_ceil(schedule.arrival, *id);
            if bus == 0 || t < time {
                bus = *id;
                time = t;
            }
        }
    }
    let wait = time - schedule.arrival;
    println!("Part 1: Earliest depart {} on bus {}, waiting {} minutes, solution: {}", time, bus, wait, bus * wait);

    // A combination of brute-force and a little care, but no fancy math necessary.
    // Huh... and it worked first try!
    let mut iter = schedule.ids.iter();
    let x = *iter.next().unwrap();
    let (mut a, mut k, mut offset) = (1, 1, 0);
    for id in iter {
        offset += 1;
        if *id > 0 {
            // d_solve(y, c, x, a1, k) -> a
            // _ y = c + a x ; start at a1 and step by k
            a = d_solve(*id, offset, x, a, k);
            k *= *id;
        }
    }
    println!("Part 2: Time t = {}", x * a);
}
