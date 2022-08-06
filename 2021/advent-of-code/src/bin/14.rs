// SPDX-License-Identifier: MIT

use std::collections::HashMap;

type Map = HashMap<(u8, u8), u8>;
type Count = HashMap<(u8, u8), i64>;

fn load(fname: &str) -> (Vec<u8>, Map) {
    let contents = std::fs::read_to_string(fname).unwrap();
    let mut lines = contents.lines();

    let word = String::from(lines.next().unwrap()).into_bytes();
    let mut map = HashMap::new();

    lines.next().unwrap(); // blank line
    for line in lines {
        let b = line.as_bytes();
        map.insert((b[0], b[1]), b[6]);
    }
    return (word, map);
}

fn step(vec: &Count, map: &Map) -> Count {
    let mut rv: Count = HashMap::new();
    for (k, v) in vec {
        if let Some(&m) = map.get(k) {
            for new in &[(k.0, m), (m, k.1)] {
                *(rv.entry(*new).or_insert(0)) += v;
            }
        }
        else { *(rv.entry(*k).or_insert(0)) += v; }
    }
    return rv;
}

fn score(orig: &[u8], vec: &Count) -> i64 {
    let mut count: HashMap<u8, i64> = HashMap::new();
    // Below, we count first letter in each pair, this will miss counting
    // last letter in the original word (it never appears first in a pair).
    // Count it here:
    count.insert(*orig.last().unwrap(), 1);
    for (k, v) in vec { *(count.entry(k.0).or_insert(0)) += v; }

    let mut list: Vec<u8> = count.keys().cloned().collect();
    list.sort_by_key(|k| count[k]);
    return count[list.last().unwrap()] - count[&list[0]];
}

fn to_count(word: &[u8]) -> Count {
    let mut count = HashMap::new();
    let mut iter = word.windows(2);
    while let Some(&[a, b]) = iter.next() {
        *(count.entry((a,b)).or_insert(0)) += 1;
    }
    return count;
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("14.in"));

    let (word, map) = load(&fname);
    let mut count = to_count(&word);
    for _ in 0..10 { count = step(&count, &map); }
    eprintln!("Part 1: {}", score(&word, &count)); // Part 1: 2602

    for _ in 0..30 { count = step(&count, &map); }
    eprintln!("Part 2: {}", score(&word, &count)); // Part 2: 2942885922173
}
