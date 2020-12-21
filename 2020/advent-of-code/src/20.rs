// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::bbox::BBox;
use advent_of_code::parse::StrParser;

use clap::{Arg, App};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub id: u16,
    pub flip: Option<bool>,
    pub rot: Option<u8>,
    img: Vec<String>,
    edges: Vec<u16>,
}
impl Tile {
    pub fn new(src: &str) -> Tile {
        let mut iter = src.lines();
        let mut parser = StrParser::new(iter.next().unwrap_or_else(|| panic!("Missing id line")));
        parser.expect_str("Tile").unwrap_or_else(|e| panic!("Missing Tile label: {}", e.to_string()));
        let id = parser.extract_u64().unwrap_or_else(|e| panic!("Can't find id: {}", e.to_string())) as u16;
        let img :Vec<String> = iter.map(|x| x.to_string()).collect();
        let mut edges = vec![0,0,0,0,0,0,0,0];

        for (i, s) in img.iter().enumerate() {
            let chars = s.chars().collect::<Vec<char>>();
            if chars[0] == '#' {
                edges[3] |= 1<<i;               // Left: bottom to top
                edges[7] |= 1<<(9-i);           // Left: top to bottom
            }
            if chars[9] == '#' {
                edges[1] |= 1<<(9-i);           // Right: top to bottom
                edges[5] |= 1<<i;               // Right: bottom to top
            }
            if i == 0 {
                for j in 0..10 {
                    if chars[j] == '#' {
                        edges[0] |= 1<<(9-j);   // Top: left to right
                        edges[4] |= 1<<j;       // Top: right to left
                    }
                }
            }
            if i == 9 {
                for j in 0..10 {
                    if chars[j] == '#' {
                        edges[2] |= 1<<j;       // Bottom: right to left
                        edges[6] |= 1<<(9-j);   // Bottom: left ro tight
                    }
                }
            }
        }

        Tile { id, img, edges, flip: None, rot: None }
    }

    #[inline]
    pub fn get_marker(&self, dir: u8, flip: bool, rot: u8) -> u16 {
        let idx = match (dir, flip, rot) {
            (0, false, 0) => 0, (1, false, 0) => 5, (2, false, 0) => 6, (3, false, 0) => 3,
            (0, false, 1) => 1, (1, false, 1) => 6, (2, false, 1) => 7, (3, false, 1) => 0,
            (0, false, 2) => 2, (1, false, 2) => 7, (2, false, 2) => 4, (3, false, 2) => 1,
            (0, false, 3) => 3, (1, false, 3) => 4, (2, false, 3) => 5, (3, false, 3) => 2,

            (0, true,  0) => 4, (1, true,  0) => 3, (2, true,  0) => 2, (3, true,  0) => 5,
            (0, true,  1) => 7, (1, true,  1) => 2, (2, true,  1) => 1, (3, true,  1) => 4,
            (0, true,  2) => 6, (1, true,  2) => 1, (2, true,  2) => 0, (3, true,  2) => 7,
            (0, true,  3) => 5, (1, true,  3) => 0, (2, true,  3) => 3, (3, true,  3) => 6,

            _ => panic!("Invalid marker query: ({}, {}, {})", dir, flip, rot),
        };
        return self.edges[idx];
    }

    #[inline]
    pub fn try_orient(&self, mark: u16, dir: u8) -> Option<(bool, u8)>  {
        for rot in 0..4 {
            if mark == self.get_marker(dir, false, rot) { return Some((false, rot)); }
        }
        for rot in 0..4 {
            if mark == self.get_marker(dir, true,  rot) { return Some((true,  rot)); }
        }
        return None;
    }

    pub fn blit(&self, map: &mut Vec<Vec<char>>, x: usize, y: usize) {
        for j in 1..9 {
            let mut chars = self.img[j].chars();
            chars.next();
            for i in 1..9 {
                map[y+j-1][x+i-1] = chars.next().unwrap();
            }
        }
    }
}

fn solve(tiles: &mut HashMap<u16, Tile>) -> (HashMap<(i16,i16), Option<(u16, bool, u8)>>, BBox<i16>) {
    let mut to_place :HashSet<u16> = tiles.keys().map(|x| *x).collect();
    let id = *to_place.iter().next().unwrap_or_else(|| panic!("No tiles!?"));
    to_place.remove(&id);
    let mut map = HashMap::new();
    map.insert((0,0), Some((id, false, 0)));
    let mut places = vec![(0_i16,0_i16)];
    let mut bbox = BBox::new();

    while !to_place.is_empty() {
        let here = places.pop().unwrap_or_else(|| panic!("No more monkeys, but {} to place", to_place.len()));
        let (id, flip, rot) = map.get(&here).unwrap_or_else(|| panic!("fell off the map")).unwrap_or_else(|| panic!("tile gone"));
        let this = tiles.get(&id).unwrap_or_else(|| panic!("lost the tile"));

        'direction:
        for dir in 0..4 {
            let target = step(&here, dir);
            if map.contains_key(&target) { continue; }

            let mark = this.get_marker(dir, flip, rot);
            let dir_from = direction_from(dir);
            for nid in to_place.iter() {
                let tile = tiles.get(nid).unwrap();
                if let Some((flip, rot)) = tile.try_orient(mark, dir_from) {
                    to_place.remove(&tile.id);
                    bbox.update(&target);
                    places.push(target.clone());
                    map.insert(target, Some((tile.id, flip, rot)));
                    continue 'direction;
                }
            }
            // else, no matches:
            map.insert(target, None);
        }
    }

    return (map, bbox);
}



fn main() {
    let matches = App::new("Advent of code 2020, Day 20 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("20.in");
    let mut tiles = records(fname);

    let (soln, bbox) = solve(&mut tiles);
    let corners = [
        (bbox.xmin(), bbox.ymin()), (bbox.xmax(), bbox.ymin()),
        (bbox.xmin(), bbox.ymax()), (bbox.xmax(), bbox.ymax()),
    ];
    let mut product = 1;
    for pt in corners.iter() {
        let (id, _, _) = soln.get(&pt).unwrap().unwrap();
        product *= id as u64;
    }
    println!("Part 1: corner product = {}", product);

    // Here there be dragons
    let dragon = Pattern::new(r"
                  #
#    ##    ##    ###
 #  #  #  #  #  #
");
    let mut map = Map::new(soln, bbox, tiles);

    for rot in 0..8 {
        let m = map.matches(&dragon);
        if m.len() > 0 {
            println!("Found {} sea monsters", m.len());
            map.mask(&dragon, &m, 'O');
            break;
        } else {
            if rot == 3 { map.flip(0, 0, map.width, map.height); }
            else { map.rot(0, 0, map.width); }
        }
    }

    println!("Part 2: the stormy seas: {}", map.count('#'));
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}
impl Map {
    pub fn new(soln: HashMap<(i16,i16), Option<(u16, bool, u8)>>, bbox: BBox<i16>, tiles: HashMap<u16, Tile>) -> Map {
        let mul = 8;
        let width = mul * bbox.width() as usize;
        let height = mul * bbox.height() as usize;
        if width != height { panic!("Expected width == height"); }
        let mut map = Vec::with_capacity(height);
        for _ in 0..height { map.push(vec![' '; width]); }
        let mut map = Map { map, width, height };

        for j in bbox.yrange() {
            for i in bbox.xrange() {
                let (id, flip, rot) = soln.get(&(i, j)).unwrap().unwrap_or_else(|| panic!("point not in solution!"));
                let tile = tiles.get(&id).unwrap_or_else(|| panic!("lost tile"));
                let x = mul * (i - bbox.xmin()) as usize;
                let y = mul * (j - bbox.ymin()) as usize;

                tile.blit(&mut map.map, x, y);
                if flip { map.flip(x, y, mul, mul); }
                for _ in 0..rot { map.rot(x, y, mul); }
            }
        }

        return map;
    }

    pub fn mask(&mut self, pat: &Pattern, matches: &Vec<(usize, usize)>, ch: char) {
        for &(i, j) in matches.iter() {
            for &(dx, dy) in pat.iter() {
                self.map[j+dy][i+dx] = ch;
            }
        }
    }

    pub fn count(&self, ch: char) -> usize {
        let mut count = 0;
        for j in 0..self.width {
            for i in 0..self.height {
                if self.map[j][i] == ch { count += 1; }
            }
        }
        return count;
    }

    pub fn matches(&self, pat: &Pattern) -> Vec<(usize, usize)> {
        let mut rv = Vec::new();
        for j in 0..(self.height-pat.height) {
            for i in 0..(self.width-pat.width) {
                let mut matched = true;
                for &(dx, dy) in pat.iter() {
                    if self.map[j+dy][i+dx] != '#' {
                        matched = false;
                        break;
                    }
                }
                if matched { rv.push((i, j)); }
            }
        }
        return rv;
    }

    pub fn flip(&mut self, x: usize, y: usize, w: usize, h: usize) {
        for j in y..(y+h) {
            for i in 0..(w/2) {
                let tmp = self.map[j][x+i];
                self.map[j][x+i] = self.map[j][x+w-1-i];
                self.map[j][x+w-1-i] = tmp;
            }
        }
    }

    pub fn rot(&mut self, x: usize, y: usize, d: usize) {
        let mut tmp = Vec::with_capacity(d);
        for j in y..(y+d) {
            let mut row = Vec::with_capacity(d);
            for i in x..(x+d) { row.push(self.map[j][i]); }
            tmp.push(row);
        }

        for j in 0..d {
            for i in 0..d {
                self.map[y+d-1-i][x+j] = tmp[j][i];
            }
        }
    }

}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                write!(f, "{}", self.map[j][i])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}




#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    vec: Vec<(usize, usize)>,
    pub width: usize,
    pub height: usize,
}
impl Pattern {
    pub fn new(src: &str) -> Pattern {
        let mut vec = Vec::new();
        let mut bbox = BBox::new();
        for (j, line) in src.lines().enumerate() {
            for (i, ch) in line.chars().enumerate() {
                if ch == '#' {
                    if vec.len() == 0 { bbox.set(&(i,j)); }
                    else { bbox.update(&(i,j)); }
                    vec.push((i, j));
                }
            }
        }
        for (x, y) in vec.iter_mut() {
            *x -= bbox.xmin();
            *y -= bbox.ymin();
        }
//         if bbox.xmin() != 0 || bbox.ymin() != 0 { panic!("Not implemented!"); }
        Pattern { vec, width: bbox.width(), height: bbox.height() }
    }
}
impl std::ops::Deref for Pattern {
    type Target = Vec<(usize, usize)>;
    fn deref(&self) -> &Self::Target { &self.vec }
}


fn records(fname: &str) -> HashMap<u16, Tile> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
    return contents.trim_end().split("\n\n").map(|chunk| { let t = Tile::new(chunk); (t.id, t) }).collect();
}

#[inline]
fn direction_from(dir: u8) -> u8 { (dir + 2) % 4 }

#[inline]
fn step(&(x, y): &(i16, i16), dir: u8) -> (i16, i16) {
    match dir {
        0 => (x, y-1),
        1 => (x+1, y),
        2 => (x, y+1),
        3 => (x-1, y),
        _ => panic!("Bad 2"),
    }
}
