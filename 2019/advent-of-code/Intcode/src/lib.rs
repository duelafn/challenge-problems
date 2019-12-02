
use std::convert::TryFrom;
use std::fmt;
use std::fs;

pub struct Intcode {
    program: Vec<i32>,
    pos: usize,
}

impl Intcode {
    pub fn new() -> Intcode {
        return Intcode { program: Vec::new(), pos: 0 };
    }

    pub fn load(fname: &String) -> Intcode {
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

    pub fn is_halted(&self) -> bool { 99 == *self.program.get(self.pos).unwrap_or(&99) }

    pub fn run(&mut self) {
        while self.step() { }
    }

    pub fn step(&mut self) -> bool {
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

    pub fn get(&self, i: usize) -> i32 { self.program[i] }
    pub fn set(&mut self, i: usize, val: i32) { self.program[i] = val }
    pub fn push(&mut self, val: i32) { self.program.push(val) }

    // fn peek(&self, i: usize) -> i32 { self.get(self.pos + i) }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2() {
        let mut ic = Intcode { program: vec![ 1,0,0,0,99 ], pos: 0 };
        ic.run();
        assert_eq!(ic.program, vec![ 2,0,0,0,99 ]);// Example 1

        let mut ic = Intcode { program: vec![ 2,3,0,3,99 ], pos: 0 };
        ic.run();
        assert_eq!(ic.program, vec![ 2,3,0,6,99 ]);// Example 2

        let mut ic = Intcode { program: vec![ 2,4,4,5,99,0 ], pos: 0 };
        ic.run();
        assert_eq!(ic.program, vec![ 2,4,4,5,99,9801 ]);// Example 3

        let mut ic = Intcode { program: vec![ 1,1,1,4,99,5,6,0,99 ], pos: 0 };
        ic.run();
        assert_eq!(ic.program, vec![ 30,1,1,4,2,5,6,0,99 ]);// Example 4

        let mut ic = Intcode::load(&String::from("02.in"));
        ic.set(1, 12);
        ic.set(2, 2);
        ic.run();
        assert_eq!(ic.get(0), 3562672);// Part 1
    }

    #[test]
    #[should_panic(expected = "Unknown command at position 0: 77")]
    fn bad_command() {
        let mut ic = Intcode { program: vec![ 77,0,0,0,99 ], pos: 0 };
        ic.run();
    }
}
