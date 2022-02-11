// SPDX-License-Identifier: MIT

use aoc::map::*;


fn load(fname: &str) -> Vec<Vec<i8>> {
    let contents = std::fs::read_to_string(fname).unwrap();
    return contents.lines().map(|l| l.chars().map(|v| v.to_digit(10).unwrap() as i8).collect()).collect()
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("09.in"));
    let map = load(&fname);

    let mut risk: i64 = 0;
    for (x, y, v) in MapIter2D::new(&map) {
        if MapNeighbor4Iter2D::new(&map, x, y).all(|(_, _, n)| n > v) {
            risk += (*v as i64) + 1;
        }
    }
    println!("Part 1: risk = {}", risk);

    let mut basin = vec![vec![0; map[0].len()]; map.len()];
    let mut sizes = Vec::new();
    let mut todo = Vec::new();
    for (x, y, &v) in MapIter2D::new(&map) {
        // Is this point in a new basin?
        if v != 9 && basin[y][x] == 0 {
            // Fill the basin
            let mut n = 1;
            let bnum = 1 + sizes.len();
            basin[y][x] = bnum;
            todo.push((x,y));
            while let Some((a, b)) = todo.pop() {
                for (i, j, &k) in MapNeighbor4Iter2D::new(&map, a, b) {
                    if k != 9 && basin[j][i] == 0 {
                        n += 1;
                        basin[j][i] = bnum;
                        todo.push((i, j));
                    }
                }
            }
            sizes.push(n);
        }
    }
    sizes.sort_unstable();
    sizes.reverse();
    println!("Part 2: {}", sizes[0] * sizes[1] * sizes[2]);
}
