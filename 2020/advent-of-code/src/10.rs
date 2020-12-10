// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use clap::{Arg, App};


fn count_from(vec: &Vec<u64>, jolt: u64, mut start: usize, mut cache: &mut HashMap<u64, u64>) -> u64 {
    if start >= vec.len() { return 1; }
    if let Some(n) = cache.get(&jolt) { return *n; }

    let mut n = 0;
    loop {
        if start >= vec.len() { if n == 0 { n = 1; } break; }
        else if vec[start] <= jolt { start += 1; }
        else if vec[start] > jolt + 3 { break; }
        else {
            n += count_from(&vec, vec[start], start + 1, &mut cache);
            start += 1;
        }
    }
    cache.insert(jolt, n);
    return n;
}

fn records(fname: &str) -> Vec<u64> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.lines().enumerate().map(
        |(lineno, line)| {
            line.parse().unwrap_or_else(|err| panic!("Parse error at '{}' in {} on line {}: {}", line, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 10 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("10.in");
    let mut adapter = records(fname);
    adapter.sort();

    let mut step = [0; 4];
    let mut last = 0;
    for a in adapter.iter() {
        if a - last > 3 { panic!("At the disco"); }
        step[(a - last) as usize] += 1;
        last = *a;
    }
    step[3] += 1; // for the phone
    println!("Part 1: d1 * d3 = {}", step[1] * step[3]);

    println!("Part 2: {} different charging combos", count_from(&adapter, 0, 0, &mut HashMap::new()));
}
