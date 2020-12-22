// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::{Hash,Hasher};

use clap::{Arg, App};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    level: usize,
    p1: VecDeque<usize>,
    p2: VecDeque<usize>,
    seen: HashSet<u64>,
}
impl Game {
    pub fn new() -> Game {
        Game { p1: VecDeque::new(), p2: VecDeque::new(), seen: HashSet::new(), level: 0 }
    }

    pub fn score(&mut self, player: u8) -> usize {
        let l = if player == 1 { &self.p1 } else { &self.p2 };
        let mut mul = l.len();
        let mut score = 0;
        for c in l.iter() {
            score += mul * *c;
            mul -= 1;
        }
        return score;
    }

    pub fn id(&self) -> u64 {
        let mut state = DefaultHasher::new();
        for c in self.p1.iter() { c.hash(&mut state); }
        ';'.hash(&mut state);
        for c in self.p2.iter() { c.hash(&mut state); }
        return state.finish();
    }

    pub fn play1(&mut self) -> u8 {
        loop {
            let rv = self.one_round_1();
            if rv != 0 { return rv; }
        }
    }

    pub fn one_round_1(&mut self) -> u8 {
        if self.p1.len() == 0 || self.p2.len() == 0 { return 99; }
        let c1 = self.p1.pop_front().unwrap();
        let c2 = self.p2.pop_front().unwrap();
        if c1 > c2 {
            self.p1.push_back(c1);
            self.p1.push_back(c2);
            if self.p2.is_empty() { return 1; }
        } else {
            self.p2.push_back(c2);
            self.p2.push_back(c1);
            if self.p1.is_empty() { return 2; }
        }
        return 0;
    }

    pub fn play2(&mut self) -> u8 {
        loop {
            let rv = self.one_round_2();
            if rv != 0 { return rv; }
        }
    }

    pub fn one_round_2(&mut self) -> u8 {
        if self.p1.len() == 0 || self.p2.len() == 0 { return 99; }
        if self.seen.contains(&self.id()) { return 1; }
        self.seen.insert(self.id());

        let c1 = self.p1.pop_front().unwrap();
        let c2 = self.p2.pop_front().unwrap();

        let winner;
        if c1 <= self.p1.len() && c2 <= self.p2.len() {
            let mut rec = Game::new();
            rec.level = self.level + 1;
            // Largest card determines winner except for the repeat rule
            // which favors player 1. Thus, if player 1 also has the
            // largest card, we know they win, and since the score of the
            // subgame is irrelevant, we don't have to actually play it out.
            let mut max = 0;
            for i in 0..c1 { if self.p1[i] > max { max = self.p1[i]; }; rec.p1.push_back(self.p1[i]); }
            for i in 0..c2 { if self.p2[i] > max { max = 0; };          rec.p2.push_back(self.p2[i]); }
            if max > 0 { winner = 1; }
            else { winner = rec.play2(); }
        } else {
            winner = if c1 > c2 { 1 } else { 2 };
        }

        let done;
        if winner == 1 {
            self.p1.push_back(c1);
            self.p1.push_back(c2);
            done = if self.p2.is_empty() { 1 } else { 0 };
        } else {
            self.p2.push_back(c2);
            self.p2.push_back(c1);
            done = if self.p1.is_empty() { 2 } else { 0 };
        }

        return done;
    }
}

impl std::convert::TryFrom<&str> for Game {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut player = 0;
        let mut game = Game::new();
        for (i, line) in src.lines().enumerate() {
            if line.starts_with("Player 1") { player = 1; continue; }
            if line.starts_with("Player 2") { player = 2; continue; }
            if line.is_empty() { continue; }

            let card = line.parse().map_err(|err| format!("Parse error at '{}' in line {}: {}", line, i+1, err))?;
            if player == 1 { game.p1.push_back(card); }
            else if player == 2 { game.p2.push_back(card); }
            else { return Err(format!("Parse error at '{}' in line {}: no active player", line, i+1)); }
        }
        return Ok(game);
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pad = "   ".repeat(self.level);
        write!(f, "{}Player 1's deck: {}\n", pad, self.p1.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "))?;
        write!(f, "{}Player 2's deck: {}\n", pad, self.p2.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "))
    }
}


fn main() {
    let matches = App::new("Advent of code 2020, Day 22 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("22.in");
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let game = Game::try_from(contents.as_str()).unwrap_or_else(|e| panic!("{}", e));

    let mut game1 = game.clone();
    let player = game1.play1();
    println!("Part 1: Player {} wins with score {}", player, game1.score(player));

    let mut game2 = game.clone();
    let player = game2.play2();
    println!("Part 2: Player {} wins with score {}", player, game2.score(player));
}
