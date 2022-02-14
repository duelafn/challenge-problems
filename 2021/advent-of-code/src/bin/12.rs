// SPDX-License-Identifier: MIT

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Todo<'a> {
    dup: bool,
    chain: Vec<&'a str>,
}
impl<'a> Todo<'a> {
    fn new(_: &'a str) -> Todo { Self { dup: false, chain: vec!["start"] } }
    fn last(&self) -> &str { *self.chain.last().unwrap() }
    fn push(&mut self, val: &'a str) { self.chain.push(val); }
    fn contains(&self, val: &str) -> bool { self.chain.contains(&val) }
}


fn load(contents: &str) -> HashMap<&str, (bool, Vec<&str>)> {
    let mut map = HashMap::new();
    for line in contents.lines() {
        let kv = line.split('-').collect::<Vec<&str>>();
        let adj = map.entry(kv[0]).or_insert_with(|| (char::is_uppercase(kv[0].chars().next().unwrap()), Vec::new()));
        adj.1.push(kv[1]);
        let adj = map.entry(kv[1]).or_insert_with(|| (char::is_uppercase(kv[1].chars().next().unwrap()), Vec::new()));
        adj.1.push(kv[0]);
    }
    return map;
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("12.in"));
    let contents = std::fs::read_to_string(fname).unwrap();
    let adj = load(&contents);

    let mut todo = vec![vec!["start"]];
    let mut paths = Vec::new();
    // Brute searching probably good enough:
    while let Some(path) = todo.pop() {
        let last = path.last().unwrap();
        for next in adj.get(last).unwrap().1.iter() {
            if *next == "end" {
                let mut newpath = path.clone();
                newpath.push(*next);
                paths.push(newpath);
            } else {
                let revisit = adj.get(next).unwrap().0;
                if revisit || !path.contains(next) {
                    let mut newpath = path.clone();
                    newpath.push(*next);
                    todo.push(newpath);
                }
            }
        }
    }
    println!("Part 1, {} paths", paths.len());

    let mut todo = vec![Todo::new(&contents)];
    let mut paths = Vec::new();
    // Brute searching may be good enough:
    while let Some(path) = todo.pop() {
        let last = path.last();
        for next in adj.get(last).unwrap().1.iter() {
            match *next {
                "start" => { },
                "end" => {
                    let mut newpath = path.clone();
                    newpath.push(*next);
                    paths.push(newpath);
                },
                next => {
                    let revisit = adj.get(&next).unwrap().0;
                    let contains = if revisit { false } else { path.contains(&next) };
                    if revisit || !contains || !path.dup {
                        let mut newpath = path.clone();
                        if contains { newpath.dup = true; }
                        newpath.push(next);
                        todo.push(newpath);
                    }
                },
            }
        }
    }
    println!("Part 2, {} paths", paths.len());
    // ... yep, first shot!
}
