// SPDX-License-Identifier: MIT

use clap::{Arg, App};

static DIRECTIONS : [(i64,i64); 8] = [
    (-1,-1), ( 0,-1), ( 1,-1),
    (-1, 0),          ( 1, 0),
    (-1, 1), ( 0, 1), ( 1, 1),
];

#[derive(Clone)]
pub struct Floorplan {
    pub map: Vec<Vec<char>>,
}
impl Floorplan {
    pub fn new() -> Floorplan {
        Floorplan { map: Vec::new() }
    }

    pub fn load(fname: &str) -> Floorplan {
        let mut plan = Floorplan::new();
        let contents = std::fs::read_to_string(fname)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

        for line in contents.trim_end_matches('\n').split('\n') {
            plan.map.push(line.chars().collect());
        }
        return plan;
    }

    pub fn step1(&mut self) -> bool {
        let mut add = Vec::new();
        let mut rm  = Vec::new();

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                match (self.map[y][x], self.count_neighbors(x, y)) {
                    ('L', 0)           => { add.push((x,y)); },
                    ('#', n) if n >= 4 => {  rm.push((x,y)); },
                    _ => { },
                }
            }
        }

        if add.is_empty() && rm.is_empty() { return false; }

        for (i, j) in add { self.map[j][i] = '#'; }
        for (i, j) in rm  { self.map[j][i] = 'L'; }
        return true;
    }

    pub fn step2(&mut self) -> bool {
        let mut add = Vec::new();
        let mut rm  = Vec::new();

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                match (self.map[y][x], self.count_visible(x, y)) {
                    ('L', 0)           => { add.push((x,y)); },
                    ('#', n) if n >= 5 => {  rm.push((x,y)); },
                    _ => { },
                }
            }
        }

        if add.is_empty() && rm.is_empty() { return false; }

        for (i, j) in add { self.map[j][i] = '#'; }
        for (i, j) in rm  { self.map[j][i] = 'L'; }
        return true;
    }

    #[inline]
    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut seated = 0;
        for (dx, dy) in DIRECTIONS.iter() {
            let (x, y) = ((x as i64+dx) as usize, (y as i64+dy) as usize);
            if y < self.map.len() {
                let row = &self.map[y];
                if x < row.len() && '#' == row[x] { seated += 1; }
            }
        }
        return seated;
    }

    #[inline]
    pub fn count_visible(&self, x: usize, y: usize) -> u8 {
        let mut seated = 0;
        let ymax = self.map.len() as i64;
        let xmax = self.map[0].len() as i64;
        for (dx, dy) in DIRECTIONS.iter() {
            let (mut x, mut y) = (x as i64+dx, y as i64+dy);
            while x >= 0 && y >= 0 && x < xmax && y < ymax {
                match self.map[y as usize][x as usize] {
                    '#' => { seated += 1; break; },
                    'L' => { break; },
                    _   => { },
                }
                x += dx; y += dy;
            }
        }
        return seated;
    }

    #[inline]
    pub fn count(&self, ch: &char) -> u64 {
        let mut n = 0;
        for row in self.map.iter() {
            for c in row.iter() {
                if c == ch { n += 1; }
            }
        }
        return n;
    }
}
impl std::fmt::Display for Floorplan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}



fn main() {
    let matches = App::new("Advent of code 2020, Day 11 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("11.in");
    let mut plan = Floorplan::load(fname);
//     println!("{}", plan);
    while plan.step1() { }
    println!("Part 1: {} occupied seats", plan.count(&'#'));
//     println!("{}", plan);

    let mut plan = Floorplan::load(fname);
//     println!("{}", plan);
    while plan.step2() { }
    println!("Part 2: {} occupied seats", plan.count(&'#'));
//     println!("{}", plan);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut plan = Floorplan::load("11.in");
        while plan.step1() { }
        assert_eq!(plan.count(&'#'), 2476);
    }

    #[test]
    fn test2() {
        let mut plan = Floorplan::load("11.in");
        while plan.step2() { }
        assert_eq!(plan.count(&'#'), 2257);
    }
}
