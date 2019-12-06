
extern crate clap;
extern crate intcode;

// Time Start: Thu, 05 Dec 2019 19:25:20 -0500
// Time Finish 1: Thu, 05 Dec 2019 20:59:57 -0500 (1 hour, 34 minutes, 37 seconds)
// Time Finish 2: Thu, 05 Dec 2019 21:11:50 -0500 (11 minutes, 53 seconds)
// Time Total: 1 hour, 46 minutes, 30 seconds

use clap::{Arg, App};

use intcode::Intcode;

fn main() {
    let matches = App::new("Advent of Code 2019, Day 05")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("05.in"));

    let mut ic = Intcode::load(&fname);
    ic.pipe(1);
    ic.run();
    println!("Step 1 diagnostic: {:?}", ic.cat());

    let mut ic = Intcode::load(&fname);
    ic.pipe(5);
    ic.run();
    println!("Step 2 diagnostic: {:?}", ic.cat());

}
