
pub mod util;

use std::fmt;
use std::fs;

use std::ops::{Index,IndexMut};

type IntcodeWord = i64;

pub enum Val {
    Address(usize),
    Quantity(IntcodeWord),
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
    program: Vec<IntcodeWord>,
    input: Vec<IntcodeWord>,
    output: Vec<IntcodeWord>,
    relative_base: usize,
    pos: usize,
    nbread: Option<IntcodeWord>,
    nbread_count: usize,
}

impl Intcode {
    pub fn new() -> Intcode {
        return Intcode { program: Vec::new(), pos: 0, relative_base: 0, input: Vec::new(), output: Vec::new(), nbread: None, nbread_count: 0 };
    }

    pub fn init(prog: Vec<IntcodeWord>) -> Intcode {
        return Intcode { program: prog, pos: 0, relative_base: 0, input: Vec::new(), output: Vec::new(), nbread: None, nbread_count: 0 };
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

    pub fn is_halted(&self) -> bool { 99 == self[self.pos] }

    pub fn run(&mut self) {
        while self.step() { }
    }

    pub fn step(&mut self) -> bool {
        let code = self[self.pos];
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
            9 => { let v = self.consume(mode, 1); self.shift_relative_base(v) },

            99 => return false,
            x_ => panic!("Unknown command at position {}: {} ({})", self.pos, x_, self),
        };
        return true;
    }

    pub fn peek(&mut self) -> Vec<IntcodeWord> { self.input.clone() }
    pub fn pipe(&mut self, val: IntcodeWord) { self.input.push(val); }
    pub fn input_len(&mut self) -> usize { self.input.len() }
    pub fn nbinput(&mut self, val: Option<IntcodeWord>) { self.nbread = val.clone() }
    pub fn nbread_count(&self) -> usize { self.nbread_count }
    pub fn set_input(&mut self, val: IntcodeWord) {
        if self.input.len() > 0 {
            self.input[0] = val;
        } else {
            self.input.push(val);
        }
    }

    pub fn cat(&mut self) -> Vec<IntcodeWord> { self.output.clone() }
    pub fn output_len(&mut self) -> usize { self.output.len() }
    pub fn has_output(&mut self) -> bool { self.output.len() > 0 }
    pub fn shift_output(&mut self) -> Option<IntcodeWord> {
        if self.output.len() > 0 {
            return Some(self.output.remove(0))
        } else {
            return None
        }
    }

    pub fn ascii_in(&mut self, input: &String) {
        for ch in input.chars() {
            self.pipe(ch as i64);
        }
    }
    pub fn ascii_out(&mut self) -> String {
        self.output.iter().map(|c| std::char::from_u32(*c as u32).unwrap_or_else(|| panic!("Expected ASCII"))).collect::<String>()
    }

    fn add(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b), Some(Address(c))) = (param.get(0), param.get(1), param.get(2)) {
            if let Some(rv) = self.get(a).checked_add(self.get(b)) {
                self[*c] = rv;
            } else { panic!("Overflow in add()") }
        } else {
            panic!("Invalid add(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }
    fn mul(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b), Some(Address(c))) = (param.get(0), param.get(1), param.get(2)) {
            if let Some(rv) = self.get(a).checked_mul(self.get(b)) {
                self[*c] = rv;
            } else { panic!("Overflow in mul()") }
        } else {
            panic!("Invalid mul(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }

    fn read(&mut self, param: Vec<Val>) {
        if let Some(Address(a)) = param.get(0) {
            if self.input.len() > 0 {
                self.nbread_count = 0;
                self[*a] = self.input.remove(0);
            } else if let Some(dflt) = self.nbread {
                self.nbread_count += 1;
                self[*a] = dflt;
            } else {
                panic!("Read from empty input queue!");
            }
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
            self[*c] = if self.get(a) < self.get(b) { 1 } else { 0 };
        } else {
            panic!("Invalid lt(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }
    fn eq(&mut self, param: Vec<Val>) {
        if let (Some(a), Some(b), Some(Address(c))) = (param.get(0), param.get(1), param.get(2)) {
            self[*c] = if self.get(a) == self.get(b) { 1 } else { 0 };
        } else {
            panic!("Invalid eq(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }

    fn checked_add_to_relative(&self, val: IntcodeWord) -> usize {
        let rv = match val {
            x @ 0..=std::i64::MAX  => self.relative_base.checked_add(x as usize),
            x @ std::i64::MIN..=-1 => self.relative_base.checked_sub((-x) as usize),
        };
        if let Some(new) = rv {
            return new;
        } else { panic!("Overflow in relative_base") }
    }

    fn shift_relative_base(&mut self, param: Vec<Val>) {
        if let Some(a) = param.get(0) {
            self.relative_base = self.checked_add_to_relative(self.get(a));
        } else {
            panic!("Invalid eq(). Expected: Val, Val, Address, found '{:?}' instead", param)
        }
    }

    pub fn get(&self, x: &Val) -> IntcodeWord {
        match x {
            Address(i) => self[*i],
            Quantity(v) => *v,
        }
    }
    pub fn set(&mut self, i: &Val, val: IntcodeWord) {
        if let Address(addr) = i {
            self[*addr] = val
        } else {
            panic!("Expected address, found '{}' instead", i)
        }
    }
    pub fn push(&mut self, val: IntcodeWord) { self.program.push(val) }

    fn consume(&mut self, mut mode: IntcodeWord, n: usize) -> Vec<Val> {
        let mut rv = Vec::new();
        for i in 1..=n {
            let m = mode % 10;
            match m {
                0 => rv.push(Address(self[self.pos+i] as usize)),
                1 => rv.push(Quantity(self[self.pos+i])),
                2 => rv.push(Address(self.checked_add_to_relative(self[self.pos+i]))),
                _ => panic!("Unexpected mode {}", m),
            };
            mode /= 10;
        }
        match self.pos.checked_add(n + 1) {
            Some(rv) => self.pos = rv,
            None     => panic!("Program overflow!"),
        }
        return rv;
    }
}

impl Index<usize> for Intcode {
    type Output = IntcodeWord;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.program.len() { &self.program[index] } else { &0 }
    }
}
impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.program.len() {
            let mut zeroes = vec![0; index + 1 - self.program.len()];
            self.program.append(&mut zeroes);
        }
        return &mut self.program[index]
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

    #[test]
    fn day09() {
        let mut ic = Intcode::init(vec![ 109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99 ]);
        ic.run();
        assert_eq!(ic.cat(), vec![ 109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99 ]);// Example 1
    }

}
