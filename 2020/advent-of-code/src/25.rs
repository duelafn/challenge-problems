// SPDX-License-Identifier: MIT

fn transform(subject: u64, loops: u64) -> u64 {
    let mut val = 1;
    for _ in 0..loops {
        val = (val * subject) % 20201227; // 20201227 is prime
    }
    return val;
}

fn solve_loops(pubkey: u64, subject: u64) -> u64 {
    let mut val = 1;
    for i in 1.. {
        val = (val * subject) % 20201227;
        if val == pubkey { return i as u64; }
    }
    unreachable!();
}


fn main() {
    // Example:
//     let card = 576_48_01;  //  5764801: 7 7 7 7 7 7 7 7
//     let door = 1780_77_24; // 17807724: 2 2 3 3 11 193 233
    // Mine:
    let card = 1422_25_96; // 14222596: 2 2 3555649
    let door = 405_74_28;  //  4057428: 2 2 3 338119

    let n = solve_loops(card, 7);
    println!("Part 1: encryption key is {}", transform(door, n));
}
