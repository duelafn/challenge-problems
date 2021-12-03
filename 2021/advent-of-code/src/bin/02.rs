// SPDX-License-Identifier: MIT

use std::convert::TryFrom;
use std::num::ParseIntError;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Pos { x: i64, d: i64, aim: i64 }
impl Pos {
    fn new() -> Self { Pos { x: 0, d: 0, aim: 0 } }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Cmd {
    Move(i64, i64) // X, D
}
impl Cmd {
    fn apply1(&self, pos: &mut Pos) {
        use Cmd::*;
        match self {
            Move(x, d) => { pos.x += *x; pos.d += *d; }
        }
    }

    fn apply2(&self, pos: &mut Pos) {
        use Cmd::*;
        match self {
            Move(x, d) => {
                pos.x += *x;
                pos.aim += *d;
                pos.d += *x * pos.aim;
            }
        }
    }
}

impl TryFrom<&str> for Cmd {
    type Error = ParseIntError;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        if src.starts_with("forward ") { return Ok(Cmd::Move(src[8..].parse()?, 0)); }
        if src.starts_with("down ") { return Ok(Cmd::Move(0, src[5..].parse()?)); }
        if src.starts_with("up ") { return Ok(Cmd::Move(0, -src[3..].parse()?)); }
        panic!("Can't parse {}", src);
    }
}

fn records(fname: &str) -> Vec<Cmd> {
    let contents = std::fs::read_to_string(fname).unwrap();

    return contents.lines().enumerate().map(
        |(lineno, chunk)| {
            Cmd::try_from(chunk).unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("02.in"));
    let script = records(&fname);

    let mut pos = Pos::new();
    for cmd in &script {
        cmd.apply1(&mut pos);
    }
    println!("End: {:?} : product={}", pos, pos.x*pos.d);

    let mut pos = Pos::new();
    for cmd in &script {
        cmd.apply2(&mut pos);
    }
    println!("End: {:?} : product={}", pos, pos.x*pos.d);
}
