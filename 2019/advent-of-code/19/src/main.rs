
extern crate clap;
extern crate intcode;

// Time Start: Sat, 21 Dec 2019 11:34:37 -0500
// Time Finish 1: Sat, 21 Dec 2019 11:40:19 -0500 (5 minutes, 42 seconds)
// Time Finish 2: Sun, 22 Dec 2019 09:21:57 -0500 (21 hours, 41 minutes, 38 seconds)
// Time Total: 21 hours, 47 minutes, 20 seconds

use clap::{Arg, App};

use intcode::Intcode;
use intcode::util::Chart;


fn get(ic: &Intcode, chart: &mut Chart, x: i64, y: i64) -> char {
    match chart.get(x, y) {
        Some(x) => *x,
        None => {
            let mut ic = ic.clone();
            ic.pipe(x);
            ic.pipe(y);
            while ic.step() && !ic.has_output() { }
            match ic.shift_output() {
                Some(1) => chart.put(x, y, '#'),
                Some(0) => chart.put(x, y, ' '),
                Some(n) => panic!("Unexpected output '{}' at ({}, {})", n, x, y),
                None => panic!("Expected output at ({}, {})", x, y)
            }
            chart.item_at(x, y)
        }
    }
}

fn box_y(ic: &Intcode, chart: &mut Chart, x: i64, size: i64, slope: f32) -> Option<i64> {
    // Find any #
    let mut a = (x as f32 * slope) as i64;
    let mut step = 10;
    while get(ic, chart, x, a) != '#' {
        if a - step < 0 { panic!("Can't find '#' at x = {}", x); }
        if get(ic, chart, x, a + step) == '#' { a = a + step; break; }
        if get(ic, chart, x, a - step) == '#' { a = a - step; break; }
        step += 10;
    }

    // Find any ' '
    let mut b = a + 100;
    while get(ic, chart, x, b) != ' ' { b += 10; }

    // Binary search top '#'
    while a < b - 1 {
        let c = (a + b) / 2;
        match get(ic, chart, x, c) {
            '#' => { a = c; },
            ' ' => { b = c; },
             x  => unreachable!("Unexpected char '{}'", x),
        }
    }

    let top = a;
    let bot = a - size + 1;
    let right = x + size - 1;

    // Fail early if not big enough
    if bot < 0 { return None; }

    // Sloppy check is sufficient? ... seems to be
    if get(ic, chart, x, bot)     != '#' { return None; }
    if get(ic, chart, right, bot) != '#' { return None; }
    return Some(top);
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 19")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("19.in"));
    let ic = Intcode::load(&fname);

    let mut chart = Chart::new();
    let mut count = 0;
    for x in 0..50 {
        for y in 0..50 {
            if '#' == get(&ic, &mut chart, x, y) { count += 1; }
        }
    }

    println!("{}\nPart 1: Num affected: {}", chart, count);

    // At x=30, we first have diagonal of 4.
    // Target diagonal is 141, Start checking around 1000, +100 steps, then bisect
    let size = 100;
    let mut a = 50;
    let mut b = 1000;
    // Find any b
    loop {
        match box_y(&ic, &mut chart, b, size, 50.0/41.0) {
            Some(_) => { break; }
            None    => { a = b; b += 100; }
        }
    }

    // Binary search
    while a < b - 1 {
        let c = (a + b) / 2;
        match box_y(&ic, &mut chart, c, size, 50.0/41.0) {
            Some(_) => { b = c; }
            None    => { a = c; }
        }
    }
    let left = b;
    let top = box_y(&ic, &mut chart, left, size, 50.0/41.0).unwrap_or_else(|| unreachable!("But we just computed this!?"));
    let bot = top - size + 1;
    println!("Part 2: Can fit 100x100 at ({}, {}), (solution = {}", left, bot, 10000*left + bot);
}
