// SPDX-License-Identifier: MIT

use clap::{Arg, App};


fn records(fname: &str) -> Vec<i64> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.lines().enumerate().map(
        |(lineno, line)| {
            line.parse().unwrap_or_else(|err| panic!("Parse error at '{}' in {} on line {}: {}", line, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 09 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("09.in");

    let vec = records(fname);
    let mut target = 0;

    'target:
    for n in 25..vec.len() {
        target = vec[n];
        for i in n-25..n {
            for j in i+1..n {
                if target == vec[i] + vec[j] {
                    continue 'target;
                }
            }
        }
        break;
    }
    println!("Part 1: {} is not the sum of any two (25)-priors", target);

    'outer:
    for i in 0..vec.len() {
        let mut sum = vec[i];
        for j in i+1..vec.len() {
            sum += vec[j];
            if sum > target { continue 'outer; }
            if sum == target {
                let key = vec[i..=j].iter().min().unwrap() + vec[i..=j].iter().max().unwrap();
                println!("Part 2: encryption weakness is {}", key);
                break 'outer;
            }
        }
    }
}
