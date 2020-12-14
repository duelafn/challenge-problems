// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::convert::TryFrom;

use clap::{Arg, App};

use Instruction::*;



enum Instruction {
    SetMask(Mask),
    SetReg(u64, u64),
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mask {
    and: u64,
    or:  u64,
    x: Vec<u8>,
}
impl Mask {
    pub fn new() -> Mask {
        Mask { and: 0, or: 0, x: vec![] }
    }

    #[inline]
    pub fn apply_v1(&self, v: u64) -> u64 { (v | self.or) & self.and }

// Average of 2105 iterations
// real    0m    8.900ms
// user    0m    7.171ms
// sys     0m    1.648ms
// cached page faults  1,177
    pub fn apply_v2(&self, v: u64) -> Vec<u64> {
        let mut rv = vec![0; 1<<(self.x.len())];
        rv[0] = v | self.or;
        for (i, idx) in self.x.iter().enumerate() {
            let m: u64 = 1 << *idx;
            let mut end = (1<<(i+1)) - 1;
            let mut n = 1<<i;
            while n > 0 { // about 75000 iterations total
                n -= 1;
                rv[end] = rv[n]|m;    end -= 1;
                rv[end] = rv[n]&(!m); end -= 1;
            }
        }
        return rv;
    }

// Average of 341 iterations
// real    0m    9.187ms
// user    0m    7.467ms
// sys     0m    1.641ms
// cached page faults  1,185
//
// Average of 2155 iterations
// real    0m    9.68ms
// user    0m    7.208ms
// sys     0m    1.779ms
    pub fn apply_v2_a(&self, v: u64) -> Vec<u64> {
        let mut rv = vec![ v | self.or ];
        for i in self.x.iter() {
            let mut tmp = Vec::with_capacity(rv.len() * 2);
            let m: u64 = 1 << *i;
            for v in rv {
                tmp.push(v|m);
                tmp.push(v&(!m));
            }
            rv = tmp;
        }
        return rv;
    }
}
impl std::convert::TryFrom<&str> for Mask {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let (mut a, mut o) = (0, 0);
        let mut vec = Vec::new();
        for (i, ch) in src.chars().rev().enumerate() {
            match ch {
                '0' => { },
                '1' => { o |= 1 << i; a |= 1 << i; },
                'X' => { a |= 1 << i; vec.push(i as u8); },
                _   => { return Err(format!("Unexpected character '{}'", ch)) },
            }
        }
        return Ok(Mask { and: a, or: o, x: vec});
    }
}



fn records(fname: &str) -> Vec<Instruction> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.lines().enumerate().map(
        |(lineno, chunk)| {
            if chunk.starts_with("mask = ") {
                SetMask(Mask::try_from(&chunk[7..]).unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err)))
            } else {
                let v: Vec<&str> = chunk.split("] = ").collect();
                let reg = (v.get(0).unwrap_or_else(|| panic!("Parse error at '{}' in {} in record {}", chunk, fname, lineno+1)))[4..]
                            .parse().unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err));
                let val = (v.get(1).unwrap_or_else(|| panic!("Parse error at '{}' in {} in record {}", chunk, fname, lineno+1)))
                            .parse().unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err));
                SetReg(reg, val)
            }
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 14 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("14.in");

    let setup = records(fname);
    let mut reg = HashMap::new();
    let mut mask = Mask::new();
    for cmd in setup.iter() {
        match cmd {
            SetMask(m)   => { mask = m.clone(); },
            SetReg(k, v) => { reg.insert(*k, mask.apply_v1(*v)); },
        }
    }
    println!("Part 1: Sum {}", reg.values().sum::<u64>());

    let mut reg = HashMap::new();
    let mut mask = Mask::new();
    for cmd in setup.iter() {
        match cmd {
            SetMask(m)   => { mask = m.clone(); },
            SetReg(k, v) => { for addr in mask.apply_v2(*k) { reg.insert(addr, *v); } },
        }
    }
    println!("Part 2: Sum {}", reg.values().sum::<u64>());
}
