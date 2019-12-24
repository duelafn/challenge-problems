
extern crate clap;

// Time Start: Tue, 24 Dec 2019 05:49:55 -0500
// Time Finish 1: Tue, 24 Dec 2019 07:29:32 -0500 (1 hour, 39 minutes, 37 seconds)
// Time Finish 2:
// Time Total:

use std::fs;

use clap::{Arg, App};

type Deck = Vec<u16>;


#[derive(Copy, Clone, Debug)]
enum Technique {
    DealNew,
    Cut(i64),
    DealInc(i64),
}
use Technique::*;

impl Technique {
    pub fn apply(&self, deck: &mut Deck) {
        match self {
            DealNew => deck.reverse(),
            Cut(n) if *n >= 0 => deck.rotate_left(*n as usize),
            Cut(n) if *n < 0  => deck.rotate_right((-*n) as usize),
            DealInc(n) => {
                let tmp = deck.clone();
                let len = tmp.len();
                let mut i = 0;
                for val in tmp {
                    deck[i] = val;
                    i += *n as usize;
                    i %= len;
                }
            },
            _ => panic!("Stupid rust"),
        }
    }
}


fn deck(n: u16) -> Deck { (0..n).collect() }

fn num_at(s: &str, n: usize) -> i64 {
    let mut ok = false;
    let mut sign = 1_i64;
    let mut rv = 0_i64;
    for ch in s.chars().skip(n) {
        match ch {
            '+' => (),
            '-' => sign *= -1,
            '0'..='9' => { ok = true; rv *= 10; rv += ch.to_digit(10).unwrap_or_else(|| unreachable!("Bummer")) as i64; },
             _  => break,
        }
    }
    if !ok { panic!("No number found at position {} in '{}'", n, s); }
    return sign * rv;
}

fn load_shuffle(fname: &String) -> Vec<Technique> {
    let contents = fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let mut i = 0;
    return contents.lines().map(|l| {
        i += 1;
        if l == "deal into new stack" {
            DealNew
        } else if l.contains("increment") {
            DealInc(num_at(l, 20))
        } else if l.contains("cut") {
            Cut(num_at(l, 4))
        } else {
            panic!("Unexpected content '{}' on line {}", l, i);
        }
    }).collect();
}

fn main() {
    let matches = App::new("Advent of Code 2019, Day 22")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("22.in"));

    let shuffle = load_shuffle(&fname);
    let mut cards = deck(10007);
    for tq in shuffle {
        tq.apply(&mut cards);
    }

    if let Some(idx) = cards.iter().position(|&x| x == 2019) {
        println!("Step 1: 2019 is at index {}", idx);
    } else {
        println!("{:?}", cards);
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_new() {
        let mut cards = deck(10);
        DealNew.apply(&mut cards);
        assert_eq!(cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn deal_inc() {
        let mut cards = deck(10);
        DealInc(3).apply(&mut cards);
        assert_eq!(cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn cut() {
        let mut cards = deck(10);
        Cut(3).apply(&mut cards);
        assert_eq!(cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        let mut cards = deck(10);
        Cut(-4).apply(&mut cards);
        assert_eq!(cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }
}
