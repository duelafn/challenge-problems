
use std::fmt;
use std::fs;

pub enum Val {
    Address(usize),
    Quantity(i64),
}
impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Address(n) => write!(f, "Address({})", n),
            Quantity(n) => write!(f, "Quantity({})", n),
        }
    }
}
impl fmt::Debug for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Address(n) => write!(f, "Address({})", n),
            Quantity(n) => write!(f, "Quantity({})", n),
        }
    }
}

use Val::*;

#[derive(Clone)]
pub struct Intcode {
    program: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    pos: usize,
}

impl Intcode {
    pub fn new() -> Intcode {
        return Intcode { program: Vec::new(), pos: 0, input: Vec::new(), output: Vec::new() };
    }

    pub fn init(prog: Vec<i64>) -> Intcode {
        return Intcode { program: prog, pos: 0, input: Vec::new(), output: Vec::new() };
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

    pub fn is_halted(&self) -> bool { 99 == self.program[self.pos] }

    pub fn run(&mut self) {
        while self.step() { }
    }

    pub fn step(&mut self) -> bool {
        let code = self.program[self.pos];
        let (mode, code) = (code / 100, code % 100);
        match code {
            1 => { let v = self.consume(mode, 3); self.add(v) },
            2 => { let v = self.consume(mode, 3); self.mul(v) },
            3 => { let v = self.consume(mode, 1); self.read(v) },
            4 => { let v = self.consume(mode, 1); self.write(v) },
            5 => { let v = self.consume(mode, 2); self.jump_if_true(v) },
            6 => { let v = self.consume(mode, 2); self.jump_if_false(v) },
            7 => { let v = self.consume(mode, 3); self.lt(v) },
            8 => { let v = self.consume(mode, 3); self.eq(v) },

            99 => return false,
            x_ => panic!("Unknown command at position {}: {} ({})", self.pos, x_, self),
        };
        return true;
    }

    pub fn pipe(&mut self, val: i64) { self.input.push(val); }
    pub fn cat(&mut self) -> Vec<i64> { self.output.clone() }
    pub fn has_output(&mut self) -> bool { self.output.len() > 0 }
    pub fn shift_output(&mut self) -> Option<i64> {
        if self.output.len() > 0 {
            return Some(self.output.remove(0))
        } else {
            return None
        }
    }

    fn add(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b), Some(Address(c))) = (param.get(0), param.get(1), param.get(2)) {
            self.program[*c] = self.get(a) + self.get(b);
        } else {
            panic!("Invalid add(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }
    fn mul(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b), Some(Address(c))) = (param.get(0), param.get(1), param.get(2)) {
            self.program[*c] = self.get(a) * self.get(b);
        } else {
            panic!("Invalid mul(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }

    fn read(&mut self, param: Vec<Val>) {
        if let Some(Address(a)) = param.get(0) {
            self.program[*a] = self.input.remove(0)
        } else {
            panic!("Invalid mul(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }
    fn write(&mut self, param: Vec<Val>) {
        if let Some(a) = param.get(0) {
            let val = self.get(a);
            self.output.push(val)
        } else {
            panic!("Invalid mul(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }

    fn jump_if_true(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b)) = (param.get(0), param.get(1)) {
            if self.get(a) != 0 {
                self.pos = self.get(b) as usize
            }
        } else {
            panic!("Invalid jump_if_true(). Expected: Val, Val, found '{:?}' instead", param)
        }
    }
    fn jump_if_false(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b)) = (param.get(0), param.get(1)) {
            if self.get(a) == 0 {
                self.pos = self.get(b) as usize
            }
        } else {
            panic!("Invalid jump_if_false(). Expected: Val, Val, found '{:?}' instead", param)
        }
    }

    fn lt(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b), Some(Address(c))) = (param.get(0), param.get(1), param.get(2)) {
            self.program[*c] = if self.get(a) < self.get(b) { 1 } else { 0 };
        } else {
            panic!("Invalid lt(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }
    fn eq(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b), Some(Address(c))) = (param.get(0), param.get(1), param.get(2)) {
            self.program[*c] = if self.get(a) == self.get(b) { 1 } else { 0 };
        } else {
            panic!("Invalid eq(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }

    pub fn get(&self, x: &Val) -> i64 {
        match x {
            Address(i) => self.program[*i],
            Quantity(v) => *v,
        }
    }
    pub fn set(&mut self, i: &Val, val: i64) {
        if let Address(addr) = i {
            self.program[*addr] = val
        } else {
            panic!("Expected address, found '{}' instead", i)
        }
    }
    pub fn push(&mut self, val: i64) { self.program.push(val) }

    fn consume(&mut self, mut mode: i64, n: usize) -> Vec<Val> {
        let mut rv = Vec::new();
        for i in 1..=n {
            let m = mode % 10;
            match m {
                0 => rv.push(Address(self.program[self.pos+i] as usize)),
                1 => rv.push(Quantity(self.program[self.pos+i])),
                _ => panic!("Unexpected mode {}", m),
            };
            mode /= 10;
        }
        self.pos += n + 1;
        return rv;
    }
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
        let mut ic = Intcode::init(vec![ 1,0,0,0,99 ]);
        ic.run();
        assert_eq!(ic.program, vec![ 2,0,0,0,99 ]);// Example 1

        let mut ic = Intcode::init(vec![ 2,3,0,3,99 ]);
        ic.run();
        assert_eq!(ic.program, vec![ 2,3,0,6,99 ]);// Example 2

        let mut ic = Intcode::init(vec![ 2,4,4,5,99,0 ]);
        ic.run();
        assert_eq!(ic.program, vec![ 2,4,4,5,99,9801 ]);// Example 3

        let mut ic = Intcode::init(vec![ 1,1,1,4,99,5,6,0,99 ]);
        ic.run();
        assert_eq!(ic.program, vec![ 30,1,1,4,2,5,6,0,99 ]);// Example 4

        let mut ic = Intcode::load(&String::from("02.in"));
        ic.set(&Val::Address(1 as usize), 12);
        ic.set(&Val::Address(2 as usize), 2);
        ic.run();
        assert_eq!(ic.get(&Val::Address(0 as usize)), 3562672);// Part 1
    }

    #[test]
    #[should_panic(expected = "Unknown command at position 0: 77")]
    fn bad_command() {
        let mut ic = Intcode::init(vec![ 77,0,0,0,99 ]);
        ic.run();
    }
}
