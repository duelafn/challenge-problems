// SPDX-License-Identifier: MIT

use petgraph::algo::astar;
use petgraph::prelude::*;

use std::collections::HashMap;


fn wanted_part1() -> State {
    let mut state = State::default();
    state.room = [b'A', b'A', b'B', b'B', b'C', b'C', b'D', b'D'];
    return state;
}

fn wanted_part2() -> State2 {
    let mut state = State2::default();
    state.room = [b'A', b'A', b'A', b'A', b'B', b'B', b'B', b'B', b'C', b'C', b'C', b'C', b'D', b'D', b'D', b'D'];
    return state;
}

fn my_state() -> State {
    let mut state = State::default();
    state.room = [b'B', b'D', b'A', b'A', b'B', b'D', b'C', b'C'];
    return state;
}

#[allow(dead_code)]
fn test_state() -> State {
    let mut state = State::default();
    state.room = [b'B', b'A', b'C', b'D', b'B', b'C', b'D', b'A'];
    return state;
}

fn move_cost(t: u8) -> u64 {
    match t {
        b'A' => 1,
        b'B' => 10,
        b'C' => 100,
        b'D' => 1000,
        _ => unreachable!("Boom!"),
    }
}

fn ch(t: u8) -> char {
    match t {
        0    => '.',
        b'A' => 'A',
        b'B' => 'B',
        b'C' => 'C',
        b'D' => 'D',
        _    => unreachable!("Boom!"),
    }
}

fn likes_room(t: u8, rnum: usize, occupants: &[u8]) -> bool {
    if rnum != (t.wrapping_sub(b'A') as usize) { return false; }
    for amphipod in occupants { if *amphipod != t { return false; } }
    return true;
}


#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
struct State2 {
    // Hall: Acceptable locations (not in front of doors)
    hall: [u8; 7],
    // Room: smaller indices are to the FRONT of the room
    room: [u8; 16],
}

impl std::fmt::Display for State2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}x{}x{}x{}x{}{}  ::",
               ch(self.hall[0]), ch(self.hall[1]), ch(self.hall[2]), ch(self.hall[3]), ch(self.hall[4]), ch(self.hall[5]), ch(self.hall[6]),
        )?;
        for i in 0..4 {
            write!(f, "  {}{}{}{}", ch(self.room[4*i+0]), ch(self.room[4*i+1]), ch(self.room[4*i+2]), ch(self.room[4*i+3]))?;
        }
        Ok(())
    }
}

impl State2 {
    fn next(&self) -> Vec<(u64, State2)> {
        let mut rv = Vec::new();
        // Room to hall
        for i in 0..4 {
            if        self.room[4*i] != 0 { self._moves_to_hall(4*i, &mut rv); }
            else if self.room[4*i+1] != 0 { self._moves_to_hall(4*i+1, &mut rv); }
            else if self.room[4*i+2] != 0 { self._moves_to_hall(4*i+2, &mut rv); }
            else if self.room[4*i+3] != 0 { self._moves_to_hall(4*i+3, &mut rv); }
        }

        // Hall to room
        for i in 0..4 {
            if self.room[4*i] == 0 {
                if self.room[4*i+1] == 0 {
                    if self.room[4*i+2] == 0 {
                        if self.room[4*i+3] == 0 { self._moves_to_room(4*i+3, &mut rv); }
                        else { self._moves_to_room(4*i+2, &mut rv); }
                    }
                    else { self._moves_to_room(4*i+1, &mut rv); }
                }
                else { self._moves_to_room(4*i, &mut rv); }
            }
        }
        return rv;
    }

    fn _moves_to_room(&self, room: usize, rv: &mut Vec<(u64, State2)>) {
        let rnum = room / 4;
        // From the left
        let mut steps = (room as u64) % 4;
        for pos in (0..(rnum+2)).rev() {
            steps += 1;
            if pos > 0 { steps += 1; }
            if self.hall[pos] != 0 {
                if likes_room(self.hall[pos], rnum, &self.room[room+1..(4*(rnum+1))]) {
                    let mut new = self.clone();
                    new.hall[pos] = 0;
                    new.room[room] = self.hall[pos];
                    rv.push((steps * move_cost(self.hall[pos]), new));
                }
                break;
            }
        }

        // From the right
        let mut steps = (room as u64) % 4;
        for pos in (rnum+2)..7 {
            steps += 1;
            if pos < 6 { steps += 1; }
            if self.hall[pos] != 0 {
                if likes_room(self.hall[pos], rnum, &self.room[room+1..(4*(rnum+1))]) {
                    let mut new = self.clone();
                    new.hall[pos] = 0;
                    new.room[room] = self.hall[pos];
                    rv.push((steps * move_cost(self.hall[pos]), new));
                }
                break;
            }
        }
    }

    fn _moves_to_hall(&self, room: usize, rv: &mut Vec<(u64, State2)>) {
        let rnum = room / 4;
        let step = move_cost(self.room[room]);
        // Moving left
        let mut cost = ((room as u64) % 4) * step;
        for pos in (0..(rnum+2)).rev() {
            if self.hall[pos] != 0 { break; }
            cost += step;
            if pos > 0 { cost += step; }
            let mut new = self.clone();
            new.room[room] = 0;
            new.hall[pos] = self.room[room];
            rv.push((cost, new));
        }

        // Moving right
        let mut cost = ((room as u64) % 4) * step;
        for pos in (rnum+2)..7 {
            if self.hall[pos] != 0 { break; }
            cost += step;
            if pos < 6 { cost += step; }
            let mut new = self.clone();
            new.room[room] = 0;
            new.hall[pos] = self.room[room];
            rv.push((cost, new));
        }
    }
}


#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    // Hall: Acceptable locations (not in front of doors)
    hall: [u8; 7],
    // Room: smaller indices are to the FRONT of the room
    room: [u8; 8],
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}x{}x{}x{}x{}{}  ::  {}{}  {}{}  {}{}  {}{}",
               ch(self.hall[0]), ch(self.hall[1]), ch(self.hall[2]), ch(self.hall[3]), ch(self.hall[4]), ch(self.hall[5]), ch(self.hall[6]),
               ch(self.room[0]), ch(self.room[1]), ch(self.room[2]), ch(self.room[3]), ch(self.room[4]), ch(self.room[5]), ch(self.room[6]), ch(self.room[7]),
        )
    }
}

impl State {
    fn upgrade(&self) -> State2 {
        let mut state = State2::default();
        state.room[1]  = b'D'; state.room[2]  = b'D';
        state.room[5]  = b'C'; state.room[6]  = b'B';
        state.room[9]  = b'B'; state.room[10] = b'A';
        state.room[13] = b'A'; state.room[14] = b'C';
        for i in 0..4 {
            state.room[4*i] = self.room[2*i];
            state.room[4*i+3] = self.room[2*i+1];
        }
        return state;
    }

    fn next(&self) -> Vec<(u64, State)> {
        let mut rv = Vec::new();
        // Room to hall
        for i in 0..4 {
            if        self.room[2*i] != 0 { self._moves_to_hall(2*i, &mut rv); }
            else if self.room[2*i+1] != 0 { self._moves_to_hall(2*i+1, &mut rv); }
        }

        // Hall to room
        for i in 0..4 {
            if self.room[2*i] == 0 {
                if self.room[2*i+1] == 0 { self._moves_to_room(2*i+1, &mut rv); }
                self._moves_to_room(2*i, &mut rv);
            }
        }
        return rv;
    }

    fn _moves_to_room(&self, room: usize, rv: &mut Vec<(u64, State)>) {
        let rnum = room / 2;
        // From the left
        let mut steps = (room as u64) % 2;
        for pos in (0..(rnum+2)).rev() {
            steps += 1;
            if pos > 0 { steps += 1; }
            if self.hall[pos] != 0 {
                if likes_room(self.hall[pos], rnum, &self.room[room+1..(2*(rnum+1))]) {
                    let mut new = self.clone();
                    new.hall[pos] = 0;
                    new.room[room] = self.hall[pos];
                    rv.push((steps * move_cost(self.hall[pos]), new));
                }
                break;
            }
        }

        // From the right
        let mut steps = (room as u64) % 2;
        for pos in (rnum+2)..7 {
            steps += 1;
            if pos < 6 { steps += 1; }
            if self.hall[pos] != 0 {
                if likes_room(self.hall[pos], rnum, &self.room[room+1..(2*(rnum+1))]) {
                    let mut new = self.clone();
                    new.hall[pos] = 0;
                    new.room[room] = self.hall[pos];
                    rv.push((steps * move_cost(self.hall[pos]), new));
                }
                break;
            }
        }
    }

    fn _moves_to_hall(&self, room: usize, rv: &mut Vec<(u64, State)>) {
        let rnum = room / 2;
        let step = move_cost(self.room[room]);
        // Moving left
        let mut cost = ((room as u64) % 2) * step;
        for pos in (0..(rnum+2)).rev() {
            if self.hall[pos] != 0 { break; }
            cost += step;
            if pos > 0 { cost += step; }
            let mut new = self.clone();
            new.room[room] = 0;
            new.hall[pos] = self.room[room];
            rv.push((cost, new));
        }

        // Moving right
        let mut cost = ((room as u64) % 2) * step;
        for pos in (rnum+2)..7 {
            if self.hall[pos] != 0 { break; }
            cost += step;
            if pos < 6 { cost += step; }
            let mut new = self.clone();
            new.room[room] = 0;
            new.hall[pos] = self.room[room];
            rv.push((cost, new));
        }
    }
}

fn solve(state: State, wanted: State) -> u64 {
    let mut seen = HashMap::new();
    let mut graph = DiGraph::new();

    let mut states = vec![ state.clone() ];
    let start_node = graph.add_node(());
    seen.insert(state.clone(), start_node);
    while let Some(state) = states.pop() {
        let a = seen[&state];
        for (c, s) in state.next() {
            let b = if seen.contains_key(&s) {
                seen[&s]
            } else {
                let b = graph.add_node(());
                seen.insert(s.clone(), b);
                states.push(s);
                b
            };
            graph.add_edge(a, b, c);
        }
    }

    let goal = seen[&wanted];
    let res = astar(&graph, start_node, |n| n == goal, |e| *e.weight(), |_| 1).unwrap();
    return res.0;
}

fn solve2(state: State2, wanted: State2) -> u64 {
    let mut seen = HashMap::new();
    let mut graph = DiGraph::new();

    let mut states = vec![ state.clone() ];
    let start_node = graph.add_node(state.clone());
    seen.insert(state.clone(), start_node);
    while let Some(state) = states.pop() {
        let a = seen[&state];
        for (c, s) in state.next() {
            let b = if seen.contains_key(&s) {
                seen[&s]
            } else {
                let b = graph.add_node(s.clone());
                seen.insert(s.clone(), b);
                states.push(s);
                b
            };
            graph.add_edge(a, b, c);
        }
    }

    let goal = seen[&wanted];
    let res = astar(&graph, start_node, |n| n == goal, |e| *e.weight(), |_| 1).unwrap();
    return res.0;
}

fn main() {
    let start = my_state();
    println!("Part 1: {}", solve(start, wanted_part1()));

    let start = my_state().upgrade();
    println!("Part 2: {}", solve2(start, wanted_part2()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test23() {
        let state = test_state();
        assert_eq!(solve(state, wanted_part1()), 12521);

        let state = test_state().upgrade();
        assert_eq!(solve2(state, wanted_part2()), 44169);
    }
}
