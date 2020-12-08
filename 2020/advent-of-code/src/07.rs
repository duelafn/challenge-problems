// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;

use clap::{Arg, App};



#[derive(Debug, Clone)]
pub struct Rules {
    bags: HashMap<String, Bag>,
}
impl Rules {
    pub fn new() -> Rules {
        Rules { bags: HashMap::new() }
    }

    pub fn get_or_create(&mut self, name: &str) -> &mut Bag {
        self.bags.entry(name.to_owned()).or_insert_with(|| Bag::new(name))
    }

    pub fn add(&mut self, bag: Bag) {
        let bag = self.bags.entry(bag.name.to_owned()).and_modify(|b| b.update_from(&bag)).or_insert(bag);
        // get_or_create below might rehash, so we have to clone the children and bag name
        let children :Vec<String> = bag.contains.keys().cloned().collect();
        let name = bag.name.clone();
        for chld in children {
            self.get_or_create(&chld).add_parent(&name);
        }
    }

    pub fn count_descendants(&mut self, name: &str) -> usize {
        if !self.bags.contains_key(name) { return 0; }
        let bag = self.bags.get(name).unwrap();
        if let Some(n) = bag.num_descendants { return n; }
        // This clone is much less satisfying - as far as I can tell, is only necessary to satisfy borrow checker
        let children :Vec<(String, usize)> = bag.contains.iter().map(|(a,b)| (a.to_string(), *b)).collect();

        let mut descendants = 0;
        for (chld, num) in children {
            descendants += num * (1 + self.count_descendants(chld.as_ref()));
        }

        if let Some(bag) = self.bags.get_mut(name) {
            bag.num_descendants = Some(descendants);
        }
        return descendants;
    }
}
impl std::convert::TryFrom<&str> for Rules {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut rules = Rules::new();
        for (lineno, chunk) in src.lines().enumerate() {
            let bag = Bag::try_from(chunk).or_else(|err| Err(format!("Parse error at '{}' in record {}: {}", chunk, lineno+1, err)))?;
            rules.add(bag);
        }
        return Ok(rules);
    }
}
impl std::ops::Deref for Rules {
    type Target = HashMap<String, Bag>;
    fn deref(&self) -> &Self::Target { &self.bags }
}


#[derive(Debug, Clone)]
pub struct Bag {
    pub name: String,
    pub num_descendants: Option<usize>,
    pub contains: HashMap<String, usize>,
    pub contained_in: HashSet<String>,
}
impl Bag {
    pub fn new(name: &str) -> Bag {
        Bag {
            name: name.to_string(),
            num_descendants: None,
            contains: HashMap::new(),
            contained_in: HashSet::new(),
        }
    }

    pub fn add_parent(&mut self, name: &str) {
        self.contained_in.insert(name.to_owned());
    }

    pub fn update_from(&mut self, other: &Bag) {
        for (k, v) in other.contains.iter() {
            self.contains.insert(k.to_owned(), v.to_owned());
        }
        for k in other.contained_in.iter() {
            self.contained_in.insert(k.to_owned());
        }
    }
}
impl std::convert::TryFrom<&str> for Bag {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut name_contents = src.split(" bags contain ");
        if let Some(name) = name_contents.next() {
            if let Some(contents) = name_contents.next() {
                let mut hash = HashMap::new();
                for item in contents.split(", ") {
                    if item != "no other bags." {
                        let word :Vec<&str> = item.split(" ").collect();
                        if 4 == word.len() {
                            hash.insert(
                                format!("{} {}", word[1], word[2]),
                                word[0].parse().or_else(|err| Err(format!("Can not parse number '{}' from bag '{}': {}", word[0], name, err)))?
                            );
                        }
                        else { return Err(format!("Error parsing item '{}' in bag '{}'", item, name)); }
                    }
                }
                if hash.is_empty() {
                    return Ok(Bag { name: name.to_string(), num_descendants: Some(0), contains: hash, contained_in: HashSet::new() });
                } else {
                    return Ok(Bag { name: name.to_string(), num_descendants: None, contains: hash, contained_in: HashSet::new() });
                }
            }
            else { return Err(format!("Can not find bag contents in '{}'", src)); }
        } else { return Err(format!("Can not find bag name in '{}'", src)); }
    }
}
impl std::ops::Deref for Bag {
    type Target = HashMap<String, usize>;
    fn deref(&self) -> &Self::Target { &self.contains }
}


// Vec of references, lifetime must match the rule set.
pub fn ancestors<'a>(rules: &'a HashMap<String, Bag>, bag: &str) -> Vec<&'a Bag> {
    let mut rv = Vec::new();
    let mut todo = vec![bag.to_string()];
    let mut seen = HashSet::new();
    while let Some(name) = todo.pop() {
        if !seen.contains(&name) {
            let this = rules.get(&name).unwrap();
            if name != bag { rv.push(this); }
            let mut parents = this.contained_in.iter().cloned().collect();
            todo.append(&mut parents);
            seen.insert(name);
        }
    }
    return rv;
}


fn records(fname: &str) -> Rules {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
    Rules::try_from(contents.as_ref()).unwrap_or_else(|err| panic!("Parse error at in {}: {}", fname, err))
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 07 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("07.in");

    let mut rules = records(fname);
    let contains_gold = ancestors(&rules, "shiny gold");
    println!("Part 1: {} bags eventually can contain a shiny gold bag", contains_gold.len());

    println!("Part 2: a shiny gold bag has {} descendants", rules.count_descendants("shiny gold"));
}
