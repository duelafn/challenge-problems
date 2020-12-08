// SPDX-License-Identifier: MIT

use advent_of_code::interp::*;
use advent_of_code::interp::Instruction::*;

use std::collections::HashSet;
use std::convert::TryFrom;

use clap::{Arg, App};


fn run_until_repeat(prog: &Interpreter, mut state: &mut InterpreterState) {
    let mut seen = HashSet::new();
    // NOTE: This also handles normal termination since step() outside the
    // program is a noop.
    while !seen.contains(&state.index) {
        seen.insert(state.index);
        prog.step(&mut state);
    }
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 08 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("08.in");
    let contents = std::fs::read_to_string(fname).unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
    let interp = Interpreter::try_from(contents.as_ref()).unwrap_or_else(|err| panic!("{}: {}", fname, err));

    let mut state = InterpreterState::new();
    run_until_repeat(&interp, &mut state);
    println!("Part 1 state: {:?}", state);

    let mut state = InterpreterState::new();
    loop {
        match interp.get(state.index) {
            None => { unreachable!("This should not happen"); }
            Some(Jmp(n)) => {
                let mut state2 = state.clone();
                state2.operate(&Nop(*n));
                run_until_repeat(&interp, &mut state2);
                if state2.index == interp.len() {
                    println!("Part 2 state: {:?}", state2);
                    break;
                } else {
                    state.operate(&Jmp(*n));
                }
            },
            Some(Nop(n)) => {
                let mut state2 = state.clone();
                state2.operate(&Jmp(*n));
                run_until_repeat(&interp, &mut state2);
                if state2.index == interp.len() {
                    println!("Part 2 state: {:?}", state2);
                    break;
                } else {
                    state.operate(&Nop(*n));
                }
            },
            _ => {
                interp.step(&mut state);
            },
        }
    }
}
