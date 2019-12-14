
extern crate clap;

// Time Start: Sat, 14 Dec 2019 07:21:21 -0500
// Time Finish 1: Sat, 14 Dec 2019 11:24:47 -0500 (4 hours, 3 minutes, 26 seconds)
// Time Finish 2: Sat, 14 Dec 2019 11:37:36 -0500 (12 minutes, 49 seconds)
// Time Total: 4 hours, 16 minutes, 15 seconds

use std::collections::HashMap;
use std::fs;

use clap::{Arg, App};


#[derive(Clone)]
pub struct Recipe {
    input: HashMap<String, u64>,
    output: u64,
}
impl Recipe {
    pub fn new() -> Recipe {
        Recipe {
            input: HashMap::new(),
            output: 0
        }
    }

    pub fn set_output(&mut self, val: u64) { self.output = val }
    pub fn get_output(&self) -> u64 { self.output }
    pub fn insert(&mut self, name: String, val: u64) -> Option<u64> { self.input.insert(name, val) }
    pub fn iter(&mut self) -> std::collections::hash_map::Iter<String, u64> { self.input.iter() }
}


pub struct Factory {
    reactions: HashMap<String, Recipe>,
}
impl Factory {
    pub fn new() -> Factory {
        Factory {
            reactions: HashMap::new(),
        }
    }

    pub fn load(fname: &str) -> Factory {
        let contents = fs::read_to_string(fname)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
        let mut factory = Factory::new();
        for line in contents.trim().split("\n") {
            factory.add_recipe(&line);
        }
        return factory;
    }

    pub fn add_recipe(&mut self, recipe: &str) {
        let mut i1 = recipe.trim().split(" => ");
        let mut r = Recipe::new();
        let ingredients = i1.next().unwrap_or_else(|| panic!("Recipe ingredient parse error in '{}'", recipe));
        let mut output = i1.next().unwrap_or_else(|| panic!("Recipe output parse error in '{}'", recipe)).split(" ");
        for item in ingredients.split(", ") {
            let mut ni = item.split(" ");
            let num = ni.next().unwrap_or_else(|| panic!("Item count parse error in '{}'", item)).parse::<u64>().unwrap_or_else(|err| panic!("Number parse error in '{}': {}", item, err));
            let name = String::from(ni.next().unwrap_or_else(|| panic!("Item name parse error in '{}'", item)));
            r.insert(name, num);
        }

        let num = output.next().unwrap_or_else(|| panic!("Output count parse error in '{}'", recipe)).parse::<u64>().unwrap_or_else(|err| panic!("Number parse error in '{}': {}", recipe, err));
        let name = String::from(output.next().unwrap_or_else(|| panic!("Output name parse error in '{}'", recipe)));
        r.set_output(num);
        if let Some(_more) = output.next() {
            panic!("Failed assumption! a reaction has multiple putputs {}", recipe);
        }
        if self.reactions.contains_key(&name) {
            panic!("Failed assumption! there are two ways to produce {}", name);
        }
        self.reactions.insert(name, r);
    }

    pub fn build(&self, n: u64, item: &String, mut soup: &mut HashMap<String, u64>) -> u64 {
        if item == "ORE" { return n; }

        // Extract what we can from the soup
        let mut todo = n;
        if let Some(available) = soup.get_mut(item) {
            if *available >= todo {
                *available -= todo;
                return 0;
            } else {
                todo -= *available;
                *available = 0;
            }
        }

        // How many batches are needed
        let mut recipe = self.reactions.get(item).unwrap_or_else(|| panic!("Unknown ingredient {}", item)).clone();
        let batch = todo / recipe.get_output() + (if 0 == (todo % recipe.get_output()) { 0 } else { 1 });

        let mut cost = 0;
        // build the sub-ingredients
        for (ingredient, count) in recipe.iter() {
            cost += self.build(batch * *count, ingredient, &mut soup);
        }

        // contribute extras to the soup
        let extra = batch * recipe.get_output() - todo;
        if extra > 0 {
            if let Some(available) = soup.get_mut(item) {
                *available += extra;
            } else {
                soup.insert(item.clone(), extra);
            }
        }
        return cost;
    }

}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 14")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("14.in");

    let factory = Factory::load(&fname);

    let mut soup = HashMap::new();
    let ore1 = factory.build(1, &String::from("FUEL"), &mut soup);
    println!("Step 1: Producing 1 fuel requires {} ORE", ore1);

    let target = 1000000000000_u64;
    let mut a = 1_u64;
    let mut b = target;
    while a < b - 1 {
        let c = (a + b) / 2;
        let mut soup = HashMap::new();
        let ore = factory.build(c, &String::from("FUEL"), &mut soup);
        if ore < target {
            a = c;
        } else {
            b = c;
        }
    }
    println!("Step 2: With {} ore, we can produce {} fuel", target, a);
}
