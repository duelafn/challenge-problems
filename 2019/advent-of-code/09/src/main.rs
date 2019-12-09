
extern crate clap;
extern crate intcode;

// Time Start: Mon, 09 Dec 2019 16:35:40 -0500
// Time Finish 1: Mon, 09 Dec 2019 17:51:14 -0500 (1 hour, 15 minutes, 34 seconds)
// Time Finish 2: Mon, 09 Dec 2019 17:52:06 -0500 (52 seconds)
// Time Total: 1 hour, 16 minutes, 26 seconds

use clap::{Arg, App};
use intcode::Intcode;


fn main() {
    let matches = App::new("Advent of Code 2019, Day 09")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("09.in"));

    let mut ic = Intcode::load(&fname);
    ic.pipe(1);
    ic.run();
    println!("Step 1 diagnostic: {:?}", ic.cat());

    let mut ic = Intcode::load(&fname);
    ic.pipe(2);
    ic.run();
    println!("Step 2 distress: {:?}", ic.cat());
}
