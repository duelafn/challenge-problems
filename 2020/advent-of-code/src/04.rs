// SPDX-License-Identifier: MIT

#[macro_use]
extern crate lazy_static;

use std::convert::TryFrom;

use regex::Regex;
use clap::{Arg, App};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}
impl Passport {
    pub fn new() -> Passport {
        Passport { byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None, cid: None }
    }

    pub fn is_almost_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_more_valid(&self) -> bool {
        if !self.is_almost_valid() { return false; }

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if let Ok(year) = self.byr.as_ref().unwrap().parse::<u16>() {
            if year < 1920 || year > 2002 { return false; }
        } else { return false; }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if let Ok(year) = self.iyr.as_ref().unwrap().parse::<u16>() {
            if year < 2010 || year > 2020 { return false; }
        } else { return false; }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if let Ok(year) = self.eyr.as_ref().unwrap().parse::<u16>() {
            if year < 2020 || year > 2030 { return false; }
        } else { return false; }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        lazy_static! { static ref RE_HGT: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap(); }
        if let Some(cap) = RE_HGT.captures(self.hgt.as_ref().unwrap().as_ref()) {
            if let Some(num) = cap.get(1) {
                let ht = num.as_str().parse::<u16>().unwrap_or(0);
                if let Some(unit) = cap.get(2) {
                    match unit.as_str() {
                        "in" => { if ht < 59  || ht > 76  { return false; } },
                        "cm" => { if ht < 150 || ht > 193 { return false; } },
	                    _ => { return false; },
                    }
                }
            }
        } else { return false; }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        lazy_static! { static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap(); }
        if !RE_HCL.is_match(self.hcl.as_ref().unwrap().as_ref()) { return false; }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        lazy_static! { static ref RE_ECL: Regex = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap(); }
        if !RE_ECL.is_match(self.ecl.as_ref().unwrap().as_ref()) { return false; }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        lazy_static! { static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap(); }
        if !RE_PID.is_match(self.pid.as_ref().unwrap().as_ref()) { return false; }

        return true;
    }
}

impl std::convert::TryFrom<&str> for Passport {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut pass = Passport::new();
        for field in src.split_ascii_whitespace() {
            match &field[0..3] {
                "byr" => { pass.byr = Some(String::from(&field[4..])); },
                "iyr" => { pass.iyr = Some(String::from(&field[4..])); },
                "eyr" => { pass.eyr = Some(String::from(&field[4..])); },
                "hgt" => { pass.hgt = Some(String::from(&field[4..])); },
                "hcl" => { pass.hcl = Some(String::from(&field[4..])); },
                "ecl" => { pass.ecl = Some(String::from(&field[4..])); },
                "pid" => { pass.pid = Some(String::from(&field[4..])); },
                "cid" => { pass.cid = Some(String::from(&field[4..])); },
                other => { return Err(format!("Invalid field: '{}'", other)); },
            }
        }
        return Ok(pass);
    }
}



fn records(fname: &str) -> Vec<Passport> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    return contents.split("\n\n").enumerate().map(
        |(lineno, record)| {
            Passport::try_from(record).unwrap_or_else(|err| panic!("Parse error at '{}' in {} in record {}: {}", record, fname, lineno+1, err))
        }
    ).collect();
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 04 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("04.in");

    let records = records(fname);
    let mut valid = 0;
    for r in records.iter() {
        if r.is_almost_valid() { valid += 1; }
    }
    println!("Part 1: {} (almost) valid records", valid);

    let mut valid = 0;
    for r in records {
        if r.is_more_valid() { valid += 1; }
    }
    println!("Part 2: {} (more) valid records", valid);
}
