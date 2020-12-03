// SPDX-License-Identifier: MIT

use std::convert::TryFrom;

use clap::{Arg, App};


struct Passwd {
    n1: usize,
    n2: usize,
    letter: char,
    passwd: String,
}

impl Passwd {
    fn is_valid_1(&self) -> bool {
        let mut n = 0;
        for ch in self.passwd.chars() {
            if ch == self.letter { n += 1; }
        }
        return n >= self.n1 && n <= self.n2;
    }

    fn is_valid_2(&self) -> bool {
        let mut matches = 0;
        if let Some(ch) = self.passwd.chars().nth(self.n1-1) {
            if ch == self.letter {
                matches += 1;
            }
        }
        if let Some(ch) = self.passwd.chars().nth(self.n2-1) {
            if ch == self.letter {
                matches += 1;
            }
        }
        return 1 == matches;
    }
}

impl std::convert::TryFrom<&str> for Passwd {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let (n1, n2, letter, passwd);
        let mut idx;
        if let Some(n) = src.find('-') {
            n1 = src[0..n].parse::<usize>().or_else(|err| Err(err.to_string()))?;
            idx = n + 1;
        } else { return Err(String::from("Can't find '-'")); }
        if let Some(n) = src.find(' ') {
            n2 = src[idx..n].parse::<usize>().or_else(|err| Err(err.to_string()))?;
        } else { return Err(String::from("Can't find ' '")); }
        if let Some(n) = src.find(':') {
            letter = src.chars().nth(n-1).ok_or("Char error")?;
            idx = n + 2;
        } else { return Err(String::from("Can't find ':'")); }
        passwd = String::from(&src[idx..]);

        return Ok(Passwd { n1, n2, letter, passwd });
    }
}



fn records(fname: &str) -> Vec<Passwd> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.lines().enumerate().map(
        |(lineno, line)| {
            Passwd::try_from(line).unwrap_or_else(|err| panic!("Parse error at '{}' in {} on line {}: {}", line, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 02 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("02.in");

    let passwords = records(fname);
    let mut part1_valid = 0;
    for p in passwords.iter() {
        if p.is_valid_1() { part1_valid += 1; }
    }
    println!("Part 1: {} passwords are valid", part1_valid);

    let mut part2_valid = 0;
    for p in passwords.iter() {
        if p.is_valid_2() { part2_valid += 1; }
    }
    println!("Part 2: {} passwords are valid", part2_valid);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Passwd::try_from("1-3 a: abcde").unwrap().is_valid_1());
        assert!(!Passwd::try_from("1-3 b: cdefg").unwrap().is_valid_1());
        assert!(Passwd::try_from("2-9 c: ccccccccc").unwrap().is_valid_1());
    }

    #[test]
    fn test2() {
        assert!(Passwd::try_from("1-3 a: abcde").unwrap().is_valid_2());
        assert!(!Passwd::try_from("1-3 b: cdefg").unwrap().is_valid_2());
        assert!(!Passwd::try_from("2-9 c: ccccccccc").unwrap().is_valid_2());
    }
}
