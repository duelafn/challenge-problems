
extern crate clap;
extern crate intcode;

// Time Start: Sat, 07 Dec 2019 09:27:44 -0500
// Time Finish 1: Sat, 07 Dec 2019 10:52:39 -0500 (1 hour, 24 minutes, 55 seconds)
// Time Finish 2: Sat, 07 Dec 2019 11:14:58 -0500 (22 minutes, 19 seconds)
// Time Total: 1 hour, 47 minutes, 14 seconds

use std::collections::VecDeque;

use clap::{Arg, App};
use intcode::Intcode;

// https://rosettacode.org/wiki/Permutations#Rust
fn permute<T, F: FnMut(&[T])>(used: &mut Vec<T>, unused: &mut VecDeque<T>, action: &mut F) {
    if unused.is_empty() {
        action(used);
    } else {
        for _ in 0..unused.len() {
            used.push(unused.pop_front().unwrap());
            permute(used, unused, action);
            unused.push_back(used.pop().unwrap());
        }
    }
}

fn load_amp(num: usize, fname: &String) -> Vec<Intcode> {
    (0..num).map(|_| Intcode::load(&fname)).collect::<Vec<_>>()
}
fn init_amp(num: usize, prog: &Vec<i64>) -> Vec<Intcode> {
    (0..num).map(|_| Intcode::init(prog.clone())).collect::<Vec<_>>()
}

fn thrusters(mut amp: Vec<Intcode>, phase: &[i64], input: i64) -> Result<i64, String> {
    let n = amp.len();
    for i in 0..n {
        amp[i].pipe(phase[i]);
    }
    amp[0].pipe(input);

    let mut stage = 0;
    loop {
        if ! amp[stage].step() {
            return Err(format!("Process {} exited without output", stage));
        }

        if amp[stage].has_output() {
            let out = amp[stage].cat();
            let rv = match out.get(0) {
                Some(x) => x,
                None => unreachable!("Process {} output None", stage),
            };
            if stage == n-1 { break; }
            else {
                stage += 1;
                amp[stage].pipe(*rv)
            }
        }
    }

    let out = amp[n-1].cat();
    return match out.get(0) {
        Some(x) => Ok(*x),
        None => Err(String::from("Process exited without output")),
    }
}


fn turbo_thrusters(mut amp: Vec<Intcode>, phase: &[i64], input: i64) -> Result<i64, String> {
    let n = amp.len();
    for i in 0..n {
        amp[i].pipe(phase[i]);
    }
    amp[0].pipe(input);

    let mut stage = 0;
    let mut rv = 0;
    loop {
        amp[stage].step();

        if amp[stage].has_output() {
            let out = amp[stage].shift_output();
            let val = match out {
                Some(x) => x,
                None => unreachable!("Process {} output None", stage),
            };
            stage += 1;
            if stage == n {
                rv = val;
                stage = 0;
            }
            amp[stage].pipe(val);
        }

        if amp[stage].is_halted() {
            if stage == n-1 { break; }
            stage += 1;
        }
    }

    return Ok(rv);
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 07")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("07.in"));

    let amp = load_amp(5, &fname);

    let mut queue = (0..(amp.len() as i64)).collect::<VecDeque<_>>();
    let mut max_phase = 0;
    let mut max_thrust = 0;
    permute(&mut Vec::new(), &mut queue, &mut |perm| {
        match thrusters(amp.clone(), perm, 0) {
            Ok(thrust) => {
                if thrust > max_thrust {
                    max_thrust = thrust;
                    max_phase = perm.iter().fold(0, |acc, d| 10 * acc + d);
                }
            },
            Err(msg) => panic!(msg),
        };
    });

    println!("Step 1: Maximum thrust is {} with phase {}", max_thrust, max_phase);


    let mut queue = ((amp.len() as i64)..(2*amp.len() as i64)).collect::<VecDeque<_>>();
    let mut max_phase = 0;
    let mut max_thrust = 0;
    permute(&mut Vec::new(), &mut queue, &mut |perm| {
        match turbo_thrusters(amp.clone(), perm, 0) {
            Ok(thrust) => {
                if thrust > max_thrust {
                    max_thrust = thrust;
                    max_phase = perm.iter().fold(0, |acc, d| 10 * acc + d);
                }
            },
            Err(msg) => panic!(msg),
        };
    });

    println!("Step 2: Maximum turbo-thrust is {} with phase {}", max_thrust, max_phase);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let amp = init_amp(5, &vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]);
        match thrusters(amp, &[4,3,2,1,0], 0) {
            Ok(thrust) => {
                assert_eq!(thrust, 43210, "Example 1 thrust");
            }
            Err(msg) => panic!(msg),
        };
    }

    #[test]
    fn example2() {
        let amp = init_amp(5, &vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]);
        match thrusters(amp, &[0,1,2,3,4], 0) {
            Ok(thrust) => {
                assert_eq!(thrust, 54321, "Example 2 thrust");
            }
            Err(msg) => panic!(msg),
        };
    }

    #[test]
    fn example3() {
        let amp = init_amp(5, &vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]);
        match thrusters(amp, &[1,0,4,3,2], 0) {
            Ok(thrust) => {
                assert_eq!(thrust, 65210, "Example 3 thrust");
            }
            Err(msg) => panic!(msg),
        };
    }


    #[test]
    fn example4() {
        let amp = init_amp(5, &vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]);
        match turbo_thrusters(amp, &[9,8,7,6,5], 0) {
            Ok(thrust) => {
                assert_eq!(thrust, 139629729, "Example 4 thrust");
            }
            Err(msg) => panic!(msg),
        };
    }

    #[test]
    fn example5() {
        let amp = init_amp(5, &vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]);
        match turbo_thrusters(amp, &[9,7,8,5,6], 0) {
            Ok(thrust) => {
                assert_eq!(thrust, 18216, "Example 5 thrust");
            }
            Err(msg) => panic!(msg),
        };
    }
}
