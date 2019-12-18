
extern crate clap;
extern crate intcode;

// Time Start: Tue, 17 Dec 2019 20:14:05 -0500
// Time Finish 1: Tue, 17 Dec 2019 22:34:14 -0500 (2 hours, 20 minutes, 9 seconds)
// Time Finish 2:
// Time Total:

use clap::{Arg, App};

use intcode::Intcode;
use intcode::util::{Chart,Direction,Robot};


fn walkable(_x: i64, _y: i64, ch: char) -> bool {
    match ch {
        '#' | '^' | '<' | '>' | 'v' => true,
        _ => false,
    }
}

fn alignment_parameter(chart: &Chart) -> i64 {
    let mut sum = 0;
    let top  = chart.bbox.ymax();
    let left = chart.bbox.xmin();
    for ((x, y), ch) in chart.map.iter() {
        if !walkable(*x, *y, *ch) { continue; }
        let mut wanted = true;
        for dir in Direction::each() {
            if wanted {
                let (a, b) = dir.step(*x, *y);
                wanted = walkable(a, b, chart.item_at(a, b));
            }
        }
        if wanted {
            sum += (x - left) * (top - y);
        }
    }
    return sum;
}

fn main() {
    let matches = App::new("Advent of Code 2019, Day 17")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("17.in"));

    let mut ic = Intcode::load(&fname);
    let mut chart = Chart::new();
    let mut bot = Robot::new();

    ic.run();
    let (mut x, mut y) = (0, 0);
    loop {
        if let Some(num) = ic.shift_output() {
            match num {
                n if n < 0 => panic!("Unexpected value: {}", n),
                10 => { y -= 1; x = -1; },
                35 => { chart.put(x, y, std::char::from_u32(num as u32).unwrap_or_else(|| panic!("Inconceivable!"))); },
                94 => { chart.put(x, y, '#'); bot.set_pos(x, y); bot.set_direction(Direction::North); },
                46 => (),  // Don't need to record empty space
                n => panic!("Unexpected character: {}", n),
            }
            x += 1;
        } else { break; }
    }

    println!("{}", chart);
    println!("{}", alignment_parameter(&chart));
}
