// SPDX-License-Identifier: MIT

fn records(fname: &str) -> Vec<i32> {
    let contents = std::fs::read_to_string(fname).unwrap();

    return contents.lines().enumerate().map(
        |(lineno, chunk)| {
            chunk.parse().unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("01.in"));

    let mut last = None;
    let mut grow = 0;
    for n in records(&fname) {
        if let Some(last) = last {
            if last < n { grow += 1; }
        }
        last = Some(n);
    }
    println!("increases: {}", grow);

    let mut last = None;
    let mut grow = 0;
    for n in records(&fname).windows(3) {
        let sum: i32 = n.iter().sum();
        if let Some(last) = last {
            if last < sum { grow += 1; }
        }
        last = Some(sum);
    }
    println!("window(3) increases: {}", grow);
}
