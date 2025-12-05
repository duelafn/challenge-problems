// SPDX-License-Identifier: MIT

use std::io::BufRead as _;
use std::ops::Range;


#[derive(Default, Debug)]
struct T {
    fresh: Vec<Range<u64>>,
    avail: Vec<u64>,
}

impl T {
    fn count_fresh(&self) -> u64 {
        let mut n = 0;
        for id in &self.avail {
            if self.fresh.iter().any(|r| r.contains(id)) { n += 1; }
        }
        n
    }

    fn compact(&mut self) {
        let mut i = 0;
        while i < self.fresh.len()-1 {
            let mut j = i+1;
            while j < self.fresh.len() {
                let a = self.fresh[i].clone();
                let b = self.fresh[j].clone();
                if a.start < b.end && b.start < a.end { // Overlap
                    self.fresh[i] = Range {
                        start: a.start.min(b.start),
                        end:   a.end.max(b.end),
                    };
                    self.fresh.swap_remove(j);
                    j = i+1; // Changed fresh[i], it may overlap an earlier item now
                }
                else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
}

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = T::default();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        if let Some((a, b)) = line.split_once('-') {
            // Range expects end to be exclusive
            rv.fresh.push(Range {
                start: a.parse().unwrap(),
                end: 1+b.parse::<u64>().unwrap(),
            });
        }

        else if !line.is_empty() {
            rv.avail.push(line.parse().unwrap());
        }
    }
    return rv;
}

fn part1(data: &T) -> u64 {
    data.count_fresh()
}

fn part2(mut data: T) -> u64 {
    data.compact();
    data.fresh.into_iter().map(|r| r.end - r.start).sum()
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("05.in"));
    let rec = load(std::fs::File::open(fname).unwrap());

    println!("Part 1: {}", part1(&rec));// 737
    println!("Part 2: {}", part2(rec));// 357485433193284
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rec = load(r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec), 3);
        assert_eq!(part2(rec), 14);
    }
}
