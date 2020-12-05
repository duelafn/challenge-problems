// SPDX-License-Identifier: MIT

use clap::{Arg, App};


pub fn seat_id(pass: &str) -> u16 {
    let (mut a, mut b) = (0, 128);
    let (mut c, mut d) = (0, 8);
    for ch in pass.chars() {
        match ch {
            'F' => { b = (a + b) / 2; },
            'B' => { a = (a + b) / 2; },
            'L' => { d = (c + d) / 2; },
            'R' => { c = (c + d) / 2; },
            ccc => { panic!("Unexpected character: '{}'", ccc); }
        }
    }
    return a * 8 + c;
}

fn records(fname: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.lines().map(|x| String::from(x)).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 05 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("05.in");

    let passes = records(fname);
    let mut seen = vec![false; 1024];
    let mut max_id = 0;
    for pass in passes.iter() {
        let id = seat_id(pass);
        if id > max_id { max_id = id; }
        seen[id as usize] = true;
    }
    println!("Part 1: Max seat ID is {}", max_id);

    for id in 1..(seen.len() - 1) {
        if !seen[id] && seen[id-1] && seen[id+1] {
            println!("Part 2: My seat ID is {}", id);
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_1() {
        assert_eq!(seat_id(&"BFFFBBFRRR"), 567);
        assert_eq!(seat_id(&"FFFBBBFRRR"), 119);
        assert_eq!(seat_id(&"BBFFBBFRLL"), 820);
    }
}
