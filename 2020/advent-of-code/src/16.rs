// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::convert::TryFrom;

use clap::{Arg, App};


#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    intervals: Vec<(u16, u16)>,
}
impl Field {
    pub fn is_valid(&self, val: u16) -> bool {
        self.intervals.iter().any(|(a, b)| *a <= val && val <= *b)
    }
}

impl std::convert::TryFrom<&str> for Field {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut name_vals = src.split(": ");
        let name = name_vals.next().ok_or(String::from("Parse error"))?.to_string();
        let validators = name_vals.next().ok_or(String::from("Parse error"))?;
        let mut intervals = Vec::new();
        for v in validators.split(" or ") {
            let ab: Vec<Result<u16,_>> = v.split('-').map(|n| n.parse::<u16>()).collect();
            if ab.len() != 2 { return Err(String::from("Parse error")) }
            intervals.push( (*ab[0].as_ref().map_err(|e| e.to_string())?, *ab[1].as_ref().map_err(|e| e.to_string())?) );
        }
        Ok(Field { name, intervals })
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.intervals)
    }
}

// This was nice, shame we didn't need it :)
//
// Just try doing THIS in perl or python, Field hashes in the same way as
// its string name. This allows tricks with HashSet.get(str) to return the
// Field... a dictionary where keys take no space. Though, sadly, messes with Eq.
impl std::hash::Hash for Field {
    #[inline]
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher { self.name.hash(state); }
}
impl PartialEq for Field {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
}
impl Eq for Field { }
// Note: .borrow() is used for equality checking when the HashSet iterates
// over items in the hash bucket. -- HashSet.get(b) checks b == x.borrow()
// for all x in the bucket hash(b).
impl std::borrow::Borrow<str> for Field {
    #[inline]
    fn borrow(&self) -> &str { &self.name.as_str() }
}


#[derive(Debug, Clone)]
pub struct IndexSuperposition<'a> {
    field: &'a Field,
    candidates: usize,
    indices: Vec<bool>,
    index: Option<usize>,
}
impl<'a> IndexSuperposition<'a> {
    pub fn new(field: &'a Field, num_fields: usize) -> Self {
        IndexSuperposition { field, index: None, candidates: num_fields, indices: vec![true; num_fields] }
    }

    #[inline]
    pub fn update(&mut self, idx: usize, val: u16) {
        if self.candidates > 1 && self.indices[idx] { // CRASH!
            if !self.field.is_valid(val) {
                self.indices[idx] = false;
                self.candidates -= 1;
                if self.candidates == 1 { self.resolve(); }
            }
        }
    }

    #[inline]
    pub fn remove(&mut self, idx: usize) -> bool {
        if self.indices[idx] { // CRASH!
            self.indices[idx] = false;
            self.candidates -= 1;
            if self.candidates == 1 { self.resolve(); }
            return true;
        }
        return false;
    }

    #[inline]
    pub fn resolve(&mut self) {
        if 1 == self.candidates && self.index.is_none() {
            self.index = self.index();
        }
    }

    pub fn index(&self) -> Option<usize> {
        if let Some(idx) = self.index { return Some(idx) }
        if 1 == self.candidates {
            for i in 0..self.indices.len() {
                if self.indices[i] {
                    return Some(i);
                }
            }
        }
        else { return None; }
        unreachable!("Oops");
    }

    #[inline]
    pub fn field(&self) -> &'a Field { self.field }
    #[inline]
    pub fn name(&self) -> &'a String { &self.field.name }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    fields: Vec<u16>,
}
impl std::convert::TryFrom<&str> for Ticket {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Ok(Ticket {
            fields: src.split(",").map(|n| n.parse::<u16>().map_err(|e| e.to_string())).collect::<Result<Vec<u16>,String>>()?
        })
    }
}
impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.fields)
    }
}
impl std::ops::Deref for Ticket {
    type Target = Vec<u16>;
    fn deref(&self) -> &Self::Target { &self.fields }
}


fn load(fname: &str) -> (Ticket, HashSet<Field>, Vec<Ticket>) {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let mut fields = HashSet::new();
    let mut tickets = Vec::new();
    let mut my_ticket = None;
    let mut block = 0;
    for (lineno, chunk) in contents.lines().enumerate() {
        if chunk.is_empty() { block += 1; continue; }
        if block == 0 {
            let f = Field::try_from(chunk).unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err));
            fields.insert(f);
        }

        if block == 1 && !chunk.starts_with("your") {
            my_ticket = Some( Ticket::try_from(chunk).unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err)) );
        }

        if block == 2 && !chunk.starts_with("nearby") {
            tickets.push( Ticket::try_from(chunk).unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", chunk, fname, lineno+1, err)) );
        }
    }

    let my_ticket = my_ticket.unwrap_or_else(|| panic!("Missing my ticket!"));
    if my_ticket.len() != fields.len() { panic!("Ticket length is not field length!"); }
    return (my_ticket, fields, tickets);
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 16 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("16.in");

    let (my_ticket, fields, tickets) = load(fname);
    let mut err = 0;
    let mut valid_tickets = Vec::new();
    for t in tickets {
        let mut bad = false;
        for v in t.iter() {
            if !fields.iter().any(|f| f.is_valid(*v)) {
                err += v;
                bad = true;
            }
        }
        if !bad { valid_tickets.push(t); }
    }
    println!("Part 1: error rate {}", err);

    let mut indices :Vec<IndexSuperposition> = fields.iter().map(|f| IndexSuperposition::new(f, my_ticket.len())).collect();
    // Our ticket is valid:
    for (i, v) in my_ticket.iter().enumerate() {
        for idx in indices.iter_mut() { idx.update(i, *v); }
    }
    // Check valid tickets:
    for t in valid_tickets.iter() {
        for (i, v) in t.iter().enumerate() {
            for idx in indices.iter_mut() {
                idx.update(i, *v);
            }
        }
    }
    // Time for deduction:
    let mut repeat = true;
    while repeat {
        repeat = false;
        let known :Vec<usize> = indices.iter().filter_map(|i| i.index()).collect();
        for i in known {
            for idx in indices.iter_mut() {
                if idx.index().is_none() {
                    if idx.remove(i) { repeat = true; }
                }
            }
        }
    }

    let mut departures = 1_u64;
    for idx in indices {
        if let Some(i) = idx.index() {
            println!(".. {}: {}", idx.name(), my_ticket[i]);
            if idx.name().starts_with("departure") {
                departures *= my_ticket[i] as u64;
            }
        } else {
            println!("{}: Unknown column, {} candidates", idx.name(), idx.candidates);
        }
    }
    println!("Part 2: multiplied: {}", departures);
}
