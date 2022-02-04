// SPDX-License-Identifier: MIT

use std::collections::HashSet;

type Word = HashSet<char>;

// 0: 6   remnant
// 1: 2   unique
// 2: 5   remnant
// 3: 5   len(x cap 1) == 2
// 4: 4   unique
// 5: 5   len(x - 6) == 0
// 6: 6   len(7 - x) == 1
// 7: 3   unique
// 8: 7   unique
// 9: 6   len(x cap 3) == 5

fn solve<'a>(seq: &'a [Word]) -> Vec<(&'a Word, usize)> {
    let mut known: Vec<Option<&Word>> = vec![None; 10];

    // Grab those unique by length
    for word in seq.iter() {
        match word.len() {
            2 => { known[1] = Some(word); },
            3 => { known[7] = Some(word); },
            4 => { known[4] = Some(word); },
            7 => { known[8] = Some(word); },
            _ => { },
        }
    }

    // Known: 1, 4, 7, 8
    // Can solve for: 3, 6
    for word in seq.iter() {
        match word.len() {
            5 => if word.intersection(known[1].unwrap()).count() == 2 { known[3] = Some(word); },
            6 => if known[7].unwrap().difference(word).count() == 1   { known[6] = Some(word); },
            _ => { },
        }
    }

    // Known: 1, 4, 7, 8; 3, 6
    // Can solve for: 5, 9
    for word in seq.iter() {
        match word.len() {
            5 => if word.difference(known[6].unwrap()).count() == 0   { known[5] = Some(word); },
            6 => if word.intersection(known[3].unwrap()).count() == 5 { known[9] = Some(word); },
            _ => { },
        }
    }

    // Known: 1, 4, 7, 8; 3, 6; 5, 9
    // Can solve for: 0, 2
    for word in seq.iter() {
        match word.len() {
            5 => if word != known[3].unwrap() && word != known[5].unwrap() { known[2] = Some(word); },
            6 => if word != known[6].unwrap() && word != known[9].unwrap() { known[0] = Some(word); },
            _ => { },
        }
    }

    return known.drain(..).enumerate().map(|(n, opt)| (opt.unwrap(), n)).collect();
}


struct Signal {
    seq: Vec<Word>,
    out: Vec<Word>,
}

impl Signal {
    fn code(&self) -> usize {
        let key = solve(self.seq.as_slice());

        let mut rv = 0;
        for word in self.out.iter() {
            rv *= 10;
            rv += key.iter().find(|(w, _)| word == *w).unwrap().1;
        }
        return rv;
    }
}


fn records(fname: &str) -> Vec<Signal> {
    let contents = std::fs::read_to_string(fname).unwrap();

    return contents.lines().map(
        |chunk| {
            let mut lr = chunk.split(" | ");
            let seq: Vec<Word> = lr.next().unwrap().split_whitespace().map(|s| s.chars().into_iter().collect::<Word>()).collect();
            let out: Vec<Word> = lr.next().unwrap().split_whitespace().map(|s| s.chars().into_iter().collect::<Word>()).collect();
            Signal { seq, out }
        }
    ).collect();
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("08.in"));
    let signals = records(&fname);

    let part1: usize = signals.iter().map(|s| s.out.iter().filter(|s| matches!(s.len(), 2|3|4|7)).count()).sum();
    println!("Part 1: {}", part1);

    let part2: usize = signals.iter().map(|s| s.code()).sum();
    println!("Part 2: {}", part2);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let signals = records("08.test");
        assert_eq!(signals[0].code(), 8394);
        assert_eq!(signals[1].code(), 9781);
        assert_eq!(signals[2].code(), 1197);
        assert_eq!(signals[3].code(), 9361);
        assert_eq!(signals[4].code(), 4873);
        assert_eq!(signals[5].code(), 8418);
        assert_eq!(signals[6].code(), 4548);
        assert_eq!(signals[7].code(), 1625);
        assert_eq!(signals[8].code(), 8717);
        assert_eq!(signals[9].code(), 4315);
    }
}
