
extern crate clap;
extern crate intcode;

// Time Start: Wed, 11 Dec 2019 12:22:14 -0500
// Time Finish 1: Wed, 11 Dec 2019 13:44:04 -0500 (1 hour, 21 minutes, 50 seconds)
// Time Finish 2: Wed, 11 Dec 2019 13:46:13 -0500 (2 minutes, 9 seconds)
// Time Total: 1 hour, 23 minutes, 59 seconds

use clap::{Arg, App};

use intcode::Intcode;

use std::collections::HashMap;
use std::fmt;


#[derive(Clone, Copy)]
enum Color { Black, White }

#[derive(Clone, Copy)]
enum Direction { Up, Down, Left, Right }

#[derive(Clone, Copy)]
enum Turn { Left, Right }


#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn step(&mut self, dx: i32, dy: i32) {
        if let (Some(x), Some(y)) = (self.x.checked_add(dx), self.y.checked_add(dy)) {
            self.x = x;
            self.y = y;
        } else {
            panic!("Point overflow!");
        }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
struct BBox(i32,i32,i32,i32);// xmin, xmax, ymin, ymax
impl BBox {
    pub fn new() -> BBox {
        BBox(0,0,0,0)
    }

    pub fn xmin(&self) -> i32 { self.0 }
    pub fn xmax(&self) -> i32 { self.1 }
    pub fn ymin(&self) -> i32 { self.2 }
    pub fn ymax(&self) -> i32 { self.3 }

    pub fn update(&mut self, pt: &Point) {
        if pt.x < self.0 { self.0 = pt.x; }
        if pt.x > self.1 { self.1 = pt.x; }
        if pt.y < self.2 { self.2 = pt.y; }
        if pt.y > self.3 { self.3 = pt.y; }
    }
}
impl fmt::Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BBox: {}, {}, {}, {}", self.0, self.1, self.2, self.3)
    }
}

struct Hull {
    paint: HashMap<Point, Color>,
    pub bbox: BBox,
}
impl Hull {
    pub fn new() -> Hull {
        Hull {
            paint: HashMap::new(),
            bbox: BBox::new(),
        }
    }

    pub fn color_at(&self, pt: &Point) -> Color {
        match self.paint.get(&pt) {
            Some(x) => *x,
            _       => Color::Black,
        }
    }

    pub fn paint(&mut self, pt: &Point, color: Color) {
        self.bbox.update(&pt);
        self.paint.insert(pt.clone(), color);
    }

    pub fn count_painted(&self) -> usize { self.paint.len() }
}
impl fmt::Display for Hull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (self.bbox.ymin()..=self.bbox.ymax()).rev() {
            for x in self.bbox.xmin()..=self.bbox.xmax() {
                if let Err(err) = match self.color_at(&Point::new(x, y)) {
                    Color::Black => write!(f, "{}", " "),
                    Color::White => write!(f, "{}", "#"),
                } {
                    return Err(err);
                }
            }
            if let Err(err) = write!(f, "{}", "\n") {
                return Err(err);
            }
        }
        Ok(())
    }
}

struct Robot {
    pos: Point,
    dir: Direction,
}
impl Robot {
    pub fn new() -> Robot {
        Robot { pos: Point::new(0,0), dir: Direction::Up }
    }

    pub fn turn(&mut self, to: Turn) {
        match to {
            Turn::Left => match self.dir {
                Direction::Up    => { self.dir = Direction::Left },
                Direction::Down  => { self.dir = Direction::Right },
                Direction::Left  => { self.dir = Direction::Down },
                Direction::Right => { self.dir = Direction::Up },
            },
            Turn::Right => match self.dir {
                Direction::Up    => { self.dir = Direction::Right },
                Direction::Down  => { self.dir = Direction::Left },
                Direction::Left  => { self.dir = Direction::Up },
                Direction::Right => { self.dir = Direction::Down },
            },
        }
    }

    pub fn step(&mut self) {
        match self.dir {
            Direction::Up    => { self.pos.step(0, 1) },
            Direction::Down  => { self.pos.step(0, -1) },
            Direction::Left  => { self.pos.step(-1, 0) },
            Direction::Right => { self.pos.step(1, 0) },
        };
    }

    pub fn run_code(&mut self, ic: &mut Intcode, hull: &mut Hull) {
        loop {
            // Assumption: always needs exactly 1 input per position!
            ic.pipe(match hull.color_at(&self.pos) {
                Color::Black => 0,
                Color::White => 1,
            });

            while ic.output_len() < 2 && ic.step() { }

            if ic.is_halted() { break; }

            hull.paint(&self.pos, match ic.shift_output() {
                Some(0) => Color::Black,
                Some(1) => Color::White,
                _ => panic!("Color error!"),
            });

            self.turn(match ic.shift_output() {
                Some(0) => Turn::Left,
                Some(1) => Turn::Right,
                _ => panic!("Color error!"),
            });

            self.step();
        }
    }
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 11")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("11.in"));

    let mut ic = Intcode::load(&fname);
    let mut hull = Hull::new();
    let mut robot = Robot::new();

    robot.run_code(&mut ic, &mut hull);

    println!("Step 1: Painted {} panels", hull.count_painted());


    let mut ic = Intcode::load(&fname);
    let mut hull = Hull::new();
    let mut robot = Robot::new();

    hull.paint(&Point::new(0,0), Color::White);
    robot.run_code(&mut ic, &mut hull);

    println!("Step 2: Painted {} panels", hull.count_painted());
    println!("{}", hull);
}
