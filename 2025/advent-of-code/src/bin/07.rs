// SPDX-License-Identifier: MIT

use std::io::BufRead as _;

type T = Vec<Vec<char>>;

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        rv.push(line.chars().collect());
    }
    return rv;
}

fn part1(data: &T) -> u64 {
    let mut rv = 0;
    let mut data = data.clone();
    for i in 0..data.len()-1 {
        for j in 0..data[i].len() {
            if matches!(data[i][j], '|' | 'S') {
                match data[i+1][j] {
                    '.' => { data[i+1][j] = '|'; }
                    '^' => { data[i+1][j-1] = '|'; data[i+1][j+1] = '|'; rv += 1; }
                    _   => { }
                }
            }
        }
    }
    rv
}

fn part2(data: &T) -> u64 {
    let mut data = data.iter().map(
        |v| v.iter().copied().map(|ch| (ch, 0u64)).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    for i in 0..data.len()-1 {
        for j in 0..data[i].len() {
            if matches!(data[i][j].0, '|' | 'S') {
                match data[i+1][j].0 {
                    '.' | '|' => {
                        data[i+1][j].0 = '|';
                        data[i+1][j].1 += data[i][j].1.max(1);
                    }
                    '^' => {
                        data[i+1][j-1].0 = '|';
                        data[i+1][j-1].1 += data[i][j].1.max(1);

                        data[i+1][j+1].0 = '|';
                        data[i+1][j+1].1 += data[i][j].1.max(1);
                    }
                    _   => { }
                }
            }
        }
    }
    data[data.len()-1].iter().map(|v| v.1).sum()
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("07.in"));
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec), 21);
        assert_eq!(part2(&rec), 40);
    }
}
