// SPDX-License-Identifier: MIT
//
// 383K target/release/02
// 502K target/release/02p
//
// # target-cpu=x86-64-v2
//
// # TSALMOTH: 12th Gen Intel(R) Core(TM) i7-12700H
// $ TIME -- target/release/02
// Part 1: 28846518423
// Part 2: 31578210022
//
// Average of 159 iterations
//
// real    0m0.032s
// cpu     0m0.032s
//   user  0m0.032s
//   sys   0m      121us
//
// cached page faults  80
// yield for I/O       1
// yield to task       2
//
// $ TIME -- target/release/02p
// Part 1: 28846518423
// Part 2: 31578210022
//
// Average of 671 iterations
//
// real    0m0.010s
// cpu     0m0.089s
//   user  0m0.085s
//   sys   0m    4.123ms
//
// cached page faults  202
// yield for I/O       149
// yield to task       26
//
//
// # JHEGAALA: Intel(R) Celeron(R) CPU  N3150  @ 1.60GHz
// $ TIME -- target/release/02
// Part 1: 28846518423
// Part 2: 31578210022
//
// Average of 15 iterations
//
// real    0m0.316s
// cpu     0m0.315s
//   user  0m0.312s
//   sys   0m    2.277ms
//
// resident size       65KiB
// cached page faults  73
// yield for I/O       1
// yield to task       2
//
// $ TIME -- target/release/02p
// Part 1: 28846518423
// Part 2: 31578210022
//
// Average of 48 iterations
//
// real    0m0.097s
// cpu     0m0.317s
//   user  0m0.311s
//   sys   0m    5.755ms
//
// resident size       12KiB
// cached page faults  100
// yield for I/O       34
// yield to task       34


use std::io::BufRead;

use rayon::prelude::{IntoParallelRefIterator as _, ParallelIterator as _};


type Range = core::ops::Range<u64>;

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

fn take_invalid1(range: Range) -> u64 {
    let mut len = 1;
    let mut max = 9;
    let mut div = 1;
    let mut sum = 0;
    for n in range {
        // No need to recompute string length a million times
        if n > max {
            len = n.to_string().len();
            max = one_zeros(len+1)-1;
            div = one_zeros(1+len/2);
        }
        if len % 2 == 0 {
            let (a, b) = (n / div, n % div);
            if a == b { sum += n; }
        }
    }
    sum
}

fn take_invalid2(range: Range) -> u64 {
    let mut len = 1;
    let mut max = 9;
    let mut sum = 0;

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
            sum += n;
            continue 'n;
        }
    }
    sum
}

fn part1(data: &T) -> u64 {
    data.par_iter().cloned().map(take_invalid1).sum()
}

fn part2(data: &T) -> u64 {
    data.par_iter().cloned().map(take_invalid2).sum()
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
