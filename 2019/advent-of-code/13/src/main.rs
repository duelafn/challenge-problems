
extern crate clap;
extern crate intcode;

// Time Start: Fri, 13 Dec 2019 09:13:42 -0500
// Time Finish 1: Fri, 13 Dec 2019 09:38:12 -0500 (24 minutes, 30 seconds)
// Time Finish 2: Fri, 13 Dec 2019 10:41:59 -0500 (1 hour, 3 minutes, 47 seconds)
// Time Total: 1 hour, 28 minutes, 17 seconds

use clap::{Arg, App};

use intcode::Intcode;
use intcode::Val;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::{thread, time};

#[derive(Copy, Clone)]
struct BBox(i64,i64,i64,i64);// xmin, xmax, ymin, ymax
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

struct Screen {
    pub screen: HashMap<(i64, i64), u8>,
    pub bbox: BBox,
}
impl Screen {
    pub fn new() -> Screen {
        Screen {
            screen: HashMap::new(),
            bbox: BBox::new(),
        }
    }

    pub fn item_at(&self, x: i64, y: i64) -> u8 {
        match self.screen.get(&(x,y)) {
            Some(x) => *x,
            None    => 0,
        }
    }

    pub fn put(&mut self, x: i64, y: i64, obj: u8) {
        self.bbox.update(x, y);
        self.screen.insert((x, y), obj);
    }
}
impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (self.bbox.ymin()..=self.bbox.ymax()).rev() {
            for x in self.bbox.xmin()..=self.bbox.xmax() {
                if let Err(err) = match self.item_at(x, y) {
                    0 => write!(f, "{}", " "), // empty
                    1 => write!(f, "{}", "#"), // wall
                    2 => write!(f, "{}", "@"), // block
                    3 => write!(f, "{}", "-"), // paddle
                    4 => write!(f, "{}", "*"), // ball
                    x => panic!("Unexpected object: {}", x),
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


struct Pong {
    paddle: i64,
    ball_x: i64,
    ball_dx: i64,
    pub score: i64,
    blocks: HashSet<(i64, i64)>,
}
impl Pong {
    pub fn new() -> Pong {
        Pong {
            paddle:  0,
            ball_x:  9999,
            ball_dx: 0,
            score:   0,
            blocks: HashSet::new(),
        }
    }

    fn draw(&self, screen: &Screen) {
        println!("\x1B[H\x1B[2J{}\n\n Score: {}", screen, self.score);
        thread::sleep(time::Duration::from_millis(10));
    }

    fn run_code(&mut self, ic: &mut Intcode) {
        let mut screen = Screen::new();
        ic.set_input(0);
        loop {
            while ic.output_len() < 3 && ic.step() { }

            if ic.is_halted() { break; }

            let x = ic.shift_output().expect("x output disappeared!");
            let y = ic.shift_output().expect("y output disappeared!");
            let obj = ic.shift_output().expect("object output disappeared!");

            if x == -1 && y == 0 {
                self.score = obj;
                continue;
            }

            screen.put(x, y, obj as u8);

            match obj {
                0 => { self.blocks.remove(&(x,y)); },                                         // empty
                1 => (),                                                                      // wall
                2 => { self.blocks.insert((x,y)); },                                          // block
                3 => { self.paddle = x; self.draw(&screen); },                                // paddle
                4 => { self.ball_dx = x - self.ball_x; self.ball_x = x; self.draw(&screen); } // ball
                x => unreachable!("Unexpected object {}", x),
            }

            if self.ball_dx < 1000 { self.ball_dx = 0; } // ugly handling of initial conditions

            if self.paddle < self.ball_x + self.ball_dx {
                ic.set_input(1);
            }
            else if self.paddle > self.ball_x + self.ball_dx {
                ic.set_input(-1);
            }
            else {
                ic.set_input(0);
            }
        }
        self.draw(&screen);
    }
}


fn run_code(ic: &mut Intcode, screen: &mut Screen) {
    loop {
        while ic.output_len() < 3 && ic.step() { }

        if ic.is_halted() { break; }

        let x = ic.shift_output().expect("x output disappeared!");
        let y = ic.shift_output().expect("y output disappeared!");
        let obj = ic.shift_output().expect("object output disappeared!") as u8;
        screen.put(x, y, obj);
    }
}



fn main() {
    let matches = App::new("Advent of Code 2019, Day 13")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("13.in"));

    let mut ic = Intcode::load(&fname);
    let mut pong = Pong::new();
    ic.set(&Val::Address(0), 2); // Insert quarter

    pong.run_code(&mut ic);

    println!("Step 2: final score {}", pong.score);


    let mut ic = Intcode::load(&fname);
    let mut screen = Screen::new();
    run_code(&mut ic, &mut screen);

    let num_blocks: u32 = screen.screen.values().map(|v| match v { 2 => 1, _ => 0 }).sum();
    println!("Step 1: {} block tiles after running", num_blocks);
}
