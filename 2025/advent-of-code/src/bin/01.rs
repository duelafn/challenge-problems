// SPDX-License-Identifier: MIT

// use std::collections::HashMap;
use std::io::BufRead;

type T = Vec<Turn>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Left(i64),
    Right(i64),
}

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        let mut iter = line.chars();
        let dir = iter.next().unwrap();
        let num = iter.as_str().parse::<i64>().unwrap();
        if num <= 0 { panic!("unexpected {num}"); }
        match dir {
            'L' => { rv.push(Turn::Left(num)); }
            'R' => { rv.push(Turn::Right(num)); }
            _   => panic!("Bummer: '{dir}'"),
        }
    }
    return rv;
}

fn part1(data: &T) -> i64 {
    let mut count = 0;
    let mut pos = 50;
    for t in data {
        match *t {
            Turn::Left(n)  => { pos = (pos - n) % 100 }
            Turn::Right(n) => { pos = (pos + n) % 100 }
        }
        if pos == 0 { count += 1; }
    }
    count
}

/// Method CLICK
fn part2(data: &T) -> i64 {
    let mut count = 0;
    let mut pos = 50;
    for t in data {
        let mut was_zero = pos == 0;
        match *t {
            Turn::Left(n)  => { pos = pos - n; }
            Turn::Right(n) => { pos = pos + n; }
        }
        if pos >= 100 {
            while pos >= 100 {
                pos -= 100;
                count += 1;
            }
        }
        else if pos < 0 {
            while pos < 0 {
                if !was_zero { count += 1; }
                pos += 100;
                was_zero = false;
            }
            if pos == 0 { count += 1; }
        }
        else if pos == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("01.in"));
    let rec = load(std::fs::File::open(fname).unwrap());

    println!("Part 1: {}", part1(&rec));// 1084
    println!("Part 2: {}", part2(&rec));// 6475
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rec = load(r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec), 3);
        assert_eq!(part2(&rec), 6);
    }
}
