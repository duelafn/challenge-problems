// SPDX-License-Identifier: MIT

use std::convert::TryFrom;

use clap::{Arg, App};

use Action::*;


enum Action {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}
impl std::convert::TryFrom<&str> for Action {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let (dir, n) = src.split_at(1);
        let n = n.parse().or_else(|err| Err(format!("Error parsing number '{}': {}", n, err)))?;
        match dir.chars().next().unwrap() {
            'N' => Ok(North(n)),
            'S' => Ok(South(n)),
            'E' => Ok(East(n)),
            'W' => Ok(West(n)),
            'L' => Ok(Left(n)),
            'R' => Ok(Right(n)),
            'F' => Ok(Forward(n)),
            ccc => Err(format!("Unknown command '{}'", ccc)),
        }
    }
}

struct Ship {
    x: i64,
    y: i64,
    heading: i64,
}
impl Ship {
    pub fn new() -> Ship {
        Ship { x: 0, y: 0, heading: 0 }
    }

    pub fn distance(&self) -> i64 { self.x.abs() + self.y.abs() }

    pub fn act(&mut self, a: &Action) {
        match a {
            North(n)   => { self.y += n },
            South(n)   => { self.y -= n },
            East(n)    => { self.x += n },
            West(n)    => { self.x -= n },
            Left(n)    => { self.heading = (self.heading + n).rem_euclid(360) },
            Right(n)   => { self.heading = (self.heading - n).rem_euclid(360) },
            Forward(n) => {
                match self.heading {
                    0   => { self.x += n },
                    90  => { self.y += n },
                    180 => { self.x -= n },
                    270 => { self.y -= n },
                    x   => { panic!("Unexpected heading {}", x); }
                }
            },
        }
    }

    pub fn apply(&mut self, a: &Waypoint, n: i64) {
        self.x += n * a.x;
        self.y += n * a.y;
    }
}
impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}), heading {}", self.x, self.y, self.heading)
    }
}


struct Waypoint {
    x: i64,
    y: i64,
}
impl Waypoint {
    pub fn new(x: i64, y: i64) -> Waypoint {
        Waypoint { x, y }
    }

    pub fn act(&mut self, a: &Action) {
        match a {
            North(n)   => { self.y += n },
            South(n)   => { self.y -= n },
            East(n)    => { self.x += n },
            West(n)    => { self.x -= n },
            Left(n)    => { for _ in 0..n/90 { let tmp = self.x; self.x = -self.y; self.y = tmp } },
            Right(n)   => { for _ in 0..n/90 { let tmp = self.x; self.x = self.y; self.y = -tmp } },
            Forward(_) => { panic!("I cannae move the wind captain!") },
        }
    }
}
impl std::fmt::Display for Waypoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}



fn records(fname: &str) -> Vec<Action> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.lines().enumerate().map(
        |(lineno, chunk)| {
            Action::try_from(chunk).unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 12 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("12.in");

    let course = records(fname);
    let mut mcboatface = Ship::new();

    for a in course.iter() {
        mcboatface.act(a);
    }
    println!("Ship at {}, distance {}", mcboatface, mcboatface.distance());


    let mut mcboatface = Ship::new();
    let mut waypoint   = Waypoint::new(10, 1);

    for a in course.iter() {
        match a {
            Forward(n) => mcboatface.apply(&waypoint, *n),
            _          => waypoint.act(a),
        }
    }
    println!("Ship at {}, distance {}; waypoint: {}", mcboatface, mcboatface.distance(), waypoint);
}
