
extern crate clap;

// Time Start: Mon, 02 Dec 2019 12:59:41 -0500
// Time Finish 1: Mon, 02 Dec 2019 14:46:54 -0500 (1 hour, 47 minutes, 13 seconds)
// Time Finish 2: Mon, 02 Dec 2019 14:53:56 -0500 (7 minutes, 2 seconds)
// Time Total: 1 hour, 54 minutes, 15 seconds

use std::convert::TryFrom;
use std::fmt;
use std::fs;

use clap::{Arg, App};


struct Intcode {
    program: Vec<i32>,
    pos: usize,
}

impl Intcode {
    fn new() -> Intcode {
        return Intcode { program: Vec::new(), pos: 0 };
    }

    fn load(fname: &String) -> Intcode {
        let mut ic = Intcode::new();
        // One line, CSV integers
        let csv = fs::read_to_string(fname).unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

        for instr in csv.trim().split(',') {
            ic.program.push(
                instr.parse().unwrap_or_else(|err| panic!("Not an integer '{}' in {}: {}", instr, fname, err))
            );
        }
        return ic;
    }

    fn is_halted(&self) -> bool { 99 == *self.program.get(self.pos).unwrap_or(&99) }

    fn run(&mut self) {
        while self.step() { }
    }

    fn step(&mut self) -> bool {
        let step = match self.peeka(0) {
            1 => {
                let (a, b, c) = (self.peeka(1),  self.peeka(2),  self.peeka(3)); // immutable borrow
                self.add(a, b, c); // mutable borrow
                4
            },
            2 => {
                let (a, b, c) = (self.peeka(1),  self.peeka(2),  self.peeka(3)); // immutable borrow
                self.mul(a, b, c); // mutable borrow
                4
            },
            99 => 0,
            x_ => panic!("Unknown command at position {}: {}", self.pos, x_),
        };

        self.pos += step;
        return step > 0;
    }

    fn add(&mut self, a: usize, b: usize, c: usize) {
        self.program[c] = self.program[a] + self.program[b];
    }

    fn mul(&mut self, a: usize, b: usize, c: usize) {
        self.program[c] = self.program[a] * self.program[b];
    }

    fn get(&self, i: usize) -> i32 { self.program[i] }
    fn peek(&self, i: usize) -> i32 { self.get(self.pos + i) }
    fn geta(&self, i: usize) -> usize {
        usize::try_from(
            self.program[i]
        ).unwrap_or_else(|err| panic!("Expected address at position {}, found '{}' instead: {}", i, self.program[i], err))
    }
    fn peeka(&self, i: usize) -> usize { self.geta(self.pos + i) }
}

impl fmt::Display for Intcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.program)
    }
}



fn main() {
    let matches = App::new("Advent of Code 2019, Day 02")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("02.in"));

    {
        let mut ic = Intcode::load(&fname);
        ic.program[1] = 12;
        ic.program[2] = 2;
        ic.run();
        println!("Part 1: {}", ic.get(0));
    }

    for noun in 0..100 {
        for verb in 0..100 {
            let mut ic = Intcode::load(&fname);
            ic.program[1] = noun;
            ic.program[2] = verb;
            ic.run();
            if ic.get(0) == 19690720 {
                println!("Part 2: got 19690720 with noun={}, verb={}, key={}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}
