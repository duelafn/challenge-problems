// SPDX-License-Identifier: MIT

fn fuel(a: i64, b: i64) -> i64 {
    let d = (b - a).abs();
    return d * (d+1) / 2;
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("07.in"));
    let contents = std::fs::read_to_string(fname).unwrap();
    let pos: Vec<i64> = contents.trim().split(',').map(|v| v.parse().unwrap()).collect();
    let max = *pos.iter().max().unwrap();

    let mut best = (-1, i64::MAX);
    for i in 0..=max {
        let cost = pos.iter().map(|v| (i-v).abs()).sum();
        if cost < best.1 { best = (i, cost); }
    }

    println!("Part 1: Position {} for cost {}", best.0, best.1);

    let mut best = (-1, i64::MAX);
    for i in 0..=max {
        let cost = pos.iter().map(|v| fuel(i, *v)).sum();
        if cost < best.1 { best = (i, cost); }
    }

    println!("Part 2: Position {} for cost {}", best.0, best.1);
}
