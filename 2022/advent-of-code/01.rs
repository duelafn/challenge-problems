
struct Load { items: Vec<i64> }
impl Load {
    fn parse(content: &str) -> Load {
        Load { items: content.split_ascii_whitespace().map(|v| v.parse().unwrap()).collect() }
    }
    fn weight(&self) -> i64 { self.items.iter().sum() }
}

struct Expedition { elves: Vec<Load> }
impl Expedition {
    fn parse(content: &str) -> Expedition {
        Expedition { elves: content.split("\n\n").map(|v| Load::parse(v)).collect() }
    }
}

fn parse(fname: &str) -> Expedition {
    Expedition::parse(&std::fs::read_to_string(fname).unwrap())
}

fn main() {
    let fname = std::env::args().nth(1).unwrap_or_else(|| String::from("01.in"));
    let mut exp = parse(&fname);
    exp.elves.sort_by_cached_key(|load| -load.weight());
    println!("Part 1: {}", exp.elves[0].weight());
    println!("Part 2: {}", exp.elves[0..3].iter().map(|e| e.weight()).sum::<i64>());
}
