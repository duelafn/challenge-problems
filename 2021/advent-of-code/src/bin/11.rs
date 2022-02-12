// SPDX-License-Identifier: MIT

use aoc::map::{Dir8, print_map};

type Map = Vec<Vec<u8>>;

fn step(map: &mut Map) -> usize {
    let mut flashes = 0;
    for row in map.iter_mut() {
        for ele in row.iter_mut() {
            *ele += 1;
        }
    }
    let mut flashed = true;
    while flashed {
        flashed = false;
        for i in 0..10 {
            for j in 0..10 {
                let flash = {
                    let ele = unsafe { map.get_unchecked_mut(j).get_unchecked_mut(i) };
                    if *ele > 9 { *ele = 0; flashes += 1; true } else { false }
                };
                if flash {
                    flashed = true;
                    for dir in Dir8::iter() {
                        if let Some((a, b)) = dir.step_usize(i, j) {
                            if let Some(v) = map.get_mut(b).and_then(|r| r.get_mut(a)) {
                                if *v > 0 { *v += 1; }
                            }
                        }
                    }
                }
            }
        }
    }
    return flashes;
}


fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("11.in"));
    let contents = std::fs::read_to_string(fname).unwrap();
    let mut map: Map = contents.lines().map(|l| { l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect() }).collect();

    let mut flashes = 0;
    for _ in 0..100 { flashes += step(&mut map); }
    println!("Flashes: {}", flashes);
    print_map(&map);

    let mut iter = 100;
    loop {
        iter += 1;
        if 100 == step(&mut map) {
            println!("Synchronized at: {}", iter);
            break;
        }
    }
}
