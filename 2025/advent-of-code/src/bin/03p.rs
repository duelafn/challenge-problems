// SPDX-License-Identifier: MIT
//
// 384K target/release/03
// 504K target/release/03p
//
// # target-cpu=x86-64-v2
//
// # TSALMOTH: 12th Gen Intel(R) Core(TM) i7-12700H
// $ TIME -- target/release/03
// Part 1: 16887
// Part 2: 167302518850275
//
// Average of 4642 iterations
//
// real    0m      902us
// cpu     0m      828us
//   user  0m      718us
//   sys   0m      110us
//
// cached page faults  124
//
// $ TIME -- target/release/03p
// Part 1: 16887
// Part 2: 167302518850275
//
// Average of 2531 iterations
//
// real    0m    1.638ms
// cpu     0m    5.459ms
//   user  0m    3.927ms
//   sys   0m    1.532ms
//
// cached page faults  251
// yield for I/O       74
// yield to task       29
//
//
// # JHEGAALA: Intel(R) Celeron(R) CPU  N3150  @ 1.60GHz
// $ TIME -- target/release/03
// Part 1: 16887
// Part 2: 167302518850275
//
// Average of 973 iterations
//
// real    0m    5.130ms
// cpu     0m    4.461ms
//   user  0m    1.433ms
//   sys   0m    3.28ms
//
// cached page faults  116
//
// $ TIME -- target/release/03p
// Part 1: 16887
// Part 2: 167302518850275
//
// Average of 988 iterations
//
// real    0m    4.665ms
// cpu     0m    6.9ms
//   user  0m    1.512ms
//   sys   0m    4.497ms
//
// cached page faults  142
// yield for I/O       24
// yield to task       5


use std::io::BufRead;

use rayon::prelude::{IntoParallelRefIterator as _, ParallelIterator as _};


type T = Vec<Vec<u64>>;

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        rv.push(line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect());
    }
    return rv;
}

fn part1_item(row: &[u64]) -> u64 {
    let mut i = 0;
    let mut a = row[i];
    for (idx, n) in row[0..row.len()-1].iter().copied().enumerate() {
        if n > a { i = idx; a = n; }
    }

    let mut b = row[i+1];
    for n in row[i+1..row.len()].iter().copied() {
        if n > b { b = n; }
    }
    10*a + b
}

fn part2_item(row: &[u64]) -> u64 {
    const N: usize = 12;
    let mut offset = 0;
    let mut todo = N;
    let mut val = 0;

    while todo > 0 {
        todo -= 1;
        let mut i = offset;
        let mut a = row[i];
        for (idx, n) in row[i..row.len()-todo].iter().copied().enumerate() {
            if n > a { i = offset+idx; a = n; }
        }
        offset = i + 1;
        val = 10 * val + a;
    }

    val
}

fn part1(data: &T) -> u64 {
    data.par_iter().map(|v| part1_item(&v)).sum()
}

fn part2(data: &T) -> u64 {
    data.par_iter().map(|v| part2_item(&v)).sum()
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("03.in"));
    let rec = load(std::fs::File::open(fname).unwrap());

    println!("Part 1: {}", part1(&rec));// 16887
    println!("Part 2: {}", part2(&rec));// 167302518850275
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rec = load(r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec), 357);
        assert_eq!(part2(&rec), 3121910778619);
    }
}
