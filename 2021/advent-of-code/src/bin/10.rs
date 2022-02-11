// SPDX-License-Identifier: MIT

use std::convert::TryFrom;

enum NavError {
    Misplaced(char),
    Incomplete(String),
}

struct Nav;
impl TryFrom<&str> for Nav {
    type Error = NavError;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut expected = Vec::new();
        for ch in src.chars() {
            match ch {
                '(' => { expected.push(')'); },
                '{' => { expected.push('}'); },
                '[' => { expected.push(']'); },
                '<' => { expected.push('>'); },
                // WARNING: Assuming that everything else is a closer
                ch => match expected.pop() {
                    Some(c) if c == ch => { }, // As expected
                    Some(_)            => { return Err(NavError::Misplaced(ch)); }
                    None               => { return Err(NavError::Misplaced(ch)); }
                }
            }
        }
        if expected.len() > 0 {
            expected.reverse();
            return Err(NavError::Incomplete(expected.into_iter().collect()));
        }
        return Ok(Nav);
    }
}

fn err_points(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _   => 0,
    }
}

fn close_points(needed: &str) -> usize {
    let mut score = 0;
    for ch in needed.chars() {
        score *= 5;
        score += match ch {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _   => 0,
        };
    }
    return score;
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("10.in"));
    let contents = std::fs::read_to_string(fname).unwrap();

    let mut err_score = 0;
    let mut closes = Vec::new();
    for line in contents.lines() {
        match Nav::try_from(line) {
            Err(NavError::Misplaced(ch)) => { err_score += err_points(ch); },
            Err(NavError::Incomplete(s)) => { closes.push(close_points(&s)); },
            Ok(_) => { },
        }
    }
    println!("Part 1: {}", err_score);
    closes.sort_unstable();
    println!("Part 2: {}", closes[closes.len()/2]);
}
