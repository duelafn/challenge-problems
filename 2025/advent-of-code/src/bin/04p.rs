// SPDX-License-Identifier: MIT
//
// 384K target/release/04
// 508K target/release/04p
//
// # target-cpu=x86-64-v2
//
// # TSALMOTH: 12th Gen Intel(R) Core(TM) i7-12700H
// $ TIME -- target/release/04
// Part 1: 1464
// Part 2: 8409
//
// Average of 1671 iterations
//
// real    0m    2.884ms
// cpu     0m    2.735ms
//   user  0m    2.285ms
//   sys   0m      450us
//
// cached page faults  88
//
// $ TIME -- target/release/04p
// Part 1: 1464
// Part 2: 8409
//
// Average of 1291 iterations
//
// real    0m    3.761ms
// cpu     0m0.039s
//   user  0m0.024s
//   sys   0m0.014s
//
// cached page faults  241
// yield for I/O       154
// yield to task       100
//
//
// # JHEGAALA: Intel(R) Celeron(R) CPU  N3150  @ 1.60GHz
// $ TIME -- target/release/04
// Part 1: 1464
// Part 2: 8409
//
// Average of 396 iterations
//
// real    0m0.011s
// cpu     0m0.011s
//   user  0m    8.453ms
//   sys   0m    2.622ms
//
// resident size       2KiB
// cached page faults  80
// yield for I/O       1
//
// $ TIME -- target/release/04p
// Part 1: 1464
// Part 2: 8409
//
// Average of 332 iterations
//
// real    0m    9.644ms
// cpu     0m0.024s
//   user  0m0.016s
//   sys   0m    8.651ms
//
// resident size       1KiB
// cached page faults  114
// yield for I/O       92
// yield to task       90


use std::io::BufRead as _;

use rayon::prelude::*;
use smallvec::SmallVec;


type T = Vec<Vec<bool>>;
type Coords = SmallVec::<[(u8, u8); 28]>; // 1 line (512 bytes) and should be large enough (average row count was 14)

// USING VEC:
// type Coords = Vec<(u8, u8)>;
//
// # TSALMOTH: 12th Gen Intel(R) Core(TM) i7-12700H
// real    0m    3.672ms
// cpu     0m0.038s
//   user  0m0.023s
//   sys   0m0.015s
//
// cached page faults  234
// yield for I/O       194
// yield to task       207
//
// # JHEGAALA: Intel(R) Celeron(R) CPU  N3150  @ 1.60GHz
// real    0m    9.777ms
// cpu     0m0.025s
//   user  0m0.016s
//   sys   0m    8.817ms
//
// cached page faults  115
// yield for I/O       96
// yield to task       89


static DIRECTIONS : [(isize,isize); 8] = [
    (-1,-1), ( 0,-1), ( 1,-1),
    (-1, 0),          ( 1, 0),
    (-1, 1), ( 0, 1), ( 1, 1),
];

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        assert!(line.len() <= u8::MAX as usize);
        rv.push(line.chars().map(|ch| ch == '@').collect());
    }
    assert!(rv.len() <= u8::MAX as usize);
    return rv;
}

fn count_neighbors(v: &T, a: usize, b: usize) -> u64 {
    let mut rv = 0;
    for (da, db) in DIRECTIONS {
        let i = (a as isize) + da;
        let j = (b as isize) + db;
        if i >= 0 && j >= 0
            && let Some(row) = v.get(i as usize)
            && let Some(occupied) = row.get(j as usize)
            && *occupied
        {
            rv += 1;
        }
    }
    rv
}

fn count_reachable(data: &T, row: usize) -> u64 {
    let mut rv = 0;
    for (col, val) in data[row].iter().enumerate() {
        if *val && count_neighbors(data, row, col) < 4 {
            rv += 1;
        }
    }
    rv
}

fn part1(data: &T) -> u64 {
    (0..data.len()).into_par_iter().map(|row| count_reachable(data, row)).sum()
}

fn list_reachable(data: &T, row: usize) -> Coords {
    let mut rv = Coords::new();
    for (col, val) in data[row].iter().enumerate() {
        if *val && count_neighbors(data, row, col) < 4 {
            rv.push((row as u8, col as u8));
        }
    }
    rv
}

fn part2(mut data: T) -> u64 {
    let mut rv = 0;
    let mut changed = true;
    while changed {
        changed = false;
        let res: Vec<_> = (0..data.len()).into_par_iter().map(|row| list_reachable(&data, row)).collect();
        for coords in res {
            for (i, j) in coords {
                rv += 1;
                data[usize::from(i)][usize::from(j)] = false;
                changed = true;
            }
        }
    }
    rv
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("04.in"));
    let rec = load(std::fs::File::open(fname).unwrap());

    println!("Part 1: {}", part1(&rec));
    println!("Part 2: {}", part2(rec));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rec = load(r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec), 13);
        assert_eq!(part2(rec), 43);
    }
}
