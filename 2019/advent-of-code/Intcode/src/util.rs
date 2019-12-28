
extern crate num_traits;

use self::num_traits::{Num, Signed, One, Zero};

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;


#[derive(Copy, Clone)]
pub struct BBox(i64,i64,i64,i64);// xmin, xmax, ymin, ymax
impl BBox {
    pub fn new() -> BBox {
        BBox(0,0,0,0)
    }

    pub fn xmin(&self) -> i64 { self.0 }
    pub fn xmax(&self) -> i64 { self.1 }
    pub fn ymin(&self) -> i64 { self.2 }
    pub fn ymax(&self) -> i64 { self.3 }

    pub fn update(&mut self, x: i64, y: i64) {
        if x < self.0 { self.0 = x; }
        if x > self.1 { self.1 = x; }
        if y < self.2 { self.2 = y; }
        if y > self.3 { self.3 = y; }
    }
}
impl fmt::Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BBox: {}, {}, {}, {}", self.0, self.1, self.2, self.3)
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Direction { North, South, East, West }
const ALL_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
impl Direction {
    pub fn from_xy<T: Num + Signed + Zero>(x: T, y: T) -> Option<Direction> {
        if x.is_zero() && y.is_positive() { return Some(Direction::North); }
        if x.is_zero() && y.is_negative() { return Some(Direction::South); }
        if y.is_zero() && x.is_positive() { return Some(Direction::East); }
        if y.is_zero() && x.is_negative() { return Some(Direction::West); }
        return None
    }

    pub fn each() -> &'static [Direction] { &ALL_DIRECTIONS }

    pub fn step<T: Num + One>(&self, x: T, y: T) -> (T, T) {
        match self {
            Direction::North => (x, y + T::one()),
            Direction::South => (x, y - T::one()),
            Direction::East  => (x + T::one(), y),
            Direction::West  => (x - T::one(), y),
        }
    }

    // xy: y-axis north is positive
    pub fn xy(&self) -> (i64, i64) {
        match self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::East  => (1, 0),
            Direction::West  => (-1, 0),
        }
    }

    // xY: y-axis inversion
    pub fn xY(&self) -> (i64, i64) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East  => (1, 0),
            Direction::West  => (-1, 0),
        }
    }

    pub fn rev(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
        }
    }
    pub fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East  => Direction::North,
            Direction::West  => Direction::South,
        }
    }
    pub fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East  => Direction::South,
            Direction::West  => Direction::North,
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::North => "N",
            Direction::South => "S",
            Direction::East  => "E",
            Direction::West  => "W",
        })
    }
}


#[derive(Clone)]
pub struct Robot {
    pos: (i64, i64),
    dir: Direction,
}
impl Robot {
    pub fn new() -> Robot {
        Robot { pos: (0,0), dir: Direction::North }
    }

    pub fn pos(&mut self) -> (i64, i64) { self.pos }
    pub fn set_pos(&mut self, x: i64, y: i64) { self.pos = (x, y); }

    pub fn direction(&mut self) -> Direction { self.dir }
    pub fn set_direction(&mut self, new: Direction) { self.dir = new; }

    pub fn step(&mut self, n: i64) {
        let (x, y) = self.dir.xy();
        self.pos.0 = self.pos.0.checked_add(x.checked_mul(n).unwrap_or_else(|| panic!("X Mul Overflow"))).unwrap_or_else(|| panic!("X Add Overflow"));
        self.pos.1 = self.pos.1.checked_add(y.checked_mul(n).unwrap_or_else(|| panic!("Y Mul Overflow"))).unwrap_or_else(|| panic!("Y Add Overflow"));
    }
}


#[derive(Clone)]
pub struct Chart {
    pub map: HashMap<(i64, i64), char>,
    pub bbox: BBox,
    pub portals: HashMap<(i64, i64), (i64, i64)>,
    pub portal_symbol: char,
    pub print_flipped: bool,
}
impl Chart {
    pub fn new() -> Chart {
        Chart {
            map: HashMap::new(),
            bbox: BBox::new(),
            portal_symbol: '^',
            portals: HashMap::new(),
            print_flipped: false,
        }
    }

    pub fn load(fname: &str) -> Chart {
        let mut chart = Chart::new();
        let contents = fs::read_to_string(fname)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

        for (j, line) in contents.trim_end_matches('\n').split('\n').enumerate() {
            for (i, ch) in line.chars().enumerate() {
                chart.put(i as i64, j as i64, ch);
            }
        }
        return chart;
    }

    pub fn count(&self, wanted: impl Fn(i64, i64, char) -> bool) -> i64 {
        let mut count = 0;
        for ((x, y), ch) in self.map.iter() {
            if wanted(*x, *y, *ch) {
                count += 1;
            }
        }
        return count;
    }

    pub fn count_at(&self, x: i64, y: i64, ch: char) -> i64 {
        match self.map.get(&(x,y)) {
            Some(c) => if *c == ch { 1 } else { 0 },
            None    => 0,
        }
    }

    pub fn item_at(&self, x: i64, y: i64) -> char {
        match self.map.get(&(x,y)) {
            Some(x) => *x,
            None    => ' ',
        }
    }

    pub fn locate(&self, ch: char) -> Option<(i64, i64)> {
        for (key, val) in self.map.iter() {
            if *val == ch {
                return Some(*key);
            }
        }
        return None;
    }

    pub fn follow_portal(&self, x0: i64, y0: i64) -> (i64, i64) {
        match self.portals.get(&(x0,y0)) {
            Some(&(x, y)) => (x,y),
            None    => panic!("Tried to follow non-existant portal at ({}, {})", x0, y0),
        }
    }
    pub fn add_portal(&mut self, x0: i64, y0: i64, x1: i64, y1: i64) {
        let sym = self.portal_symbol;
        self.put(x0, y0, sym);
        self.portals.insert((x0, y0), (x1, y1));
    }

    pub fn get(&self, x: i64, y: i64) -> Option<&char> { self.map.get(&(x,y)) }
    pub fn put(&mut self, x: i64, y: i64, obj: char) {
        self.bbox.update(x, y);
        self.map.insert((x, y), obj);
    }

    pub fn shortest_path(&self, x: i64, y: i64, valid: impl Fn(i64, i64, char) -> bool, wanted: impl Fn(i64, i64, char) -> bool) -> Option<(i64, i64, Vec<Direction>)> {
        let mut todo: VecDeque<(i64, i64, Vec<Direction>)> = VecDeque::new();
        let mut seen: HashSet<(i64, i64)> = HashSet::new();

        todo.push_back((x, y, Vec::new()));
        loop {
            let (x, y, path) = todo.pop_front()?; // or return None
            let spot = self.item_at(x, y);
            if wanted(x, y, spot) {
                return Some((x, y, path));
            }
            if valid(x, y, spot) {
                for dir in Direction::each() {
                    let (mut a, mut b) = dir.step(x, y);
                    if self.item_at(a, b) == self.portal_symbol {
                        let (aa, bb) = self.follow_portal(a, b);
                        a = aa; b = bb;
                    }
                    if !seen.contains(&(a, b)) {
                        let mut newpath = path.clone();
                        newpath.push(*dir);
                        todo.push_back((a, b, newpath));
                    }
                }
            }
            seen.insert((x, y));
        };
    }

}
impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.print_flipped {
            for y in self.bbox.ymin()..=self.bbox.ymax() {
                for x in self.bbox.xmin()..=self.bbox.xmax() {
                    write!(f, "{}", self.item_at(x, y))?;
                }
                write!(f, "{}", "\n")?;
            }
        } else {
            for y in (self.bbox.ymin()..=self.bbox.ymax()).rev() {
                for x in self.bbox.xmin()..=self.bbox.xmax() {
                    write!(f, "{}", self.item_at(x, y))?;
                }
                write!(f, "{}", "\n")?;
            }
        }
        Ok(())
    }
}
