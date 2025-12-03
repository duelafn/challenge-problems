// SPDX-License-Identifier: MIT

// use std::collections::HashMap;
use std::io::BufRead;

type T = Vec<Vec<u64>>;

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        rv.push(line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect());
    }
    return rv;
}

fn part1(data: &T) -> u64 {
    let mut sum = 0;
    for row in data {
        let mut i = 0;
        let mut a = row[i];
        for (idx, n) in row[0..row.len()-1].iter().copied().enumerate() {
            if n > a { i = idx; a = n; }
        }

        let mut b = row[i+1];
        for n in row[i+1..row.len()].iter().copied() {
            if n > b { b = n; }
        }
        sum += 10*a + b;
    }
    return sum;
}

fn part2(data: &T) -> u64 {
    const N: usize = 12;

    let mut sum = 0;
    for row in data {
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

        sum += val;
    }
    return sum;
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
