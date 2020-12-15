// SPDX-License-Identifier: MIT

use std::collections::HashMap;

fn main() {
    let start = [ 8,11,0,19,1,2 ];

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
    println!("Part 2: The 30000000th number is {}", num);
}
