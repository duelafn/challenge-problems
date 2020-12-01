// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;

use clap::{Arg, App};


fn records(fname: &str) -> Vec<i32> {
    let contents = fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.lines().enumerate().map(
        |(lineno, line)| {
            line.parse().unwrap_or_else(|err| panic!("Parse error at '{}' in {} on line {}: {}", line, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020 Solutions")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("01.in");

    let mut seen = HashMap::new();
    for n in records(fname) {
        seen.insert(n, true);
        if let Some(_) = seen.get(&(2020-n)) {
            println!("Part 1: Found {} and {} whose product is {}", n, 2020-n, n * (2020-n));
        }
    }

    for n in seen.keys() {
        for m in seen.keys() {
            if let Some(_) = seen.get(&(2020-n-m)) {
                println!("Part 2: Found {}, {}, and {} whose product is {}", n, m, 2020-n-m, n * m * (2020-n-m));
                std::process::exit(0);
            }
        }
    }
}
