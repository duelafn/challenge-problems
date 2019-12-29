
extern crate clap;
extern crate intcode;

use std::io;
use std::io::Write;

use clap::{Arg, App};

use intcode::Intcode;
use intcode::util::{Chart, Robot, Direction::*};


fn add_walls(chart: &mut Chart, bot: &Robot, msg: &String) {
    let doors = items("Doors here lead:", &msg);
    if doors.len() > 0 {
        let (x, y) = bot.pos();
        if !doors.contains(&String::from("north")) { chart.put(x, y+1, '#'); }
        if !doors.contains(&String::from("south")) { chart.put(x, y-1, '#'); }
        if !doors.contains(&String::from("east"))  { chart.put(x+1, y, '#'); }
        if !doors.contains(&String::from("west"))  { chart.put(x-1, y, '#'); }
    }
}

fn inventory(ic: &mut Intcode) -> Vec<String> {
    ic.ascii_in(&String::from("inv\n"));
    ic.run();
    let msg = ic.ascii_out();
    ic.clear_output();
    return items("Items in your inventory:", &msg);
}

fn items<S: Into<String>>(prefix: S, msg: &String) -> Vec<String> {
    let mut items = Vec::new();
    let mut in_items = false;
    let pre = prefix.into();
    for line in msg.lines() {
        if line == pre { in_items = true; }
        else if in_items {
            if line == "" { break; }
            if let Some(item) = line.get(2..) {
                items.push(String::from(item));
            }
        }
    }
    return items;
}

fn direction<S: Into<String>>(s: S) -> String {
    match s.into().as_str() {
        "n"     => String::from("north"),
        "s"     => String::from("south"),
        "e"     => String::from("east"),
        "w"     => String::from("west"),
        "north" => String::from("north"),
        "south" => String::from("south"),
        "east"  => String::from("east"),
        "west"  => String::from("west"),
         _      => panic!("Invalid direction!"),
    }
}

fn unlock_door<S: Into<String>>(mut ic: &mut Intcode, dir: S) -> Option<String> {
    let dir = direction(dir);
    let items = inventory(&mut ic);
    let num = items.len();
    let n = 2_u32.pow(num as u32);
    let mut last = n - 1;
    let mut s = String::new();
    let mut rv = String::new();
    rv.push_str(&format!("{:?}\n", items));

    for i in (0..n).rev() {
        rv.push_str(&format!("{} ", i));
        for k in 0..num {
            let old = 1 & (last >> k);
            let new = 1 & (i >> k);
            if 1 == new { rv.push_str(&format!("{} ", items[k])); }
            if old != new {
                s.clear();
                if 1 == new {
                    s.push_str("take ");
                } else {
                    s.push_str("drop ");
                }
                s.push_str(&items[k]);
                s.push_str("\n");
                ic.ascii_in(&s);
            }
        }
        ic.run();
        ic.clear_output();

        s.clear();
        s.push_str(&dir);
        s.push_str("\n");
        ic.ascii_in(&s);
        ic.run();

        let msg = ic.ascii_out();
        if msg.contains("are heavier than") || msg.contains("are lighter than") {
            rv.push_str(&format!("-> {}\n", if msg.contains("are heavier than") { "too light" } else { "too heavy" }));
            last = i;
        } else {
            return None;
        }
    }

    return None;
//     return Some(rv);
}

fn play_interactive(xic: &Intcode, xchart: &Chart, xbot: &Robot) {
    let mut buffer = String::new();
    let mut ic = xic.clone();
    let mut chart = xchart.clone();
    let mut bot = xbot.clone();

    let mut save_ic = xic.clone();
    let mut save_chart = xchart.clone();
    let mut save_bot = xbot.clone();

    let mut note = Vec::new();

    loop {
        let msg = ic.ascii_out();
        ic.clear_output();
        println!("\x1B[H\x1B[2J");

        if msg.contains("You can't go that way.") { bot.step(-1); }
        let (x, y) = bot.pos();
        let ch = chart.item_at(x, y);
        add_walls(&mut chart, &bot, &msg);

        for m in note.iter() { println!("{}", m); }
        note.clear();

        chart.put(x, y, if ch == ' ' || ch == '.' { 'O' } else { '@' });
        println!("{}\n\n{}", chart, msg);
        chart.put(x, y, if ch == ' ' { '.' } else { ch });

        print!("> ");
        io::stdout().flush().unwrap();
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap_or_else(|err| panic!("Read error: {}", err));

        let code = match buffer.trim() {
            "?" | "h" => {
                note.push(String::from("
q             - quit
n, s, e, w    - move in direction
u*            - solve pressure plate (unlock) in direction (n,s,e,w)
i             - inventory
t, d          - take/drop all here (issue when 'Items here' message is visible)
save          - save current state
load          - load poreciously saved state
.*            - label current space with '*' (any character)
<OTHER>       - any other legal command
"));
                None
            },
            "q" => { return; }
            "n" => { bot.set_direction(North); bot.step(1); Some("north") },
            "s" => { bot.set_direction(South); bot.step(1); Some("south") },
            "e" => { bot.set_direction(East); bot.step(1); Some("east") },
            "w" => { bot.set_direction(West); bot.step(1); Some("west") },
            "i" => Some("inv"),
            "d" => {
                let mut s = String::new();
                for item in inventory(&mut ic) {
                    s.push_str("drop ");
                    s.push_str(&item.as_str());
                    s.push_str("\n");
                }
                ic.ascii_in(&s);
                None
            },
            "t" => {
                let mut s = String::new();
                for item in items("Items here:", &msg) {
                    s.push_str("take ");
                    s.push_str(&item.as_str());
                    s.push_str("\n");
                }
                ic.ascii_in(&s);
                None
            },
            ""  => None,
            "save" => {
                save_ic = ic.clone();
                save_chart = chart.clone();
                save_bot = bot.clone();
                None
            },
            "load" => {
                ic = save_ic.clone();
                chart = save_chart.clone();
                bot = save_bot.clone();
                None
            }
            s if s.starts_with("u") => {
                if let Some(ch) = s.chars().nth(1) {
                    if let Some(msg) = unlock_door(&mut ic, ch.to_string()) {
                        note.push(msg);
                    }
                }
                None
            },
            s if s.starts_with(".") => {
                if let Some(ch) = s.chars().nth(1) {
                    chart.put(x, y, ch);
                }
                None
            },
            s   => Some(s),
        };

        if let Some(cmd) = code {
            ic.ascii_in(&String::from(cmd));
            ic.ascii_in(&String::from("\n"));
        }
        ic.run();
    }
}



fn main() {
    let matches = App::new("Advent of Code 2019, Day 25")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("25.in"));

    let mut ic = Intcode::load(&fname);
    let chart = Chart::new();
    let bot = Robot::new();
    ic.run();
    play_interactive(&ic, &chart, &bot);
}
