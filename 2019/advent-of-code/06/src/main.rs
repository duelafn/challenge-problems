
extern crate clap;

// Time Start: Fri, 06 Dec 2019 17:13:45 -0500
// Time Finish 1: Sat, 07 Dec 2019 00:24:28 -0500 (7 hours, 10 minutes, 43 seconds)
// Time Finish 2: Sat, 07 Dec 2019 01:22:54 -0500 (58 minutes, 26 seconds)
// Time Total: 8 hours, 9 minutes, 9 seconds

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::io::prelude::*;

use clap::{Arg, App};


pub enum EdgeError {
    DuplicateParent,
    MissingSource,
    MissingDestination,
}

struct Node<K> { // Yes, private
    parent: Option<K>,
    children: Vec<K>,
}
impl<K> Node<K> {
    fn new() -> Node<K> {
        Node { parent: None, children: Vec::new() }
    }
}

pub struct KeyTree<K, D> {
    nodes: HashMap<K, Node<K>>,
    data: HashMap<K, D>,
}
impl<K, D> KeyTree<K, D>
    where K: Eq + Hash + Clone
{
    pub fn new() -> KeyTree<K, D> {
        KeyTree::<K, D> {
            nodes: HashMap::new(),
            data: HashMap::new(),
        }
    }

    pub fn contains_node(&self, key: &K) -> bool { self.nodes.contains_key(&key) }
    pub fn set_node(&mut self, key: K, data: D) {
        if let Some(stored) = self.data.get_mut(&key) {
            *stored = data;
        } else {
            if !self.nodes.contains_key(&key) {
                self.nodes.insert(key.clone(), Node::new());
            }
            self.data.insert(key, data);
        }
    }

    pub fn add_edge(&mut self, a: &K, b: &K) -> Result<(), EdgeError> {
        if ! self.nodes.contains_key(&a) { return Err(EdgeError::MissingSource); }
        if ! self.nodes.contains_key(&b) { return Err(EdgeError::MissingDestination); }

        if let Some(node) = self.nodes.get_mut(&b) {
            if node.parent.is_some() {
                return Err(EdgeError::DuplicateParent);
            }
            node.parent = Some(a.clone());
        }

        if let Some(node) = self.nodes.get_mut(&a) {
            node.children.push(b.clone());
        }

        return Ok(());
    }


    pub fn get_mut(&mut self, key: &K) -> Option<&mut D> { self.data.get_mut(&key) }
    pub fn get(&self, key: &K)      -> Option<&D>     { self.data.get(&key) }
    pub fn parent(&self, key: &K)   -> Option<K>      { match self.nodes.get(&key) { Some(node) => node.parent.clone(), _ => None } }
    pub fn children(&self, key: &K) -> Vec<K>         { match self.nodes.get(&key) { Some(node) => node.children.clone(), _ => Vec::new() } }


    // Sigh, when you find a stackoverflow answer that suggests that it
    // might be easier to create your own iterator object than to figure
    // out the return type magic: https://stackoverflow.com/questions/34459976/34462306#34462306
    // pub type NodeMap<'a, K> = FilterMap<std::collections::hash_map::Iter<'a, K, Node<K>>, FnMut((K, Node<K>)) -> Option<K>>;
    pub fn leaves(&self) -> Vec<K> { self.nodes.iter().filter_map(|(key, node): (&K, &Node<K>)| if node.children.is_empty() { Some(key.clone()) } else { None }).collect() }
    pub fn roots(&self)  -> Vec<K> { self.nodes.iter().filter_map(|(key, node): (&K, &Node<K>)| match &node.parent { Some(_name) => None, None => Some(key.clone()) }).collect() }


    pub fn find_path(&self, a: &K, b: &K) -> Option<Vec<K>>
        where K: std::fmt::Debug
    {
        let mut path = Vec::new();
        let mut seen = HashSet::new();
        path.push(a.clone());
        seen.insert(a.clone());
        let A = match self.nodes.get(&a) { Some(node) => node, _ => panic!("No such node") };
        return self._find_path(&A, &b, self.nodes.len(), &path, &seen);
    }
    fn _find_path(&self, A: &Node<K>, b: &K, limit: usize, path: &Vec<K>, seen: &HashSet<K>) -> Option<Vec<K>>
        where K: std::fmt::Debug
    {
        if path.len() > limit { return None; }
        let mut lim = limit;
        let mut best = None;

        if let Some(parent) = &A.parent {
            if !seen.contains(parent) {
                let mut p = path.clone();
                p.push(parent.clone());
                if *parent == *b { return Some(p); }
                let mut s = seen.clone();
                s.insert(parent.clone());
                let Parent = self.nodes.get(parent).unwrap();
                best = self._find_path(&Parent, &b, lim, &p, &s);
                if let Some(v) = &best {
                    if v.len() < lim { lim = v.len(); }
                }
            }
        }

        for chld in &A.children {
            if !seen.contains(chld) {
                let mut p = path.clone();
                p.push(chld.clone());
                if *chld == *b { return Some(p); }
                let mut s = seen.clone();
                s.insert(chld.clone());
                let Chld = self.nodes.get(chld).unwrap();
                let rv = self._find_path(&Chld, &b, lim, &p, &s);
                if let Some(v) = rv {
                    if v.len() < lim { lim = v.len(); }
                    if let Some(bst) = &best {
                        if v.len() < bst.len() {
                            best = Some(v);
                        }
                    } else {
                        best = Some(v);
                    }
                }
            }
        }

        return best;
    }


    // NOTE: This is guaranteed to return a top-down vector. A node will be
    // visited before any of its children.
    pub fn climb(&self) -> Vec<K> {
        // Should make an iter, I just don't have the energy for that
        let mut todo = Vec::new();
        let mut seen = HashSet::new();
        for key in self.roots() {
            self._iter_dep(&key, &mut todo, &mut seen);
            todo.push(key); // Don't need to add roots to seen
        }
        return todo;
    }
    fn _iter_dep(&self, key: &K, mut todo: &mut Vec<K>, mut seen: &mut HashSet<K>) {
        for chld in self.children(key) {
            if !seen.contains(&chld) {
                self._iter_dep(&chld, &mut todo, &mut seen);
                seen.insert(chld.clone());
                todo.push(chld);
            }
        }
    }

}


struct D6Payload {
    pub orbits: u32,
}
impl D6Payload {
    fn new() -> D6Payload {
        D6Payload { orbits: 0 }
    }
}


fn read_tree(fname: &String) -> KeyTree<String, D6Payload> {
    let file = File::open(fname).unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
    let reader = BufReader::new(file);
    let name = fname.clone();
    let mut lineno :u32 = 0;

    let mut tree = KeyTree::<String, D6Payload>::new();

    for l in reader.lines() {
        lineno += 1;
        let line = l.unwrap_or_else(|err| panic!("Error reading {}, line {}: {}", name, lineno, err));
        let mut iter = line.trim().split(')');
        if let Some(a) = iter.next() {
            if let Some(b) = iter.next() {
                let (a, b) = (String::from(a), String::from(b));
                let add_a = !tree.contains_node(&a);
                let add_b = !tree.contains_node(&b);
                if add_a { tree.set_node(a.clone(), D6Payload::new()) }
                if add_b { tree.set_node(b.clone(), D6Payload::new()) }
                if let Err(err) = tree.add_edge(&a, &b) {
                    panic!("Error adding edge {} -> {}: {}", a, b, match err {
                        EdgeError::DuplicateParent    => format!("child ({}) already has a parent (line {})", b, lineno),
                        EdgeError::MissingSource      => format!("parent ({}) missing from tree (line {})", a, lineno),
                        EdgeError::MissingDestination => format!("child ({}) missing from tree (line {})", b, lineno),
                    })
                }
                continue;
            }
        }
        panic!("Error parsing {}, line {}: {}", name, lineno, line);
    }

    return tree;
}


fn main() {
    let matches = App::new("Advent of Code 2019, Day 06")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("06.in"));

    let mut tree = read_tree(&fname);

    let mut total_orbits = 0;
    for node in tree.climb() {
        let mut count = 0;
        for chld in tree.children(&node) {
            if let Some(payload) = tree.get(&chld) {
                count += 1 + payload.orbits;
            } else {
                panic!("Node {} is missing its payload", chld);
            }
        }

        if let Some(this) = tree.get_mut(&node) {
            this.orbits = count;
            total_orbits += count;
        }
    }

    println!("Total orbits: {}", total_orbits);

    let path = tree.find_path(&String::from("YOU"), &String::from("SAN"));
    if let Some(p) = path {
        println!("Path to SAN: {}", p.len() - 3);
    } else {
        println!("No path to SANTA!");
    }
}
