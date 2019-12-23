
extern crate clap;
extern crate intcode;

// Time Start: Mon, 23 Dec 2019 08:38:23 -0500
// Time Finish 1: Mon, 23 Dec 2019 09:39:50 -0500 (1 hour, 1 minute, 27 seconds)
// Time Finish 2:
// Time Total:

use clap::{Arg, App};

use intcode::Intcode;

fn main() {
    let matches = App::new("Advent of Code 2019, Day 21")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("21.in"));

    let mut ic = Intcode::load(&fname);

    // J = D & !(C&B&A)
    let jumpcode = "
NOT T J
AND A J
AND B J
AND C J
NOT J J
AND D J
WALK
".trim_start();

    ic.ascii_in(&String::from(jumpcode));
    ic.run();

    println!("{:?}", ic.cat());
}
