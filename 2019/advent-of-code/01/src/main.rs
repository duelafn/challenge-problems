
extern crate clap;

// Time Start: Sun, 01 Dec 2019 15:40:57 -0500
// Time Finish: Sun, 01 Dec 2019 16:53:33 -0500
// Time Total: 1 hour, 12 minutes, 36 seconds

use std::boxed::Box;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use clap::{Arg, App};

fn records(fname: &str) -> Box<Iterator<Item=i32>> {
    let file = File::open(fname).unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
    let reader = BufReader::new(file);
    let name = String::from(fname);

    return Box::new(reader.lines().map(
        move |x| x
            .unwrap_or_else(|err| panic!("Error reading {}: {}", name, err))
            .parse()
            .unwrap_or_else(|err| panic!("Parse error: {}", err))
    ));
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 01")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("01.in");

    let mut naive_fuel = 0;
    let mut real_fuel = 0;
    for weight in records(fname) {
        let mut this_fuel = weight / 3 - 2;
        naive_fuel += this_fuel;
        while this_fuel > 0 {
            real_fuel += this_fuel;
            this_fuel = this_fuel / 3 - 2;
        }
    }
    println!("Fuel required (naïve): {}", naive_fuel);
    println!("Fuel required (actual): {}", real_fuel);
}
