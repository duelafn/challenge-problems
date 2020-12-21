// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::parse::StrParser;

use clap::{Arg, App};


fn load(fname: &str) -> Result<(Vec<HashSet<String>>, HashMap<String, HashSet<String>>), String> {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let mut recipes = Vec::new();
    let mut allergens :HashMap<String, HashSet<String>> = HashMap::new();

    for line in contents.lines() {
        let mut p = StrParser::new(line);
        let mut set = HashSet::new();
        while p.skip_char('(') == 0 {
            set.insert(p.extract_alnum()?);
        }
        p.expect_str("contains")?;
        while p.skip_char(')') == 0 {
            let aller = p.extract_alnum()?;
            allergens.entry(aller)
                .and_modify(     |s| { s.retain(|i| set.contains(i)); } )
                .or_insert_with( ||  { set.clone() } );
            p.skip_char(',');
        }
        recipes.push(set);
    }

    Ok((recipes, allergens))
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 21 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("21.in");

    let (recipes, mut allergens) = load(fname).unwrap();
    let mut bad_foods :HashSet<String> = HashSet::new();
    loop {
        let mut todo = false;
        for v in allergens.values_mut() {
            if v.len() == 1 {
                bad_foods.insert(v.iter().next().unwrap().to_owned());
            } else {
                v.retain(|e| !bad_foods.contains(e));
                todo = true;
            }
        }
        if !todo { break; }
    }

    let mut count = 0;
    for rec in recipes.iter() {
        count += rec.difference(&bad_foods).count();
    }
    println!("Part 1: {} instances of good ingredients", count);

    let mut list = allergens.iter().map(|(k,v)| (k.to_owned(), v.iter().next().unwrap().to_owned())).collect::<Vec<(String,String)>>();
    list.sort();
    println!("Part 2: Dangerous: {}", list.iter().map(|(_,v)| v.clone()).collect::<Vec<String>>().join(","));
}
