// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::convert::TryFrom;

use clap::{Arg, App};


struct Group(HashMap<char, u32>, u32);
impl std::convert::TryFrom<&str> for Group {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut h = HashMap::new();
        let mut members = 0;
        let mut new_entry = true;
        for ch in src.chars() {
            match ch {
                'a'..='z' => {
                    if new_entry { members += 1; new_entry = false; }
                    h.entry(ch).and_modify(|count| *count += 1).or_insert(1_u32);
                },
                '\n' => { new_entry = true; },
                _  => { return Err(format!("Unexpected character '{}'", ch)); },
            }
        }
        return Ok(Group(h, members));
    }
}
impl std::ops::Deref for Group {
    type Target = HashMap<char, u32>;
    fn deref(&self) -> &Self::Target { &self.0 }
}



fn records(fname: &str) -> Vec<Group> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.split("\n\n").enumerate().map(
        |(lineno, chunk)| {
            Group::try_from(chunk).unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 06 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("06.in");

    let answers = records(fname);
    let mut n = 0;
    for grp in answers.iter() {
        n += grp.len();
    }
    println!("Part 1: Number of 'yes': {}", n);

    let mut n = 0;
    for grp in answers.iter() {
        let size = grp.1;
        n += grp.values().filter(|&n| *n == size).count();
    }
    println!("Part 2: Number of 'all yes': {}", n);
}
