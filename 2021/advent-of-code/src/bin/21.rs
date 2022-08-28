// SPDX-License-Identifier: MIT

use std::collections::HashMap;

// Part 1:
trait Die {
    fn roll(&mut self) -> u32;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct DeterministicDice {
    next: u32,
}
impl DeterministicDice {
    fn new() -> DeterministicDice { DeterministicDice { next: 1 } }
}
impl Die for DeterministicDice {
    fn roll(&mut self) -> u32 {
        let rv = self.next;
        self.next += 1;
        if self.next > 100 { self.next = 1; }
        return rv;
    }
}

struct Game {
    turns:  usize,
    pos:    [u32; 2],
    score:  [u32; 2],
    die:    DeterministicDice,
}

impl Game {
    fn new(a: u32, b: u32) -> Game {
        Game { pos: [a-1, b-1], score: [0, 0], turns: 0, die: DeterministicDice::new() }
    }

    // Return false when game over
    fn step(&mut self) -> bool {
        let player = self.turns % 2;
        self.turns += 1;
        for _ in 0..3 {
            let roll = self.die.roll();
            self.pos[player] = (self.pos[player] + roll) % 10;
        }
        self.score[player] += 1 + self.pos[player];
        return self.score[player] < 1000;
    }

    fn checksum(&self) -> u32 {
        self.score[0].min(self.score[1]) * 3 * (self.turns as u32)
    }
}


// Part 2:
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    last:  u8,
    pos:   (u8, u8),
    score: (u8, u8),
}

impl State {
    fn new(a: u8, b: u8) -> State {
        State { last: 1, pos: (a-1, b-1), score: (0, 0) }
    }

    fn step(&self, roll: u8) -> State {
        if self.last == 1 {
            let newpos = (self.pos.0 + roll) % 10;
            State {
                last:  0,
                pos:   (newpos, self.pos.1),
                score: (self.score.0 + newpos + 1, self.score.1),
            }
        } else {
            let newpos = (self.pos.1 + roll) % 10;
            State {
                last:  1,
                pos:   (self.pos.0, newpos),
                score: (self.score.0, self.score.1 + newpos + 1),
            }
        }
    }

    fn done(&self) -> bool { (self.score.0 >= 21) || (self.score.1 >= 21) }
    // winner only valid after done()
    fn winner(&self) -> u8 { self.last }
}

fn dirac_dice(a: u8, b: u8) -> u64 {
    // game length: 3..21 turns
    // Dice histogram:
    //  outcome: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    let hist = [ 0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
    // Maximum: 44100 states
    let mut multiverse = HashMap::with_capacity(10*10*21*21);
    let mut reboot = HashMap::with_capacity(10*10*21*21);
    let mut wins = (0, 0);

    // Initial game state
    multiverse.insert(State::new(a, b), 1_u64);

    while !multiverse.is_empty() {
        for (input, count) in multiverse.drain() {
            for (roll, times) in hist.iter().enumerate().skip(3) {
                let next = input.step(roll as u8);
                if next.done() {
                    let wins = if next.winner() == 0 { &mut wins.0 } else { &mut wins.1 };
                    *wins += count * times;
                } else {
                    *reboot.entry(next).or_insert(0) += count * times;
                }
            }
        }
        // multiverse is drained, swap our pointers:
        std::mem::swap(&mut multiverse, &mut reboot);
    }

    return wins.0.max(wins.1);
}


fn main() {
    let mut game = Game::new(6, 3);
    while game.step() { }
    println!("Part 1: {}", game.checksum());
    println!("Part 2: {}", dirac_dice(6, 3));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test21() {
        let mut game = Game::new(4, 8);
        while game.step() { }
        println!("Scores: {:?};  turn: {};  rolls: {}", game.score, game.turns, 3*game.turns);
        assert_eq!(game.checksum(), 739785);
        assert_eq!(dirac_dice(4, 8), 444356092776315);
    }
}
