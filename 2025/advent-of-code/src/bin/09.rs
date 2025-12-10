// SPDX-License-Identifier: MIT

use std::io::BufRead as _;

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
fn is_green(data: &T, a: &Point, b: &Point) -> bool {
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
