// SPDX-License-Identifier: MIT

// use std::collections::HashMap;

use advent_of_code::parse::StrParser;

use clap::{Arg, App};
use regex::Regex;

type RulePath = Vec<(usize, u8)>;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Char(char),
    Match1(usize),
    Match2(usize, usize),
    Match3(usize, usize, usize),
    Alt(usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ruleset {
    rules: Vec<Option<Node>>,
}
impl Ruleset {
    pub fn new(rules: Vec<Option<Node>>) -> Ruleset {
        Ruleset { rules }
    }

    pub fn to_regex_str(&self, i: usize) -> String {
        match self.rules[i] {
            Some(Node::Char(ch))        => { ch.to_string() }
            Some(Node::Match1(a))       => { self.to_regex_str(a) }
            Some(Node::Match2(a, b))    => { format!("{}{}", self.to_regex_str(a), self.to_regex_str(b)) }
            Some(Node::Match3(a, b, c)) => { format!("{}{}{}", self.to_regex_str(a), self.to_regex_str(b), self.to_regex_str(c)) }
            Some(Node::Alt(a, b))       => { format!("(?:{}|{})", self.to_regex_str(a), self.to_regex_str(b)) }
            None                        => { panic!("Bummer"); }
        }
    }

//     pub fn is_match(&self, word: &str) -> bool {
//         let mut matcher = RulesetMatcher::new(&self, word);
//         return matcher.is_match();
//     }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleMatcher<'a> {
    matcher: &'a Ruleset,
    paths: Vec<RulePath>,
    chars_read: usize,
}
impl<'a> RuleMatcher<'a> {
    pub fn new(matcher: &'a Ruleset) -> RuleMatcher<'a> {
        RuleMatcher { paths: Vec::new(), chars_read: 0, matcher }
    }

    pub fn read(&mut self, ch: char) -> usize {
        // Initialize the list from path 0
        if self.chars_read == 0 {
            self.chars_read = 1;
            self.paths = paths_to(self.matcher, ch, 0);
        } else if self.paths.len() > 0 {
            self.chars_read += 1;
            let mut vec = Vec::new();
            for mut path in self.paths.drain(..) {
                step(self.matcher, &mut path);
                if let Some((rule, p)) = path.pop() {
                    match self.matcher.rules[rule] {
                        None => { continue; }
                        Some(Node::Char(c1))  => {
                            if c1 == ch { path.push((rule, p)); vec.push(path); }
                        },
                        Some(Node::Alt(i, j)) => { // Examine alternatives, but Alt() is excluded from the path
                            vec.append(&mut with_parent(&path, paths_to(self.matcher, ch, i)));
                            vec.append(&mut with_parent(&path, paths_to(self.matcher, ch, j)));
                        },
                        _ => { unreachable!("Match rule in read?"); },
                    }
                }
            }
            self.paths = vec;
        }
        return self.paths.len();
    }
}

// RuleMatcher helper functions
fn with(&(rule, p): &(usize, u8), mut paths: Vec<RulePath>) -> Vec<RulePath> {
    for v in paths.iter_mut() { v.push( (rule, p) ); }
    paths
}

fn with_parent(parent: &RulePath, mut paths: Vec<RulePath>) -> Vec<RulePath> {
    paths.drain(..).map(|mut p| { let mut new = parent.clone(); new.append(&mut p); new }).collect()
}

fn paths_to(matcher: &Ruleset, ch: char, rule: usize) -> Vec<RulePath> {
    loop {
        match matcher.rules[rule] {
            None                             => { return Vec::new(); }
            Some(Node::Char(c1))             => { if c1 == ch { return vec![vec![(rule,0)]] } else { return Vec::new() }; }
            Some(Node::Match1(i))            => { return paths_to(matcher, ch, i) } // Reparent children of single-item matchers
            Some(Node::Match2(i, ..))        => { return with(&(rule, 0),  paths_to(matcher, ch, i)) }
            Some(Node::Match3(i, ..))        => { return with(&(rule, 0),  paths_to(matcher, ch, i)) }
            Some(Node::Alt(i, j))            => {
                let mut a = paths_to(matcher, ch, i);
                let mut b = paths_to(matcher, ch, j);
                match (a.len(), b.len()) {
                    (0, 0) => { return a; },
                    (_, 0) => { return a; },
                    (0, _) => { return b; },
                    (_, _) => { a.append(&mut b); return a; },
                }
            }
        }
    }
}

fn step(matcher: &Ruleset, path: &mut RulePath) -> bool {
    loop {
        if let Some(&(rule, part)) = path.last() {
            match matcher.rules[rule].as_ref() {
                None                             => { unreachable!("step None?"); }
                Some(Node::Alt(..))              => { unreachable!("step Alt?"); } // Shouldn't see any Alt() (excluded from paths_to(), should accept in resolve())
                Some(Node::Char(..))             => { path.pop(); } // step back out pf a Char()
                _                                => { path.pop(); path.push((rule, part+1)); return resolve(matcher, path); } // Advance anything else
            }
        } else { return false; }
    }
}

// Char and Alt are considered terminals for higher-level resolution.
// Everything else resolve and extend the path.
fn resolve(matcher: &Ruleset, path: &mut RulePath) -> bool {
    loop {
        if let Some(&(rule, part)) = path.last() {
            match (matcher.rules[rule].as_ref(), part) {
                (None, _)                        => { unreachable!("resolve None?"); }
                (Some(Node::Char(_)), _)         => { return true; }
                (Some(Node::Alt(_, _)), _)       => { return true; }

                (Some(Node::Match1(i)), 0)       => { path.pop(); path.push((*i, 0)); }
                (Some(Node::Match1(_)), _)       => { path.pop(); return step(matcher, path); }

                (Some(Node::Match2(i, _)), 0)    => { path.push((*i, 0)); }
                (Some(Node::Match2(_, j)), 1)    => { path.pop(); path.push((*j, 0)); }
                (Some(Node::Match2(_, _)), _)    => { path.pop(); return step(matcher, path); }

                (Some(Node::Match3(i, _, _)), 0) => { path.push((*i, 0)); }
                (Some(Node::Match3(_, j, _)), 1) => { path.push((*j, 0)); }
                (Some(Node::Match3(_, _, k)), 2) => { path.pop(); path.push((*k, 0)); }
                (Some(Node::Match3(_, _, _)), _) => { path.pop(); return step(matcher, path); }
            }
        } else { return false; }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesetMatcher<'a> {
    s: Vec<char>,
    i: usize,
    matcher: &'a Ruleset,
    node: Vec<(usize, usize, u8)>,
}
impl<'a> RulesetMatcher<'a> {
    pub fn new(matcher: &'a Ruleset, src: &str) -> RulesetMatcher<'a> {
        RulesetMatcher { s: src.chars().collect(), i: 0, node: vec![(0,0,0)], matcher }
    }

    fn resolve(&mut self) -> bool {
        loop {
            if let Some(&(_pos, rule, part)) = self.node.last() {
                match (self.matcher.rules[rule].as_ref(), part) {
                    (None, _)                        => { return false; }
                    (Some(Node::Char(..)), _)        => { return true; }

                    (Some(Node::Match1(i)), 0)       => { self.node.pop(); self.node.push((self.i, *i, 0)); }
                    (Some(Node::Match1(_)), _)       => { self.node.pop(); return self.next(); }

                    (Some(Node::Match2(i, _)), 0)    => { self.node.push((self.i, *i, 0)); }
                    (Some(Node::Match2(_, j)), 1)    => { self.node.pop(); self.node.push((self.i, *j, 0)); }
                    (Some(Node::Match2(_, _)), _)    => { self.node.pop(); return self.next(); }

                    (Some(Node::Match3(i, _, _)), 0) => { self.node.push((self.i, *i, 0)); }
                    (Some(Node::Match3(_, j, _)), 1) => { self.node.push((self.i, *j, 0)); }
                    (Some(Node::Match3(_, _, k)), 2) => { self.node.pop(); self.node.push((self.i, *k, 0)); }
                    (Some(Node::Match3(_, _, _)), _) => { self.node.pop(); return self.next(); }

                    (Some(Node::Alt(i, _)), 0)       => { self.node.push((self.i, *i, 0)); }
                    (Some(Node::Alt(_, j)), 1)       => { self.node.pop(); self.node.push((self.i, *j, 0)); }
                    (Some(Node::Alt(_, _)), _)       => { self.node.pop(); return self.next(); }
                }
            } else { return false; }
        }
    }

    fn next_alt(&mut self) -> bool {
        loop {
            if let Some(&(pos, rule, part)) = self.node.last() {
                match self.matcher.rules[rule].as_ref() {
                    None                             => { return false; }
                    Some(Node::Char(..))             => { self.node.pop(); }
                    Some(Node::Match1(..))           => { self.node.pop(); }
                    Some(Node::Match2(..))           => { self.node.pop(); }
                    Some(Node::Match3(..))           => { self.node.pop(); }
                    Some(Node::Alt(..))              => { self.node.pop(); self.i = pos; self.node.push((self.i, rule, part+1)); return self.resolve(); }
                }
            } else { return false; }
        }
    }

    fn next(&mut self) -> bool {
        loop {
            if let Some(&(pos, rule, part)) = self.node.last() {
                match self.matcher.rules[rule].as_ref() {
                    None                             => { return false; }
                    Some(Node::Char(..))             => { self.node.pop(); }
                    Some(Node::Alt(..))              => { self.node.pop(); self.i = pos; self.node.push((self.i, rule, part+1)); return self.resolve(); }
                    _                                => { self.node.pop(); self.node.push((self.i, rule, part+1)); return self.resolve(); }
                }
            } else { return false; }
        }
    }

    fn check(&self) -> bool {
        if let Some(&(_pos, rule, _)) = self.node.last() {
            match self.matcher.rules[rule].as_ref() {
                Some(Node::Char(ch))                 => { return self.s[self.i] == *ch; }
                _                                    => { panic!("Error: check attempted on un-resolve()d path"); }
            }
        }
        return false;
    }

    pub fn is_match(&mut self) -> bool {
        self.resolve();
        while self.i < self.s.len() {
            let more;
            println!("{}. {}", self.i, self.node.iter().map(|p| format!("({},{},{})", p.0, p.1, p.2)).collect::<Vec<String>>().join(" - "));
            if self.check() { self.i += 1; more = self.next(); }
            else            { more = self.next_alt(); }
            if !more { break; }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        println!("Match FINISHED!");
        return self.i == self.s.len();
    }
}



fn extract_match(p: &mut StrParser) -> Result<Node, String> {
    let mut m = Vec::new();
    while p.has_more() && Some('|') != p.peek() {
        m.push(p.extract_u64()? as usize);
    }
    match m.len() {
        1 => Ok(Node::Match1(m[0])),
        2 => Ok(Node::Match2(m[0], m[1])),
        3 => Ok(Node::Match3(m[0], m[1], m[2])),
        n => Err(format!("Unexpected number of consecutive matches {} in '{}'", n, p.full_string())),
    }
}

fn records(fname: &str) -> Result<(Ruleset, Vec<String>), String> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let mut rules = vec![None; 200]; // CHEAT!
    let mut strings = Vec::new();
    for line in contents.lines() {
        if line.is_empty() { continue; }

        // Just collect rules for now, can't build proper tree due to order issues
        if line.chars().next().map(|a| a.is_ascii_digit()).unwrap_or(false) {
            let mut p = StrParser::new_skip_ws(line);
            let idx = p.extract_u64()? as usize;
            p.expect_char(':')?;

            if p.skip_char('"') > 0 {
                let ch = p.step()?;
                p.expect_char('"')?;
                p.expect_eol()?;
                rules[idx] = Some(Node::Char(ch));
            }

            else {
                let a = extract_match(&mut p)?;
                if p.skip_char('|') > 0 {
                    let b = extract_match(&mut p)?;
                    p.expect_eol()?;
                    let i = rules.len();
                    rules.push(Some(a));
                    rules.push(Some(b));
                    rules[idx] = Some(Node::Alt(i, i+1));
                }
                else {
                    p.expect_eol()?;
                    rules[idx] = Some(a);
                }
            }
        }
        else { strings.push(line.to_string()); }
    }

    return Ok((Ruleset::new(rules), strings));
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 19 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("19.in");

    let (mut rules, words) = records(fname).unwrap();
//     println!("{}", rules.to_regex_str(0));
    let re = Regex::new(&format!("^{}$", rules.to_regex_str(0))).unwrap();
    println!("Part 1: {} matched", words.iter().filter(|l| re.is_match(l)).count());

    {   // Patch rule 8
        let end = rules.rules.len();
        rules.rules.push(Some(Node::Match1(42)));
        rules.rules.push(Some(Node::Match2(42, 8)));
        rules.rules[8] = Some(Node::Alt(end, end+1));
    }

    {   // Patch rule 11
        let end = rules.rules.len();
        rules.rules.push(Some(Node::Match2(42, 31)));
        rules.rules.push(Some(Node::Match3(42, 11, 31)));
        rules.rules[11] = Some(Node::Alt(end, end+1));
    }

//     println!("Part 2: {} matched", words.iter().filter(|l| rules.is_match(l)).count());

    let mut matcher = RuleMatcher::new(&rules);
    let source :Vec<char> = words[0].chars().collect();
    for ch in source[0..5].iter() {
        let len = matcher.read(*ch);
        println!("{} chars read, {} candidate paths", matcher.chars_read, len);
    }

    println!("BAH! Plan B");
    std::process::Command::new("/usr/bin/perl").arg("19.pl").arg(fname).status();
}
