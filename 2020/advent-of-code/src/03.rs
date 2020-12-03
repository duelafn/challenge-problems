// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::convert::TryFrom;

use clap::{Arg, App};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Obstacle {
    Tree,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    width: u64,
    height: u64,
    map: HashMap<(u64,u64), Obstacle>
}
impl Map {
    pub fn at(&self, i: u64, j: u64) -> Obstacle {
        let i = i.rem_euclid(self.width);
        match self.map.get(&(i, j)) {
            Some(x) => *x,
            None    => Obstacle::None,
        }
    }

    pub fn trees_on_slope(&self, j: u64, i: u64) -> u64 {
        let (mut x, mut y) = (0, 0);
        let mut rv = if self.at(x,y) == Obstacle::Tree { 1 } else { 0 };
        while y < self.height {
            x += i; y += j;
            if self.at(x,y) == Obstacle::Tree {
                rv += 1
            };
        }
        return rv;
    }
}
impl std::convert::TryFrom<&str> for Map {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut map = HashMap::new();
        let (mut i, mut j) = (0, 0);
        let mut width = 0;
        for ch in src.chars() {
            if ch == '.' { i += 1; } // pass
            else if ch == '#' { map.insert((i,j), Obstacle::Tree); i += 1; }
            else if ch == '\n' { width = i; i = 0; j += 1; }
            else { return Err(format!("Unexpected character '{}'", ch)); }
        }
        return Ok(Map { map, width, height: j });
    }
}


fn main() {
    let matches = App::new("Advent of code 2020, Day 03 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("03.in");
    let contents = std::fs::read_to_string(fname).unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let map = Map::try_from(contents.as_ref()).unwrap();

    println!("Part 1: {} trees on slope 1/3", map.trees_on_slope(1, 3));

    println!("Part 2: product {}",
             map.trees_on_slope(1, 1)
             * map.trees_on_slope(1, 3)
             * map.trees_on_slope(1, 5)
             * map.trees_on_slope(1, 7)
             * map.trees_on_slope(2, 1)
    );
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let contents = std::fs::read_to_string("03.test-1.in").unwrap_or_else(|err| panic!("Error reading 03.test-1.in: {}", err));
        let map = Map::try_from(contents.as_ref()).unwrap();
        assert_eq!(map.trees_on_slope(1, 3), 7);
    }
}

