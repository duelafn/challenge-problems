
extern crate clap;
extern crate intcode;

// Time Start: Sat, 21 Dec 2019 11:34:37 -0500
// Time Finish 1: Sat, 21 Dec 2019 11:40:19 -0500 (5 minutes, 42 seconds)
// Time Finish 2:
// Time Total:

use clap::{Arg, App};

use intcode::Intcode;
use intcode::util::Chart;

fn main() {
    let matches = App::new("Advent of Code 2019, Day 19")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("19.in"));

    let mut chart = Chart::new();
    for x in 0..50 {
        for y in 0..50 {
            let mut ic = Intcode::load(&fname);
            ic.pipe(x);
            ic.pipe(y);
            while ic.step() && !ic.has_output() { }
            match ic.shift_output() {
                Some(n) => if n == 1 { chart.put(x, y, '#'); },
                None => ()
            }
        }
    }

    println!("{}\nNum affected: {}", chart, chart.map.len());
}
