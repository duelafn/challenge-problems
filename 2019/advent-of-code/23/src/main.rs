
extern crate clap;
extern crate intcode;

// Time Start: Thu, 26 Dec 2019 15:07:33 -0500
// Time Finish 1: Thu, 26 Dec 2019 15:47:02 -0500 (39 minutes, 29 seconds)
// Time Finish 2:
// Time Total:

use std::collections::HashSet;

use clap::{Arg, App};

use intcode::Intcode;


struct Network {
    computers: Vec<Intcode>,
    nat: (i64, i64),
}
impl Network {
    pub fn new(fname: &String, num: i64) -> Network {
        let mut comps = Vec::new();
        for i in 0..num {
            let mut ic = Intcode::load(&fname);
            ic.pipe(i);
            ic.nbinput(Some(-1));
            comps.push(ic);
        }
        return Network { computers: comps, nat: (0, 0) };
    }

    pub fn step(&mut self) -> Option<i64> {
        let num_comp = self.computers.len();
        let mut blocked = 0;
        for i in 0..self.computers.len() {
            let comp = &mut self.computers[i];
            comp.step();
            if comp.nbread_count() > 1 && comp.input_len() == 0 { blocked += 1; }
            if comp.output_len() >= 3 {
                let n = comp.shift_output().unwrap_or_else(|| panic!("expected 1 output!")) as usize;
                let x = comp.shift_output().unwrap_or_else(|| panic!("expected 1 output!"));
                let y = comp.shift_output().unwrap_or_else(|| panic!("expected 1 output!"));
                if n == 255 {
                    self.nat = (x, y);
                } else if n >= num_comp {
                    panic!("Bad address, sent {}, {} to address {}", x, y, n);
                } else {
                    self.computers[n].pipe(x);
                    self.computers[n].pipe(y);
                }
            }
        }

        if num_comp == blocked {
            self.computers[0].pipe(self.nat.0);
            self.computers[0].pipe(self.nat.1);
            return Some(self.nat.1)
        }
        return None;
    }
}



fn main() {
    let matches = App::new("Advent of Code 2019, Day 23")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("23.in"));

    let mut net = Network::new(&fname, 50);
    let mut seen = HashSet::new();
    loop {
        if let Some(y) = net.step() {
            if seen.len() == 0 {
                println!("First Y-value sent to NAT: {}", y);
            }
            if !seen.insert(y) {
                println!("First duplicated Y-value sent to NAT: {}", y);
                break;
            }
        }
    };
}
