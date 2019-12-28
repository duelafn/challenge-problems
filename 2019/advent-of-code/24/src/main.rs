
extern crate clap;
extern crate intcode;

// Time Start: Thu, 26 Dec 2019 16:04:06 -0500
// Time Finish 1: Thu, 26 Dec 2019 16:55:31 -0500 (51 minutes, 25 seconds)
// Time Finish 2: Sat, 28 Dec 2019 12:30:54 -0500 (1 day, 19 hours, 35 minutes, 23 seconds)
// Time Total: 1 day, 20 hours, 26 minutes, 48 seconds

use std::collections::HashSet;
use std::collections::VecDeque;

use clap::{Arg, App};

use intcode::util::{Chart,Direction::*};

trait Life {
    fn step(&mut self);
    fn fingerprint(&self) -> u32;
}
impl Life for Chart {
    fn fingerprint(&self) -> u32 {
        let mut finger = 0;
        for y in (self.bbox.ymin()..=self.bbox.ymax()).rev() {
            for x in (self.bbox.xmin()..=self.bbox.xmax()).rev() {
                finger <<= 1;
                if '#' == self.item_at(x, y) {
                    finger |= 1;
                }
            }
        }
        return finger;
    }

    fn step(&mut self) {
        let orig = self.clone();
        for x in self.bbox.xmin()..=self.bbox.xmax() {
            for y in self.bbox.ymin()..=self.bbox.ymax() {
                let mut adjacent = 0;
                for dir in &[North, South, East, West] {
                    let (a, b) = dir.step(x, y);
                    if '#' == orig.item_at(a, b) { adjacent += 1; }
                }
                if adjacent == 1 {
                    self.put(x, y, '#');
                } else if adjacent == 2 {
                    if '#' == orig.item_at(x, y) {
                        self.put(x, y, '.');
                    } else {
                        self.put(x, y, '#');
                    }
                } else {
                    self.put(x, y, '.');
                }
            }
        }
    }
}

fn step_pluto(deque: &mut VecDeque<Chart>) -> i64 {
    let mut lifeforms = 0;
    let orig = deque.clone();
    let len = deque.len();

    // Assumpotion: first and last are always empty
    for i in 0..len {
        let map = deque.get_mut(i).unwrap_or_else(|| unreachable!("Invalid index!?"));
        for x in 0..5 {
            for y in 0..5 {
                if x != 2 || y != 2 {
                    let mut adjacent = 0;
                    for dir in &[North, South, East, West] {
                        let (a, b) = dir.step(x, y);
                        adjacent += match (a, b) {
                            // "Outside" ring
                            (-1, _) if i > 0 => orig[i-1].count_at(1, 2, '#'),
                            ( 5, _) if i > 0 => orig[i-1].count_at(3, 2, '#'),
                            (_, -1) if i > 0 => orig[i-1].count_at(2, 1, '#'),
                            (_,  5) if i > 0 => orig[i-1].count_at(2, 3, '#'),
                            // "Inside" ring
                            (2, 2) if x == 1 && i < len-1 => (0..5).map(|k| orig[i+1].count_at(0, k, '#')).sum(),
                            (2, 2) if x == 3 && i < len-1 => (0..5).map(|k| orig[i+1].count_at(4, k, '#')).sum(),
                            (2, 2) if y == 1 && i < len-1 => (0..5).map(|k| orig[i+1].count_at(k, 0, '#')).sum(),
                            (2, 2) if y == 3 && i < len-1 => (0..5).map(|k| orig[i+1].count_at(k, 4, '#')).sum(),
                            // Normal in-plane neighbor, ALSO i = 0 or len-1, but overflow slots will be empty
                            (_, _)  => orig[i].count_at(a, b, '#'),
                        };
                    }
                    if adjacent == 1 {
                        map.put(x, y, '#');
                    } else if adjacent == 2 {
                        if '#' == orig[i].item_at(x, y) {
                            map.put(x, y, '.');
                        } else {
                            map.put(x, y, '#');
                        }
                    } else {
                        map.put(x, y, '.');
                    }

                    if '#' == map.item_at(x, y) { lifeforms += 1; }
                }
            }
        }
    }

    // Add empty left/right if needed if (anything present on current left or right)
    if deque[len-1].count(|_,_,ch| ch == '#') > 0 { deque.push_back(Chart::new()); }
    if deque[0].count(|_,_,ch| ch == '#') > 0     { deque.push_front(Chart::new()); }

    return lifeforms;
}

fn show_lr(deque: &VecDeque<Chart>) {
    for y in 0..5 {
        for i in 0..deque.len() {
            for x in 0..5 {
                print!("{}", deque[i].item_at(x, y));
            }
            print!(" ");
        }
        print!("\n");
    }
    print!("\n");
}



fn main() {
    let matches = App::new("Advent of Code 2019, Day 24")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("24.in");

    let mut eris = Chart::load(&fname);
    eris.print_flipped = true;
    let mut seen = HashSet::new();
    while seen.insert(eris.fingerprint()) {
        if seen.len() < 4 {
            println!("{}:\n{}", seen.len()-1, eris);
        }
        eris.step();
    }

    println!("{}\nPart 1: Rating = {}\n", eris, eris.fingerprint());


    let mut test1 = VecDeque::from(vec![Chart::new(), Chart::load(&"test.1"), Chart::new()]);
    let mut count = 0;
    for _ in 0..10 { count = step_pluto(&mut test1); }
    println!("TEST: {} layers, count = {}", test1.len() - 2, count);
    show_lr(&test1);
    assert_eq!(count, 99);

    let mut eris = VecDeque::from(vec![Chart::new(), Chart::load(&fname), Chart::new()]);
    let mut count = 0;
    for _ in 0..200 { count = step_pluto(&mut eris); }
    println!("Part 2: {} bugs in {} layers", count, eris.len() - 2);
}
