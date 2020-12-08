// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::convert::TryFrom;

use petgraph::prelude::{Graph,NodeIndex,Outgoing,Dfs};
use petgraph::visit::Reversed;

use clap::{Arg, App};


#[derive(Debug, Clone)]
pub struct Node {
    num_descendants: Option<u32>,
}
impl Node {
    pub fn new() -> Node { Node { num_descendants: None } }
}

#[derive(Debug, Clone)]
pub struct Rules {
    bags: HashMap<String, NodeIndex>,
    graph: Graph<Node, u32>
}
impl Rules {
    pub fn new() -> Rules {
        Rules { bags: HashMap::new(), graph: Graph::new() }
    }

    pub fn get_or_create(&mut self, name: &str) -> NodeIndex {
        let idx = self.bags.get(name);
        match idx {
            Some(n) => { return *n; },
            None    => {
                let new_node = self.graph.add_node(Node::new());
                self.bags.insert(name.to_owned(), new_node);
                return new_node;
            }
        }
    }

    pub fn count_descendants(&mut self, name: &str) -> u32 {
        if !self.bags.contains_key(name) { return 0; }
        return self.count_descendants_id(*self.bags.get(name).unwrap());
    }
    pub fn count_descendants_id(&mut self, id: NodeIndex) -> u32 {
        if let Some(Node { num_descendants: Some(n), .. }) = self.graph.node_weight(id) {
            return *n;
        }

        let mut descendants = 0;
        let mut edges = self.graph.neighbors_directed(id, Outgoing).detach();
        while let Some((edgeid, nodeid)) = edges.next(&self.graph) {
            descendants += self.graph[edgeid] * (1 + self.count_descendants_id(nodeid));
        }

        self.graph[id].num_descendants = Some(descendants);
        return descendants;
    }

    pub fn count_ancestors(&mut self, name: &str) -> u32 {
        if !self.bags.contains_key(name) { return 0; }
        return self.count_ancestors_id(*self.bags.get(name).unwrap());
    }
    pub fn count_ancestors_id(&mut self, id: NodeIndex) -> u32 {
        let graph = Reversed(&self.graph);
        let mut dfs = Dfs::new(&graph, id);
        let mut ancestors = 0;
        while let Some(_) = dfs.next(&graph) {
            ancestors += 1;
        }
        return ancestors - 1;// Counts the initial node
    }

    fn parse_line(src: &str) -> Result<(String, Vec<(u32, String)>), String> {
        let mut name_contents = src.split(" bags contain ");
        if let Some(name) = name_contents.next() {
            if let Some(contents) = name_contents.next() {
                let mut vec = Vec::new();
                for item in contents.split(", ") {
                    if item != "no other bags." {
                        let word :Vec<&str> = item.split(" ").collect();
                        if 4 == word.len() {
                            vec.push((
                                word[0].parse().or_else(|err| Err(format!("Can not parse number '{}' from bag '{}': {}", word[0], name, err)))?,
                                format!("{} {}", word[1], word[2]),
                            ));
                        }
                        else { return Err(format!("Error parsing item '{}' in bag '{}'", item, name)); }
                    }
                }
                return Ok((name.to_string(), vec));
            }
            else { return Err(format!("Can not find bag contents in '{}'", src)); }
        } else { return Err(format!("Can not find bag name in '{}'", src)); }
    }
}
impl std::convert::TryFrom<&str> for Rules {
    type Error = String;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut rules = Rules::new();
        for (lineno, chunk) in src.lines().enumerate() {
            let (name, contents) = Rules::parse_line(chunk).or_else(|err| Err(format!("Parse error at '{}' in record {}: {}", chunk, lineno+1, err)))?;
            let node = rules.get_or_create(&name);
            for (num, child) in contents {
                let child_id = rules.get_or_create(&child);
                rules.graph.add_edge(node, child_id, num);
            }
        }
        return Ok(rules);
    }
}


fn records(fname: &str) -> Rules {
    let contents = std::fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    Rules::try_from(contents.as_ref()).unwrap_or_else(|err| panic!("Parse error in {}: {}", fname, err))
}

fn main() {
    let matches = App::new("Advent of code 2020, Day 07 Solution")
        .arg(Arg::with_name("FILE").help("Input file to process").index(1))
        .get_matches();
    let fname = matches.value_of("FILE").unwrap_or("07.in");

    let mut rules = records(fname);
    println!("Part 1: {} bags eventually can contain a shiny gold bag", rules.count_ancestors("shiny gold"));
    println!("Part 2: a shiny gold bag has {} descendants", rules.count_descendants("shiny gold"));
}
