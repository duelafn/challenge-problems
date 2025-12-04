// SPDX-License-Identifier: MIT

use std::io::BufRead as _;

type T = Vec<Vec<bool>>;

static DIRECTIONS : [(isize,isize); 8] = [
    (-1,-1), ( 0,-1), ( 1,-1),
    (-1, 0),          ( 1, 0),
    (-1, 1), ( 0, 1), ( 1, 1),
];

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        rv.push(line.chars().map(|ch| ch == '@').collect());
    }
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

fn part1(data: &T) -> u64 {
    let mut rv = 0;
    let rows = data.len();
    let cols = data[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if data[row][col] && count_neighbors(data, row, col) < 4 {
                rv += 1;
            }
        }
    }
    rv
}

fn part2(mut data: T) -> u64 {
    let mut rv = 0;
    let rows = data.len();
    let cols = data[0].len();
    let mut changed = true;
    while changed {
        changed = false;
        for row in 0..rows {
            for col in 0..cols {
                if data[row][col] && count_neighbors(&data, row, col) < 4 {
                    rv += 1;
                    data[row][col] = false;
                    changed = true;
                }
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
