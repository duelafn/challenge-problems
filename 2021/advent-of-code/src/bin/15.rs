// SPDX-License-Identifier: MIT

use petgraph::Graph;
use petgraph::algo::astar;
use petgraph::prelude::*;

use aoc::map::neighbors4;

type Map = Vec<Vec<NodeIndex>>;

fn load(fname: &str, scale: u32) -> (Map, Graph<u32, ()>) {
    let contents = std::fs::read_to_string(fname).unwrap();
    let mut map = Vec::new();
    let mut graph = DiGraph::new();
    for i in 0..scale {
        for line in contents.lines() {
            let mut row = Vec::new();
            for j in 0..scale {
                for ch in line.chars() {
                    let mut weight = ch.to_digit(10).unwrap() + i + j;
                    if weight > 9 { weight -= 9; }
                    row.push(graph.add_node(weight));
                }
            }
            map.push(row);
        }
    }

    for (y, line) in map.iter().enumerate() {
        for (x, a) in line.iter().enumerate() {
            for (_, _, b) in neighbors4(&map, x, y) {
                graph.add_edge(*a, *b, ());
            }
        }
    }

    return (map, graph);
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("15.in"));
    let (map, graph) = load(&fname, 1);
    let start = map[0][0];
    let goal  = map[map.len()-1][map[0].len()-1];

    let res = astar(&graph, start, |n| n == goal, |e| graph[e.target()], |_| 1).unwrap();
    println!("Part 1: {}", res.0);

    let (map, graph) = load(&fname, 5);
    let start = map[0][0];
    let goal  = map[map.len()-1][map[0].len()-1];

    let res = astar(&graph, start, |n| n == goal, |e| graph[e.target()], |_| 1).unwrap();
    println!("Part 2: {}", res.0);
}
