// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::collections::HashMap;

use clap::{Arg, App};


#[inline]
fn walk((mut x, mut y): (i64, i64), path: &str) -> (i64, i64) {
    let mut ch = path.chars();
    loop {
        match ch.next() {
            Some('n') => match ch.next() {
                Some('e') => { x += 1; y -= 1; },
                Some('w') => { x -= 1; y -= 1; },
                Some(chr) => panic!("Unexpected direction '{}'", chr),
                None      => panic!("Unexpected end of string"),
            },
            Some('s') => match ch.next() {
                Some('e') => { x += 1; y += 1; },
                Some('w') => { x -= 1; y += 1; },
                Some(chr) => panic!("Unexpected direction '{}'", chr),
                None      => panic!("Unexpected end of string"),
            },
            Some('e') => { x += 2; },
            Some('w') => { x -= 2; },
            Some(chr) => panic!("Unexpected direction '{}'", chr),
            None      => { break; },
        }
    }
    return (x, y);
}


#[inline]
fn turn(black: &mut HashSet<(i64, i64)>) {
    let mut neighbors = HashMap::new();

    for &(x, y) in black.iter() {
        neighbors.entry((x, y)).or_insert(0); // Make sure current tile is seen during drain
        neighbors.entry((x+1, y-1)).and_modify(|e| { *e += 1 }).or_insert(1);
        neighbors.entry((x-1, y-1)).and_modify(|e| { *e += 1 }).or_insert(1);
        neighbors.entry((x+1, y+1)).and_modify(|e| { *e += 1 }).or_insert(1);
        neighbors.entry((x-1, y+1)).and_modify(|e| { *e += 1 }).or_insert(1);
        neighbors.entry((x+2, y))  .and_modify(|e| { *e += 1 }).or_insert(1);
        neighbors.entry((x-2, y))  .and_modify(|e| { *e += 1 }).or_insert(1);
    }

    for (pt, n) in neighbors.drain() {
        if black.contains(&pt) {
            if n == 0 || n > 2 { black.remove(&pt); }
        } else {
            if n == 2 { black.insert(pt); }
        }
    }
}



fn main() {
    let matches = App::new("Advent of code 2020, Day 24 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("24.in");
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let mut black = HashSet::new();
    for line in contents.lines() {
        let pt = walk((0,0), line);
        if black.take(&pt).is_none() {
            black.insert(pt);
        }
    }
    println!("Part 1: There are {} black tiles", black.len());

    for _ in 1..=100 { turn(&mut black); }
    println!("Part 2: There are {} black tiles", black.len());
}
