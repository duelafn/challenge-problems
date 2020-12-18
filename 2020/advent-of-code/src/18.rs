// SPDX-License-Identifier: MIT

use clap::{Arg, App};

use Expr::*;


#[derive(Debug, Clone)]
pub enum Expr {
    Mul(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Num(u64),
}
impl Expr {
    fn eval(&self) -> u64 {
        match self {
            Num(x)    => *x,
            Add(a, b) => a.eval() + b.eval(),
            Mul(a, b) => a.eval() * b.eval(),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num(x)   => write!(f, "{}", x),
            Add(a,b) => write!(f, "({} + {})", a, b),
            Mul(a,b) => write!(f, "({} * {})", a, b),
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum Math {
    Left,
    Right,
    LeftAdd,
    RightAdd,
}

pub struct ParseStr {
    s: Vec<char>,
    i: usize,
    math: Math
}
impl ParseStr {
    pub fn new(s: &str, math: Math) -> Self {
        ParseStr { math, s: s.chars().filter(|x| !x.is_ascii_whitespace()).collect(), i: 0 }
    }

    #[inline]
    pub fn peek(&self)     -> Option<char> {
        if self.i < self.s.len() { Some(self.s[self.i]) }
        else { None }
    }

    #[inline]
    pub fn next(&mut self) -> Option<char> {
        if self.i < self.s.len() { self.i += 1; Some(self.s[self.i-1]) }
        else { None }
    }

    #[inline]
    pub fn at(&mut self, ch: char) -> bool { self.i < self.s.len() && ch == self.s[self.i] }

    #[inline]
    pub fn step(&mut self) { self.i += 1; }

    pub fn parse(&mut self) -> Result<Expr, String> { self.extract_group(false) }


    #[inline]
    fn err(&self, msg: &str) -> String {
        if self.i < self.s.len() {
            format!("Parse error at column {}, {}, found {}", self.i, msg, self.s[self.i..].iter().collect::<String>())
        } else {
            format!("Parse error at end of line, {}", msg)
        }
    }

    #[inline]
    fn extract_factor(&mut self) -> Result<Expr, String> {
        match self.peek() {
            None      => Err(self.err("unexpected end of input")),
            Some('(') => self.extract_group(true),
            Some(chr) if chr >= '0' && chr <= '9' => { self.step(); Ok(Num((chr as u64) - 48)) },
            Some(_ch) => Err(self.err("expected a factor")),
        }
    }

    #[inline]
    fn extract_operator(&mut self) -> Result<char, String> {
        match self.next() {
            None      => Err(self.err("unexpected end of input")),
            Some('+') => Ok('+'),
            Some('*') => Ok('*'),
            Some(_ch) => Err(self.err("expected an operator")),
        }
    }

    #[inline]
    fn extract_lparen(&mut self) -> Result<char, String> {
        match self.next() {
            None      => Err(self.err("unexpected end of input")),
            Some('(') => Ok('('),
            Some(_ch) => Err(self.err("expected open paren")),
        }
    }

    #[inline]
    fn extract_rparen(&mut self) -> Result<char, String> {
        match self.next() {
            None      => Err(self.err("unexpected end of input")),
            Some(')') => Ok(')'),
            Some(_ch) => Err(self.err("expected close paren")),
        }
    }

    fn extract_group(&mut self, pgroup: bool) -> Result<Expr, String> {
        if pgroup { self.extract_lparen()?; }
        let mut ex = Vec::new();
        let mut op = Vec::new();
        ex.push(self.extract_factor()?);
        while self.at('+') || self.at('*') {
            op.push(self.extract_operator()?);
            ex.push(self.extract_factor()?);
        }
        if pgroup { self.extract_rparen()?; }

        // We're certain about array lengths here, so unwrap at will.
        match self.math {
            Math::Left => {
                let mut ex = ex.drain(..);
                let mut rv = ex.next().unwrap();
                for op in op.iter() {
                    match op {
                        '+' => { rv = Add(Box::new(rv), Box::new(ex.next().unwrap())); },
                        '*' => { rv = Mul(Box::new(rv), Box::new(ex.next().unwrap())); },
                        _ch => { unreachable!("Inconceivable!"); }
                    }
                }
                return Ok(rv);
            },

            Math::Right => {
                let mut ex = ex.drain(..).rev();
                let mut rv = ex.next().unwrap();
                for op in op.iter().rev() {
                    match op {
                        '+' => { rv = Add(Box::new(ex.next().unwrap()), Box::new(rv)); },
                        '*' => { rv = Mul(Box::new(ex.next().unwrap()), Box::new(rv)); },
                        _ch => { unreachable!("Inconceivable!"); }
                    }
                }
                return Ok(rv);
            },

            Math::LeftAdd => {
                loop {
                    let idx = op.iter().position(|&s| s == '+');
                    match idx {
                        None => { break; },
                        Some(idx) => {
                            op.remove(idx);
                            let l = ex.remove(idx);
                            ex[idx] = Add(Box::new(l), Box::new(ex[idx].clone()));
                        }
                    }
                }
                let mut ex = ex.drain(..);
                let mut rv = ex.next().unwrap();
                for f in ex { rv = Mul(Box::new(rv), Box::new(f)); }
                return Ok(rv);
            },

            Math::RightAdd => {
                loop {
                    let idx = op.iter().rposition(|&s| s == '+');
                    match idx {
                        None => { break; },
                        Some(idx) => {
                            op.remove(idx);
                            let l = ex.remove(idx);
                            ex[idx] = Add(Box::new(l), Box::new(ex[idx].clone()));
                        }
                    }
                }
                let mut ex = ex.drain(..).rev();
                let mut rv = ex.next().unwrap();
                for f in ex { rv = Mul(Box::new(f), Box::new(rv)); }
                return Ok(rv);
            },
        }
    }
}



fn records(fname: &str, math: Math) -> Vec<Expr> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.lines().enumerate().map(
        |(lineno, chunk)| {
            ParseStr::new(chunk, math).parse().unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}:\n  {}", chunk, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 18 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("18.in");

    let homework = records(fname, Math::Left);
//     for p in homework { println!("{} => {}", p, p.eval()); }
    println!("Part 1: sum = {}", homework.iter().map(|p| p.eval()).sum::<u64>());

    let homework = records(fname, Math::LeftAdd);
    println!("Part 2: sum = {}", homework.iter().map(|p| p.eval()).sum::<u64>());
}
