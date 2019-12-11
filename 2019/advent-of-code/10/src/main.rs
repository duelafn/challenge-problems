
extern crate clap;
extern crate num_integer;

// Time Start: Tue, 10 Dec 2019 16:01:02 -0500
// Time Finish 1: Tue, 10 Dec 2019 17:26:44 -0500 (1 hour, 25 minutes, 42 seconds)
// Time Finish 2: Tue, 10 Dec 2019 19:03:18 -0500 (1 hour, 36 minutes, 34 seconds)
// Time Total: 3 hours, 2 minutes, 16 seconds

use std::fs;

use clap::{Arg, App};
use num_integer::Integer;


struct Point(i32, i32);

struct Map {
    map: Vec<String>,
}
impl Map {
    pub fn new(fname: &String) -> Map {
        let contents = fs::read_to_string(fname)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

        let map = contents.trim().split_ascii_whitespace().map(|s| String::from(s)).collect::<Vec<String>>();
        return Map { map: map };
    }

    pub fn width(&self)  -> i32 { self.map.get(0).map_or(0, |row| row.len() as i32) }
    pub fn height(&self) -> i32 { self.map.len() as i32 }

    pub fn get(&self, i: i32, j: i32) -> char {
        if i < 0 || j < 0 { return '?' }
        self.map.get(j as usize).map_or('?', |row| row.chars().nth(i as usize).unwrap_or('?'))
    }

    pub fn visible_asteroids(&self, i: i32, j: i32) -> i32 {
        let w = self.width() as i32;
        let h = self.height() as i32;
        let mut asteroids = 0;

        for x in 0..w {
            for y in 0..h {
                let dx = x - i;
                let dy = y - j;

                if dx.gcd(&dy) == 1 { // (0,0) excluded since gcd is 0  (see tests)
                    let mut mult = 1;
                    let mut ch = self.get(i+dx, j+dy);
                    while ch == '.' {
                        mult += 1;
                        ch = self.get(i+mult*dx, j+mult*dy);
                    }

                    match ch {
                        '#' => { asteroids += 1 },
                        '?' => (), // edge of map
                         _  => unreachable!("Should have hit something by now!"),
                    }
                }
            }
        }
        return asteroids;
    }

    // Need a clockwise comparable "angle" starting with (0,-1) being at "0"
    pub fn direction(&self, dx: &i32, dy: &i32) -> i32 {
        // y.atan2(x) : but we want (0,-1) to be at 0, so swap x and y
        let deg = 180.0 - (*dx as f64).atan2(*dy as f64).to_degrees();
        let dir = (1e6 * deg).round() as i32; // scale so we can compare integers
        return if dir == 360000000 { 0 } else { dir }; // Above generates ( 0 .. 360000000 ], change side of "]"
    }

    pub fn laser_targets(&self, i: i32, j: i32) -> Vec<Point> {
        //                   mult, dir, point
        let mut targets: Vec<(i32, i32, Point)> = Vec::new();

        let w = self.width() as i32;
        let h = self.height() as i32;
        for x in 0..w {
            for y in 0..h {
                let dx = x - i;
                let dy = y - j;

                if dx.gcd(&dy) == 1 { // (0,0) excluded since gcd is 0  (see tests)
                    let dir = self.direction(&dx, &dy);
                    let mut mult = 1;
                    let mut rotation = 1;
                    let mut ch = self.get(i+dx, j+dy);
                    while ch != '?' {
                        if ch == '#' {
                            targets.push( (rotation, dir, Point(i+mult*dx, j+mult*dy)) );
                            rotation += 1;
                        }
                        mult += 1;
                        ch = self.get(i+mult*dx, j+mult*dy);
                    }
                }
            }
        }

        targets.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        return targets.iter().map(|t| Point((t.2).0, (t.2).1)).collect::<Vec<Point>>();
    }
}



fn main() {
    let matches = App::new("Advent of Code 2019, Day 10")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("10.in"));

    let map = Map::new(&fname);

    let w = map.width();
    let h = map.height();
    let mut best = (0, 0, 0);

    for x in 0..w {
        for y in 1..h {
            if '#' == map.get(x, y) { // Can only build outpost on an asteroid!
                let seen = map.visible_asteroids(x, y);
                if seen > best.2 {
                    best = (x, y, seen);
                }
            }
        }
    }

    println!("Can see {} asteroids from ({}, {})", best.2, best.0, best.1);

    let targets = map.laser_targets(best.0, best.1);
    if let Some(target) = targets.get(0) {
        println!("The 1st asteroid to be vaporized is at ({}, {})", target.0, target.1);
    }
    if let Some(target) = targets.get(9) {
        println!("The 10th asteroid to be vaporized is at ({}, {})", target.0, target.1);
    }
    if let Some(target) = targets.get(99) {
        println!("The 100th asteroid to be vaporized is at ({}, {})", target.0, target.1);
    }
    if let Some(target) = targets.get(199) {
        println!("The 200th asteroid to be vaporized is at ({}, {})", target.0, target.1);
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use num_integer::Integer;

    #[test]
    fn sanity() {
        assert_eq!(0_i32.gcd(&1), 1);
        assert_eq!(1_i32.gcd(&0), 1);
        assert_eq!(0_i32.gcd(&0), 0);
        assert_eq!(5_i32.gcd(&0), 5);
        assert_eq!(0_i32.gcd(&2), 2);

        assert_eq!(1.0_f64.atan2(0.0).to_degrees().round() as i32, 90);
        assert_eq!(0.0_f64.atan2(1.0).to_degrees().round() as i32, 0);
        assert_eq!(-1.0_f64.atan2(0.0).to_degrees().round() as i32, -90);
        assert_eq!(0.0_f64.atan2(-1.0).to_degrees().round() as i32, 180);

        let map = Map::new(&String::from("test.1"));
        assert_eq!(map.direction(&0, &-1), 0);
        assert_eq!(map.direction(&1, &0), 90_000_000);
        assert_eq!(map.direction(&0, &1), 180_000_000);
        assert_eq!(map.direction(&-1, &0), 270_000_000);
    }
}
