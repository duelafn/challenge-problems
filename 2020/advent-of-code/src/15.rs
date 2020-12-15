// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::time::Instant;


fn main() {
    let start = [ 8,11,0,19,1,2 ];

    let t0 = Instant::now();
    let mut last = None;
    let mut num = 0;
    let mut seen = HashMap::new();
    for (i, n) in start.iter().enumerate() {
        num = *n;
        last = seen.insert(num, i);
    }

    for turn in start.len()..2020 {
        match last {
            None    => { num = 0; },
            Some(n) => { num = turn - n - 1; },
        }
        last = seen.insert(num, turn);
    }
    println!("Part 1: The 2020th number is {}", num);

    for turn in 2020..30000000 {
        match last {
            None    => { num = 0; },
            Some(n) => { num = turn - n - 1; },
        }
        last = seen.insert(num, turn);
    }
    println!("Part 2: The 30000000th number is {}, hash RAM: {}M, hash time: {}", num, (seen.capacity() * 11 / 10) * (8 + 8 + 8) / 1024/1023, (Instant::now() - t0).as_millis());
    // HashMap: (cap() * 11 / 10) * (size(K) + size(V) + size(u64))


    let start = [ 8,11,0,19,1,2 ];

    let t0 = Instant::now();
    let mut last = -1;
    let mut num = 0;
    let mut seen: Vec<i64> = vec![-1; 1000];
    for (i, n) in start.iter().enumerate() {
        num = *n;
        last = seen[num];
        seen[num] = i as i64;
    }

    for turn in (start.len() as i64)..2020 {
        if last < 0 { num = 0; }
        else        { num = (turn - last - 1) as usize; }
        if num >= seen.len() { seen.resize(num+1, -1); }
        last = seen[num];
        seen[num] = turn;
    }
    println!("Part 1: The 2020th number is {}", num);

    for turn in 2020..30000000 {
        if last < 0 { num = 0; }
        else        { num = (turn - last - 1) as usize; }
        if num >= seen.len() { seen.resize(num+1, -1); }
        last = seen[num];
        seen[num] = turn;
    }
    println!("Part 2: The 30000000th number is {}, vec RAM: {}M, vec time: {}", num, seen.len() * 8 / 1024/1024, (Instant::now() - t0).as_millis());
}
