
extern crate clap;
extern crate intcode;

// Time Start: Tue, 17 Dec 2019 20:14:05 -0500
// Time Finish 1: Tue, 17 Dec 2019 22:34:14 -0500 (2 hours, 20 minutes, 9 seconds)
// Time Finish 2: Wed, 18 Dec 2019 04:58:40 -0500 (6 hours, 24 minutes, 26 seconds)
// Time Total: 8 hours, 44 minutes, 35 seconds

use clap::{Arg, App};

use intcode::{Intcode,Val};
use intcode::util::{Chart,Direction,Robot};

use std::collections::HashSet;
use std::collections::VecDeque;

const L: u8 = 76;
const R: u8 = 82;


fn walkable(_x: i64, _y: i64, ch: char) -> bool {
    match ch {
        '#' | '^' | '<' | '>' | 'v' => true,
        _ => false,
    }
}

fn alignment_parameter(chart: &Chart) -> i64 {
    let mut sum = 0;
    let top  = chart.bbox.ymax();
    let left = chart.bbox.xmin();
    for ((x, y), ch) in chart.map.iter() {
        if !walkable(*x, *y, *ch) { continue; }
        let mut wanted = true;
        for dir in Direction::each() {
            if wanted {
                let (a, b) = dir.step(*x, *y);
                wanted = walkable(a, b, chart.item_at(a, b));
            }
        }
        if wanted {
            sum += (x - left) * (top - y);
        }
    }
    return sum;
}

#[derive(Copy, Clone, PartialEq)]
pub enum CommandType { None, Turn, Move }

#[derive(Clone, Debug)]
pub struct Program {
    pub main: Vec<u8>,
    pub a: Vec<u8>,
    pub b: Vec<u8>,
    pub c: Vec<u8>,
    pub start: (i64, i64),
}
impl Program {
    pub fn new(x: i64, y: i64) -> Program {
        Program {
            main: Vec::with_capacity(10),
            a: Vec::with_capacity(10),
            b: Vec::with_capacity(10),
            c: Vec::with_capacity(10),
            start: (x, y),
        }
    }

    pub fn append(&mut self, cmd: u8) {
        match self.main.last() {
            Some(0) => self.a.push(cmd),
            Some(1) => self.b.push(cmd),
            Some(2) => self.c.push(cmd),
            None    => { self.main.push(0); self.a.push(cmd) },
            _       => unreachable!("Strange!")
        }
    }
    pub fn call(&mut self, func: u8) {
        self.main.push(func);
    }

    pub fn call_len(&self) -> usize {
        match self.main.last() {
            Some(0) => self.a.len(),
            Some(1) => self.b.len(),
            Some(2) => self.c.len(),
            _ => 0,
        }
    }

    pub fn last_type(&self) -> CommandType {
        let stack;
        match self.main.last() {
            Some(0) => { stack = &self.a; },
            Some(1) => { stack = &self.b; },
            Some(2) => { stack = &self.c; },
            _ => return CommandType::None,
        }

        match stack.last() {
            Some(x) if { 0 < *x && *x < 20 } => CommandType::Move,
            Some(&L) | Some(&R)              => CommandType::Turn,
            Some(x) => panic!("Bad command: {}", *x),
            None => CommandType::None,
        }
    }

    pub fn len_vec(&self, v: &Vec<u8>) -> u8 {
        let mut l = self.a.len() as u8 - 1;
        for val in v {
            l += match val {
                0..=9   => 1,
                10..=20 => 2,
                _ => 1,
            }
        }
        return l;
    }

    pub fn len_ok(&self) -> bool {
        if self.main.len() > 10 { return false; }
        if self.len_vec(&self.a) > 20 { return false; }
        if self.len_vec(&self.b) > 20 { return false; }
        if self.len_vec(&self.c) > 20 { return false; }
        return true;
    }

    pub fn run(&mut self, chart: &Chart, visited: &mut HashSet<(i64, i64)>) -> Result<(i64, i64, Direction, usize), ()> {
        if !self.len_ok() { return Err(()); }

        let (mut x, mut y) = self.start;
        let mut dir = Direction::North;
        visited.clear();
        visited.insert((x,y));

        let mut overlap = 0;
        for func in self.main.iter() {
            let f = match func { 0 => &self.a, 1 => &self.b, 2 => &self.c, _ => unreachable!("Bummer") };
            for step in f {
                match step {
                    &L => { dir = dir.left(); },
                    &R => { dir = dir.right(); },
                    steps => {
                        for _ in 0..*steps {
                            let (a, b) = dir.step(x, y);
                            x = a; y = b;
                            if chart.item_at(x, y) != '#' { return Err(()); }
                            if visited.contains(&(x,y)) {
                                overlap += 1;
                                if overlap > 2 { return Err(()); }
                            } else { overlap = 0; }
                            visited.insert((x,y));
                        }
                    },
                }
            }
        }
        return Ok((x, y, dir, visited.len()));
    }
}

pub fn find_program(chart: &Chart, program: &Program) -> Option<Program> {
    let mut todo: VecDeque<(i64, i64, Direction, Program)> = VecDeque::new();
    let mut visited = HashSet::new();
    let num_spaces = chart.map.len();
    todo.push_back((program.start.0, program.start.1, Direction::North, program.clone()));

    let mut best_len = 0;
    loop {
        let (x, y, dir, prog) = todo.pop_front()?;

        // Try to extend the current program with moves
        if prog.last_type() != CommandType::Move {
            let (mut a, mut b) = (x, y);
            for i in 1..20 {
                // Peek ahead:
                let (a1, b1) = dir.step(a, b); if chart.item_at(a1, b1) != '#' { break; }
                a = a1; b = b1;
                let mut p = prog.clone();
                p.append(i);
                if let Ok((x, y, dir, len)) = p.run(&chart, &mut visited) {
                    if len == num_spaces { return Some(p); }
                    if len > best_len { best_len = len; println!("length {}/{}, {:?}", len, num_spaces, p); }
                    todo.push_back((x, y, dir, p));
                }
            }
        }

        // Try to extend the current program with turns
        if prog.last_type() != CommandType::Turn {
            let (a, b) = dir.left().step(x, y);
            if chart.item_at(a, b) == '#' { // look-ahead
                let mut p = prog.clone();
                p.append(L);
                todo.push_back((x, y, dir.left(), p));
            }
            let (a, b) = dir.right().step(x, y); // look-ahead
            if chart.item_at(a, b) == '#' {
                let mut p = prog.clone();
                p.append(R);
                todo.push_back((x, y, dir.right(), p));
            }
        }

        // Try to call another function, but only if current call is deep enough
        if prog.main.len() > 0 && prog.call_len() > 2 {
            for i in 0..3 {
                let mut p = prog.clone();
                p.call(i);
                if let Ok((x,y, dir, len)) = p.run(&chart, &mut visited) {
                    if len == num_spaces { return Some(p); }
                    if len > best_len { best_len = len; println!("length {}/{}, {:?}", len, num_spaces, p); }
                    todo.push_back((x, y, dir, p));
                }
            }
        }
    }
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 17")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("17.in"));

    let mut ic = Intcode::load(&fname);
    let mut chart = Chart::new();
    let mut bot = Robot::new();

    ic.run();
    let (mut x, mut y) = (0, 0);
    loop {
        if let Some(num) = ic.shift_output() {
            match num {
                n if n < 0 => panic!("Unexpected value: {}", n),
                10 => { y -= 1; x = -1; },
                35 => { chart.put(x, y, '#'); },
                94 => { chart.put(x, y, '#'); bot.set_pos(x, y); bot.set_direction(Direction::North); },
                46 => (),  // Don't need to record empty space
                n => panic!("Unexpected character: {}", n),
            }
            x += 1;
        } else { break; }
    }

    // println!("{}", chart);
    println!("Part 1: {}", alignment_parameter(&chart));

    let mut all_points = HashSet::new();
    for key in chart.map.keys() {
        all_points.insert(key.clone());
    }

    let program = Program::new(bot.pos().0, bot.pos().1);
    let prog = find_program(&chart, &program).unwrap_or_else(|| panic!("Did not find a solution :("));
//     let prog = Program { main: vec![0, 1, 0, 2, 0, 1, 2, 0, 1, 2], a: vec![82, 8, 82, 10, 82, 10], b: vec![82, 4, 82, 8, 82, 10, 82, 12], c: vec![82, 12, 82, 4, 76, 12, 76, 12], start: (28, 0) };
    // println!("{:?}", prog);

    let mut ic = Intcode::load(&fname);
    ic.set(&Val::Address(0), 2);
    let mut code = String::new();

    let mapper = |v: &u8| match *v { L => String::from("L"), R => String::from("R"), v => v.to_string() };

    code += &(prog.main.iter().map(|v| String::from_utf8(vec![65+v]).unwrap()).collect::<Vec<String>>().join(",") + "\n");
    code += &(prog.a.iter().map(mapper).collect::<Vec<String>>().join(",") + "\n");
    code += &(prog.b.iter().map(mapper).collect::<Vec<String>>().join(",") + "\n");
    code += &(prog.c.iter().map(mapper).collect::<Vec<String>>().join(",") + "\n");
    code += &"n\n";
    for ch in code.chars() {
        ic.pipe(ch as i64);
    }

    ic.run();
    // println!("{}", ic.cat().iter().map(|c| std::char::from_u32(*c as u32).unwrap()).collect::<String>());
    println!("Part 2: {}", ic.cat().last().unwrap_or_else(|| panic!("Bummer")));
}
