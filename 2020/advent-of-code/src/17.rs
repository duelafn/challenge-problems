// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use clap::{Arg, App};

use advent_of_code::bbox::*;


static DIRECTIONS_3D : [(i32,i32,i32); 26] = [
    (-1,-1,-1), ( 0,-1,-1), ( 1,-1,-1),
    (-1, 0,-1), ( 0, 0,-1), ( 1, 0,-1),
    (-1, 1,-1), ( 0, 1,-1), ( 1, 1,-1),

    (-1,-1, 0), ( 0,-1, 0), ( 1,-1, 0),
    (-1, 0, 0),             ( 1, 0, 0),
    (-1, 1, 0), ( 0, 1, 0), ( 1, 1, 0),

    (-1,-1, 1), ( 0,-1, 1), ( 1,-1, 1),
    (-1, 0, 1), ( 0, 0, 1), ( 1, 0, 1),
    (-1, 1, 1), ( 0, 1, 1), ( 1, 1, 1),
];

static DIRECTIONS_4D : [(i32,i32,i32,i32); 80] = [
    (-1,-1,-1,-1), ( 0,-1,-1,-1), ( 1,-1,-1,-1),
    (-1, 0,-1,-1), ( 0, 0,-1,-1), ( 1, 0,-1,-1),
    (-1, 1,-1,-1), ( 0, 1,-1,-1), ( 1, 1,-1,-1),

    (-1,-1, 0,-1), ( 0,-1, 0,-1), ( 1,-1, 0,-1),
    (-1, 0, 0,-1), ( 0, 0, 0,-1), ( 1, 0, 0,-1),
    (-1, 1, 0,-1), ( 0, 1, 0,-1), ( 1, 1, 0,-1),

    (-1,-1, 1,-1), ( 0,-1, 1,-1), ( 1,-1, 1,-1),
    (-1, 0, 1,-1), ( 0, 0, 1,-1), ( 1, 0, 1,-1),
    (-1, 1, 1,-1), ( 0, 1, 1,-1), ( 1, 1, 1,-1),


    (-1,-1,-1, 0), ( 0,-1,-1, 0), ( 1,-1,-1, 0),
    (-1, 0,-1, 0), ( 0, 0,-1, 0), ( 1, 0,-1, 0),
    (-1, 1,-1, 0), ( 0, 1,-1, 0), ( 1, 1,-1, 0),

    (-1,-1, 0, 0), ( 0,-1, 0, 0), ( 1,-1, 0, 0),
    (-1, 0, 0, 0),                ( 1, 0, 0, 0),
    (-1, 1, 0, 0), ( 0, 1, 0, 0), ( 1, 1, 0, 0),

    (-1,-1, 1, 0), ( 0,-1, 1, 0), ( 1,-1, 1, 0),
    (-1, 0, 1, 0), ( 0, 0, 1, 0), ( 1, 0, 1, 0),
    (-1, 1, 1, 0), ( 0, 1, 1, 0), ( 1, 1, 1, 0),


    (-1,-1,-1, 1), ( 0,-1,-1, 1), ( 1,-1,-1, 1),
    (-1, 0,-1, 1), ( 0, 0,-1, 1), ( 1, 0,-1, 1),
    (-1, 1,-1, 1), ( 0, 1,-1, 1), ( 1, 1,-1, 1),

    (-1,-1, 0, 1), ( 0,-1, 0, 1), ( 1,-1, 0, 1),
    (-1, 0, 0, 1), ( 0, 0, 0, 1), ( 1, 0, 0, 1),
    (-1, 1, 0, 1), ( 0, 1, 0, 1), ( 1, 1, 0, 1),

    (-1,-1, 1, 1), ( 0,-1, 1, 1), ( 1,-1, 1, 1),
    (-1, 0, 1, 1), ( 0, 0, 1, 1), ( 1, 0, 1, 1),
    (-1, 1, 1, 1), ( 0, 1, 1, 1), ( 1, 1, 1, 1),
];

use Cube::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cube { Active, Inactive, }
impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Active => write!(f, "#"),
            Inactive => write!(f, "."),
        }
    }
}


#[derive(Clone)]
pub struct ConwayLife3D {
    pub map: HashSet<(i32,i32,i32)>,
    pub bbox: BBox3D<i32>,
}

impl ConwayLife3D {
    pub fn new() -> ConwayLife3D {
        ConwayLife3D { map: HashSet::new(), bbox: BBox3D::new() }
    }

    pub fn load(&mut self, plane: &str) {
        for (j, line) in plane.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.map.insert((i as i32, j as i32, 0));
                    self.bbox.update(&(i as i32, j as i32, 0));
                }
            }
        }
    }

    pub fn get(&self, pt: &(i32, i32, i32)) -> Cube {
        if self.map.contains(pt) { Active } else { Inactive }
    }

    pub fn step(&mut self) -> bool {
        let mut add = HashSet::new();
        let mut rm  = HashSet::new();

        for x in self.bbox.xmin()-1..=self.bbox.xmax()+1 {
        for y in self.bbox.ymin()-1..=self.bbox.ymax()+1 {
        for z in self.bbox.zmin()-1..=self.bbox.zmax()+1 {
            let mut count = 0;
            for (dx,dy,dz) in DIRECTIONS_3D.iter() {
                if self.get(&(x+dx,y+dy,z+dz)) == Active {
                    count += 1;
                    if count > 3 { break; } // shortcut
                }
            }
            if self.map.contains(&(x,y,z)) {
                if count != 2 && count != 3 { rm.insert((x,y,z)); }
            } else {
                if count == 3 { add.insert((x,y,z)); }
            }
        }}}

        if add.is_empty() && rm.is_empty() { return false; }

        for pt in rm.iter()   { self.map.remove(pt); }
        for pt in add.drain() {
            self.bbox.update(&pt);
            self.map.insert(pt);
        }
        return true;
    }

    #[inline]
    pub fn count(&self) -> usize { self.map.len() }
}
impl std::fmt::Display for ConwayLife3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in self.bbox.zmin()..=self.bbox.zmax() {
            if z > self.bbox.zmin() { write!(f, "\nz={}\n", z)?; }
            else                    { write!(f, "z={}\n", z)?; }
            for y in self.bbox.ymin()..=self.bbox.ymax() {
                if y > self.bbox.ymin() { write!(f, "\n")?; }
                for x in self.bbox.xmin()..=self.bbox.xmax() {
                    write!(f, "{}", self.get(&(x,y,z)))?;
                }
            }
        }
        Ok(())
    }
}


#[derive(Clone)]
pub struct ConwayLife4D {
    pub map: HashSet<(i32,i32,i32,i32)>,
    pub bbox: BBox4D<i32>,
}

impl ConwayLife4D {
    pub fn new() -> ConwayLife4D {
        ConwayLife4D { map: HashSet::new(), bbox: BBox4D::new() }
    }

    pub fn load(&mut self, plane: &str) {
        for (j, line) in plane.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.map.insert((i as i32, j as i32, 0, 0));
                    self.bbox.update(&(i as i32, j as i32, 0, 0));
                }
            }
        }
    }

    #[inline]
    pub fn get(&self, pt: &(i32, i32, i32, i32)) -> Cube {
        if self.map.contains(pt) { Active } else { Inactive }
    }

    pub fn step(&mut self) -> bool {
        let mut add = HashSet::new();
        let mut rm  = HashSet::new();

        for x in self.bbox.xmin()-1..=self.bbox.xmax()+1 {
        for y in self.bbox.ymin()-1..=self.bbox.ymax()+1 {
        for z in self.bbox.zmin()-1..=self.bbox.zmax()+1 {
        for w in self.bbox.wmin()-1..=self.bbox.wmax()+1 {
            let mut count = 0;
            for (dx,dy,dz,dw) in DIRECTIONS_4D.iter() {
                if self.get(&(x+dx,y+dy,z+dz,w+dw)) == Active {
                    count += 1;
                    if count > 3 { break; } // shortcut
                }
            }
            if self.map.contains(&(x,y,z,w)) {
                if count != 2 && count != 3 { rm.insert((x,y,z,w)); }
            } else {
                if count == 3 { add.insert((x,y,z,w)); }
            }
        }}}}

        if add.is_empty() && rm.is_empty() { return false; }

        for pt in rm.iter()   { self.map.remove(pt); }
        for pt in add.drain() {
            self.bbox.update(&pt);
            self.map.insert(pt);
        }
        return true;
    }

    #[inline]
    pub fn count(&self) -> usize { self.map.len() }
}


fn main() {
    let matches = App::new("Advent of code 2020, Day 17 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("17.in");
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let mut life = ConwayLife3D::new();
    life.load(&contents);
    for _ in 0..6 { life.step(); }
    println!("Part 1: after 6 cycles, {} cubes are active", life.count());

    let mut life = ConwayLife4D::new();
    life.load(&contents);
    for _ in 0..6 { life.step(); }
    println!("Part 2: after 6 cycles, {} cubes are active", life.count());
}
