// SPDX-License-Identifier: MIT

use core::slice::Iter;
use std::fmt::Display;


pub fn print_map<D: Display>(map: &Vec<Vec<D>>) {
    for row in map.iter() {
        for v in row.iter() {
            print!("{}", v);
        }
        print!("\n");
    }
}


/// Four cardinal directions.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Dir4 { N, E, S, W }

impl Dir4 {
    pub fn next(&self) -> Self {
        use Dir4::*;
        match self { N => E, E => S, S => W, W => N }
    }

    pub fn iter() -> Iter<'static, Dir4> {
        use Dir4::*; static DIRECTIONS: [Dir4; 4] = [N, E, S, W];
        DIRECTIONS.iter()
    }

//     pub fn dx(&self) -> i64 { match self { Dir4::N =>  0, Dir4::E => 1, Dir4::S => 0, Dir4::W => -1 } }
//     pub fn dy(&self) -> i64 { match self { Dir4::N => -1, Dir4::E => 0, Dir4::S => 1, Dir4::W =>  0 } }
//
//     pub fn step(&self, x: i64, y: i64) -> (i64, i64) {
//         match self { Dir4::N => (x, y-1), Dir4::E => (x+1, y), Dir4::S => (x, y+1), Dir4::W => (x-1, y) }
//     }

    pub fn step_usize(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Dir4::N => if y > 0 { Some((x, y-1)) } else { None },
            Dir4::E => Some((x+1, y)),
            Dir4::S => Some((x, y+1)),
            Dir4::W => if x > 0 { Some((x-1, y)) } else { None },
        }
    }
}

// impl From<usize> for Dir4 {
//     fn from(src: usize) -> Self {
//         use Dir4::*;
//         match src % 4 { 0 => N, 1 => E, 2 => S, 3 => W, _ => unreachable!(), }
//     }
// }

// impl std::convert::TryFrom<&str> for Dir4 {
//     type Error = ();
//     fn try_from(src: &str) -> Result<Self, ()> {
//         use Dir4::*;
//         match src {
//             "N" | "North" | "north" | "n" => Ok(N),
//             "E" | "East"  | "east"  | "e" => Ok(E),
//             "S" | "South" | "south" | "s" => Ok(S),
//             "W" | "West"  | "west"  | "w" => Ok(W),
//             _ => Err(()),
//         }
//     }
// }


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Dir8 { N, NE, E, SE, S, SW, W, NW }

impl Dir8 {
    pub fn next(&self) -> Self {
        use Dir8::*;
        match self { N => NE, NE => E, E => SE, SE => S, S => SW, SW => W, W => NW, NW => N }
    }

    pub fn iter() -> Iter<'static, Dir8> {
        use Dir8::*; static DIRECTIONS: [Dir8; 8] = [N, NE, E, SE, S, SW, W, NW];
        DIRECTIONS.iter()
    }

//     pub fn dx(&self) -> i64 {
//         use Dir8::*;
//         match self { N =>  0, NE =>  1, E =>  1, SE =>  1, S =>  0, SW => -1, W => -1, NW => -1 }
//     }
//     pub fn dy(&self) -> i64 {
//         use Dir8::*;
//         match self { N => -1, NE => -1, E =>  0, SE =>  1, S =>  1, SW =>  1, W =>  0, NW => -1 }
//     }
//
//     pub fn step(&self, x: i64, y: i64) -> (i64, i64) { (x + self.dx(), y + self.dy()) }

    pub fn step_usize(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        use Dir8::*;
        match self {
            N  => if y > 0 { Some((x, y-1)) } else { None },
            NE => if y > 0 { Some((x+1, y-1)) } else { None },
            E  => Some((x+1, y)),
            SE => Some((x+1, y+1)),
            S  => Some((x, y+1)),
            SW => if x > 0 { Some((x-1, y+1)) } else { None },
            W  => if x > 0 { Some((x-1, y)) } else { None },
            NW => if x > 0 && y > 0 { Some((x-1, y-1)) } else { None },
        }
    }
}



#[derive(Clone)]
pub struct MapIter2D<'a, T> {
    map: &'a Vec<Vec<T>>,
    next: (usize, usize),
}

impl<'a, T> MapIter2D<'a, T> {
    pub fn new(map: &'a Vec<Vec<T>>) -> MapIter2D<'a, T> {
        MapIter2D { map, next: (0, 0) }
    }
}

impl<'a, T> Iterator for MapIter2D<'a, T> {
    type Item = (usize, usize, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (mut c, mut r) = self.next;
        loop {
            match self.map.get(r) {
                Some(row) => {
                    match row.get(c) {
                        Some(val) => {
                            self.next = (c+1, r);
                            return Some((c, r, val));
                        },
                        None => {
                            r += 1;
                            c = 0;
                        },
                    }
                },
                None => { return None; },
            }
        }
    }
}



#[derive(Clone)]
pub struct MapNeighbor4Iter2D<'a, T> {
    map: &'a Vec<Vec<T>>,
    pos: (usize, usize),
    next: Option<Dir4>,
}

impl<'a, T> MapNeighbor4Iter2D<'a, T> {
    pub fn new(map: &'a Vec<Vec<T>>, x: usize, y: usize) -> MapNeighbor4Iter2D<'a, T> {
        MapNeighbor4Iter2D { map, pos: (x, y), next: Some(Dir4::N) }
    }
}

impl<'a, T> Iterator for MapNeighbor4Iter2D<'a, T> {
    type Item = (usize, usize, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next.take() {
                None => { return None; },

                Some(dir) => {
                    // We start with north, so finish with west
                    if dir != Dir4::W {
                        self.next = Some(dir.next());
                    }

                    if let Some((x, y)) = dir.step_usize(self.pos.0, self.pos.1) {
                        if let Some(row) = self.map.get(y) {
                            if let Some(val) = row.get(x) {
                                return Some((x, y, val));
                            }
                        }
                    }
                }
            }
        }
    }
}



#[derive(Clone)]
pub struct MapNeighbor8Iter2D<'a, T> {
    map: &'a Vec<Vec<T>>,
    pos: (usize, usize),
    iter: Iter<'static, Dir8>,
}

impl<'a, T> MapNeighbor8Iter2D<'a, T> {
    pub fn new(map: &'a Vec<Vec<T>>, x: usize, y: usize) -> MapNeighbor8Iter2D<'a, T> {
        MapNeighbor8Iter2D { map, pos: (x, y), iter: Dir8::iter() }
    }
}

impl<'a, T> Iterator for MapNeighbor8Iter2D<'a, T> {
    type Item = (usize, usize, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => { return None; },
                Some(dir) => {
                    if let Some((x, y)) = dir.step_usize(self.pos.0, self.pos.1) {
                        if let Some(row) = self.map.get(y) {
                            if let Some(val) = row.get(x) {
                                return Some((x, y, val));
                            }
                        }
                    }
                }
            }
        }
    }
}
