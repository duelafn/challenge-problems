// SPDX-License-Identifier: MIT

use std::collections::VecDeque;

struct Population {
    pop: VecDeque<u64>,
}
impl Population {
    fn load(src: &str) -> Self {
        let mut pop = VecDeque::from(vec![0; 9]);
        for age in src.trim().split(',').map(|v| v.parse().unwrap()) {
            pop[age] += 1;
        }
        Population { pop }
    }

    fn step(&mut self) {
        let n = self.pop.pop_front().unwrap();
        self.pop.push_back(n);
        self.pop[6] += n;
    }

    fn size(&self) -> u64 { self.pop.iter().sum() }
}
impl std::fmt::Display for Population {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, n) in self.pop.iter().enumerate() {
            if i > 0 { write!(f, "  ")?; }
            write!(f, "{}:{}", i, n)?;
        }
        Ok(())
    }
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("06.in"));
    let contents = std::fs::read_to_string(fname).unwrap();

    let mut pop = Population::load(&contents);
    for _ in 0..80 { pop.step(); }
    println!("Part 1: {} fish", pop.size());

    for _ in 80..256 { pop.step(); }
    println!("Part 2: {} fish", pop.size());
}
