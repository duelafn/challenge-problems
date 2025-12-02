// SPDX-License-Identifier: MIT

// use std::collections::HashMap;
use std::io::BufRead;

type Range = core::ops::Range<u64>;

struct Accum(u64);

impl Accum {
    fn take(&mut self, val: u64) { self.0 += val; }
}

type T = Vec<Range>;

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        for range in line.split(',') {
            let Some((a, b)) = range.split_once('-') else { panic!("Bummer: '{line}'"); };
            rv.push(Range { start: a.parse().unwrap(), end: b.parse::<u64>().unwrap()+1 });
        }
    }
    return rv;
}

fn one_zeros(len: usize) -> u64 {
    10_u64.pow(u32::try_from(len-1).unwrap())
}

fn take_invalid1(range: Range, accum: &mut Accum) {
    let mut len = 1;
    let mut max = 9;
    let mut div = 1;
    for n in range {
        // No need to recompute string length a million times
        if n > max {
            len = n.to_string().len();
            max = one_zeros(len+1)-1;
            div = one_zeros(1+len/2);
        }
        if len % 2 == 0 {
            let (a, b) = (n / div, n % div);
            if a == b { accum.take(n); }
        }
    }
}

fn take_invalid2(range: Range, accum: &mut Accum) {
    let mut len = 1;
    let mut max = 9;

    'n:
    for n in range {
        // No need to recompute string length a million times
        if n > max {
            len = n.to_string().len();
            max = one_zeros(len+1)-1;
        }

        'k:
        for k in 1..len/2+1 {
            if len % k != 0 { continue; }
            let div = one_zeros(k+1);
            let val = n % div;
            let mut rem = n / div;
            if rem == 0 { panic!("not possible?"); }
            while rem != 0 {
                if (rem % div) != val { continue 'k; }
                rem /= div;
            }
            accum.take(n);
            continue 'n;
        }
    }
}

fn part1(data: &T) -> u64 {
    let mut accum = Accum(0);
    for range in data { take_invalid1(range.clone(), &mut accum); }
    accum.0
}

fn part2(data: &T) -> u64 {
    let mut accum = Accum(0);
    for range in data { take_invalid2(range.clone(), &mut accum); }
    accum.0
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("02.in"));
    let rec = load(std::fs::File::open(fname).unwrap());

    println!("Part 1: {}", part1(&rec));// 28846518423
    println!("Part 2: {}", part2(&rec));// 31578210022
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rec = load(r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec), 1227775554);
        assert_eq!(part2(&rec), 4174379265);
    }
}
