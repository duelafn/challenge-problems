// SPDX-License-Identifier: MIT
//
// 384K target/release/09
// 504K target/release/09p
//
// # target-cpu=x86-64-v2
//
// # TSALMOTH: 12th Gen Intel(R) Core(TM) i7-12700H
// $ TIME -- target/release/09
// Part 1: 4771508457
// Part 2: 1539809693
//
// real    4m58.218s
// cpu     4m58.205s
//   user  4m58.205s
//   sys   0m      0us
//
// resident size       5.902 MiB
// cached page faults  81
// yield for I/O       1
// yield to task       2,437
//
// $ TIME -- target/release/09p
// Part 1: 4771508457
// Part 2: 1539809693
//
// real    0m13.281s
// cpu     4m19.551s
//   user  4m14.632s
//   sys   0m4.918s
//
// resident size       5.754 MiB
// cached page faults  355
// yield for I/O       154,919
// yield to task       197,339
//
//
// # JHEGAALA: Intel(R) Celeron(R) CPU  N3150  @ 1.60GHz
// $ TIME -- target/release/09
// real    49m5.203s
// cpu     49m5.176s
//   user  49m5.148s
//   sys   0m0.027s
//
// resident size       1.864MiB
// cached page faults  76
// yield for I/O       3
// yield to task       24,950
//
// $ TIME -- target/release/09p
// real    9m4.147s
// cpu     35m48.457s
//   user  35m40.107s
//   sys   0m8.349s
//
// resident size       2.012MiB
// cached page faults  121
// yield for I/O       152452
// yield to task       308697
//
//
// # TSALMOTH: 12th Gen Intel(R) Core(TM) i7-12700H
// $ part2s(), parallelism hoisted to outer loop
// real    0m43.360s
// cpu     13m32.511s
//   user  13m32.467s
//   sys   0m0.043s


use std::io::BufRead as _;

use rayon::prelude::*;

type Point = [i64; 2];
type T = Vec<Point>;

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        let mut iter = line.split(',');
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        rv.push([a,b]);
    }
    return rv;
}

fn area(a: &Point, b: &Point) -> u64 {
    (a[0].abs_diff(b[0])+1)
        * (a[1].abs_diff(b[1])+1)
}

fn part1(data: &T) -> u64 {
    let mut max = 0;
    for (i, a) in data[0..data.len()-1].iter().enumerate() {
        for b in data[i..data.len()].iter() {
            max = max.max(area(a, b));
        }
    }
    max
}

fn is_inside(data: &T, x: i64, y: i64) -> bool {
    let mut n = 0;
    let mut pos = data[data.len()-1];
    for p in data {
        if pos[0] == p[0] { // vertical
            if pos[0] < x { // potential ray crossing
                if (pos[1] < y) != (p[1] < y) { n += 1 }
            }
            // on the line
            else if pos[0] == x && ((pos[1] < y) != (p[1] < y)) { return true; }
            else { }
        }
        else if pos[1] == p[1] { // horizontal
            // on the line
            if pos[1] == y && ((pos[0] < x) != (p[0] < x))
            { return true; }
        }
        else { panic!("assumption violation"); }

        pos = *p;
    }
    return 1 == n % 2;
}

// Checking edges necessary (protect against U protrusions) but sufficient
fn is_green_s(data: &T, a: &Point, b: &Point) -> bool {
    let (x0, x1) = if a[0] > b[0] { (b[0], a[0]) } else { (a[0], b[0]) };
    let (y0, y1) = if a[1] > b[1] { (b[1], a[1]) } else { (a[1], b[1]) };
    // verticals
    for y in y0..y1+1 {
        if !(is_inside(data, x0, y) && is_inside(data, x1, y)) { return false; }
    }
    // horizontals
    for x in x0+1..x1 {
        if !(is_inside(data, x, y0) && is_inside(data, x, y1)) { return false; }
    }
    return true;
}

// Checking edges necessary (protect against U protrusions) but sufficient
fn is_green(data: &T, a: &Point, b: &Point) -> bool {
    let (x0, x1) = if a[0] > b[0] { (b[0], a[0]) } else { (a[0], b[0]) };
    let (y0, y1) = if a[1] > b[1] { (b[1], a[1]) } else { (a[1], b[1]) };
    // verticals
    if !(y0..y1+1).into_par_iter().all(|y| {
        is_inside(data, x0, y) && is_inside(data, x1, y)
    }) { return false; }
    // horizontals
    if !(x0+1..x1).into_par_iter().all(|x| {
        is_inside(data, x, y0) && is_inside(data, x, y1)
    }) { return false; }
    return true;
}

fn part2(data: &T) -> u64 {
    let mut max = 0;
    for (i, a) in data[0..data.len()-1].iter().enumerate() {
        for b in data[i..data.len()].iter() {
            if is_green(data, a, b) {
                max = max.max(area(a, b));
            }
        }
    }
    max
}

fn part2s(data: &T) -> u64 {
    data[0..data.len()-1].par_iter().enumerate().map(|(i, a)| {
        let mut max = 0;
        for b in data[i..data.len()].iter() {
            if is_green_s(data, a, b) {
                max = max.max(area(a, b));
            }
        }
        max
    }).max().unwrap_or_default()
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("09.in"));
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec), 50);
        assert_eq!(part2(&rec), 24);
    }
}
