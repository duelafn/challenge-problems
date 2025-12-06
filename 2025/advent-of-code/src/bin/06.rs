// SPDX-License-Identifier: MIT

use std::io::BufRead as _;

type T = Vec<String>;

struct Problem {
    values: Vec<i64>,
    operation: char,
}

impl Problem {
    fn new(operation: char) -> Self { Problem { values: Vec::new(), operation } }
    fn take(&mut self, operation: char) -> Self {
        let mut new = Problem { values: Vec::new(), operation };
        std::mem::swap(self, &mut new);
        new
    }
}


fn load<R: std::io::Read>(contents: R) -> T {
    std::io::BufReader::new(contents).lines().map(|l| l.unwrap()).collect()
}

fn checksum(data: &[Problem]) -> i64 {
    let mut rv = 0;
    for p in data {
        rv += match p.operation {
            '+' => p.values.iter().sum::<i64>(),
            '*' => p.values.iter().product::<i64>(),
            _   => panic!("bummer"),
        };
    }
    rv
}

fn part1(data: &T) -> i64 {
    let mut rows = Vec::new();
    for line in data {
        let row: Vec<String> = line.trim().split_ascii_whitespace().map(String::from).collect();
        rows.push(row);
    }

    let mut rv: Vec<Problem> = rows.pop().unwrap().into_iter()
        .map(|op| Problem::new(op.chars().next().unwrap()))
        .collect();
    for row in rows {
        for (p, val) in rv.iter_mut().zip(row.into_iter()) {
            p.values.push(val.parse().unwrap());
        }
    }

    checksum(&rv)
}

fn part2(data: &T) -> i64 {
    let mut iters: Vec<std::str::Chars<'_>> = data.iter().map(|s| s.chars()).collect();
    let mut ops = iters.pop().unwrap();

    let mut rv = Vec::new();
    let mut p = Problem::new(' ');
    loop {
        match ops.next() {
            None => break,
            Some(' ') => { }, // continue with old problem
            Some(op) => { // new problem
                if p.operation == ' ' { p = Problem::new(op); }
                else { rv.push(p.take(op)); }
            }
        }

        let mut has_value = false; // so we can detect 0
        let mut value = 0;
        for c in iters.iter_mut() {
            if let Some(n) = c.next() && let Some(n) = n.to_digit(10) {
                has_value = true;
                value = 10*value + i64::from(n);
            }
        }
        if has_value { p.values.push(value); }
    }
    if p.operation != ' ' { rv.push(p); }

    checksum(&rv)
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("06.in"));
    let rec = load(std::fs::File::open(fname).unwrap());

    println!("Part 1: {}", part1(&rec));
    println!("Part 2: {}", part2(&rec));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rec = load(r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec), 4277556);
        assert_eq!(part2(&rec), 3263827);
    }
}
