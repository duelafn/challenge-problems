
extern crate clap;

// Time Start: Mon, 16 Dec 2019 07:42:46 -0500
// Time Finish 1: Mon, 16 Dec 2019 08:30:39 -0500 (47 minutes, 53 seconds)
// Time Finish 2:
// Time Total:

use std::fs;

use clap::{Arg, App};

const PAT1: [i32; 4] = [0,1,0,-1];

fn fft(vec: &Vec<i32>, pat: &[i32], n: u32) -> Vec<i32> {
    let mut new = vec.clone();
    let mut vec = Vec::with_capacity(vec.len());
    for _ in 0..n {
        std::mem::swap(&mut vec, &mut new);
        // let (mut vec, mut new) = (new, vec);
        new.clear();
        for place in 0..vec.len() {
            let mut val = 0;
            let mut pat_idx = 0;
            let mut repeat = place;
            for idx in 0..vec.len() {
                if repeat == 0 {
                    pat_idx = (pat_idx + 1) % pat.len();
                    repeat = place + 1;
                }
                val += vec[idx] * pat[pat_idx];
                val = val.signum() * (val.abs() % 100000000);
                repeat -= 1;
            }
            new.push(val.abs() % 10);
        }
    }
    return new;
}

fn repeat<T: Clone>(vec: &Vec<T>, n: usize) -> Vec<T> {
    let mut rv = Vec::with_capacity(n * vec.len());
    for _ in 0..n {
        rv.extend_from_slice(&vec[0..vec.len()]);
    }
    return rv;
}

fn num_at(vec: &Vec<i32>, idx: usize, len: usize) -> i32 {
    let mut rv = 0;
    for i in 0..len {
        rv = 10*rv + vec[idx + i];
    }
    return rv;
}

fn load(fname: &str) -> Vec<i32> {
    let contents = fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
    return contents.trim().chars().map(|c| c.to_digit(10).unwrap_or_else(|| panic!("Parse error")) as i32).collect();
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 16")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("16.in");
    let vec = load(&fname);
    println!("Part 1: {}", num_at(&fft(&vec, &PAT1, 100), 0, 8));

    let vec2 = repeat(&vec, 10000);
    let part2 = fft(&vec2, &PAT1, 100);
    println!("Part 2: {}", num_at(&part2, num_at(&part2, 0, 7) as usize, 8));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(fft(&vec![1,2,3,4,5,6,7,8], &PAT1, 1), vec![4,8,2,2,6,1,5,8]);
        assert_eq!(fft(&vec![4,8,2,2,6,1,5,8], &PAT1, 1), vec![3,4,0,4,0,4,3,8]);
        assert_eq!(fft(&vec![3,4,0,4,0,4,3,8], &PAT1, 1), vec![0,3,4,1,5,5,1,8]);
        assert_eq!(fft(&vec![0,3,4,1,5,5,1,8], &PAT1, 1), vec![0,1,0,2,9,4,9,8]);
        assert_eq!(fft(&vec![8,0,8,7,1,2,2,4,5,8,5,9,1,4,5,4,6,6,1,9,0,8,3,2,1,8,6,4,5,5,9,5], &PAT1, 100)[0..8], [2,4,1,7,6,1,7,6]);
        assert_eq!(fft(&vec![1,9,6,1,7,8,0,4,2,0,7,2,0,2,2,0,9,1,4,4,9,1,6,0,4,4,1,8,9,9,1,7], &PAT1, 100)[0..8], [7,3,7,4,5,4,1,8]);
        assert_eq!(fft(&vec![6,9,3,1,7,1,6,3,4,9,2,9,4,8,6,0,6,3,3,5,9,9,5,9,2,4,3,1,9,8,7,3], &PAT1, 100)[0..8], [5,2,4,3,2,1,3,3]);
    }
}
