// SPDX-License-Identifier: MIT

use std::io::BufRead;

fn find_indices(v: &Vec<i32>, a: i32, b: i32) -> (usize, usize) {
    let mut start = 0;
    for (i, &n) in v.iter().enumerate() {
        if n == a { start = i }
        if n == b { return (start, i); }
    }
    unreachable!("Bummer");
}

struct Reactor {
    reactor: Vec<Vec<Vec<bool>>>,
    x: Vec<i32>,
    y: Vec<i32>,
    z: Vec<i32>,
}

impl Reactor {
    fn init(blocks: &Vec<Region>) -> Reactor {
        let mut x = vec![-50, 51];
        let mut y = vec![-50, 51];
        let mut z = vec![-50, 51];
        for r in blocks {
            x.push(r.x.0); x.push(r.x.1);
            y.push(r.y.0); y.push(r.y.1);
            z.push(r.z.0); z.push(r.z.1);
        }
        x.sort();  y.sort();  z.sort();
        x.dedup(); y.dedup(); z.dedup();
        let reactor = (0..x.len()).map(|_| (0..y.len()).map(|_| vec![false; z.len()]).collect()).collect();
        Reactor { reactor, x, y, z }
    }

    fn apply(&mut self, step: &Region) {
        let rx = find_indices(&self.x, step.x.0, step.x.1);
        let ry = find_indices(&self.y, step.y.0, step.y.1);
        let rz = find_indices(&self.z, step.z.0, step.z.1);
        for x in rx.0..rx.1 {
            for y in ry.0..ry.1 {
                for z in rz.0..rz.1 {
                    self.reactor[x][y][z] = step.state;
                }
            }
        }
    }

    fn count(&self) -> u64 {
        let mut n = 0;
        for (i, plane) in self.reactor.iter().enumerate() {
            for (j, row) in plane.iter().enumerate() {
                for (k, val) in row.iter().enumerate() {
                    if *val {
                        n += ((self.x[i+1] - self.x[i]) as u64)
                           * ((self.y[j+1] - self.y[j]) as u64)
                           * ((self.z[k+1] - self.z[k]) as u64);
                    }
                }
            }
        }
        return n;
    }

    fn count_inner(&self) -> u64 {
        let mut n = 0;
        for (i, plane) in self.reactor.iter().enumerate() {
            if self.x[i] < -50 { continue; }
            if self.x[i] > 50 { break; }
            for (j, row) in plane.iter().enumerate() {
                if self.y[j] < -50 { continue; }
                if self.y[j] > 50 { break; }
                for (k, val) in row.iter().enumerate() {
                    if self.z[k] < -50 { continue; }
                    if self.z[k] > 50 { break; }
                    if *val {
                        n += ((self.x[i+1] - self.x[i]) as u64)
                           * ((self.y[j+1] - self.y[j]) as u64)
                           * ((self.z[k+1] - self.z[k]) as u64);
                    }
                }
            }
        }
        return n;
    }
}

/// A region representing the HALF-OPEN volume,
/// [x0, x1), [y0, y1), [z0, z1)
#[derive(Debug, Clone)]
struct Region {
    state: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Region {
    // on x=11847..30590,y=-66766..73165,z=19550..26333
    fn from_str(mut src: &str) -> Region {
        let state;
        match &src[0..2] {
            "on" => { state = true;  src = &src[3..]; },
            "of" => { state = false; src = &src[4..]; },
            pre  => panic!("Unexpected prefix '{}'", pre),
        }
        let mut iter = src.split(',');
        let mut pair = (iter.next().unwrap())[2..].split("..").map(|s| s.parse::<i32>().unwrap());
        let x = (pair.next().unwrap(), 1+pair.next().unwrap());
        let mut pair = (iter.next().unwrap())[2..].split("..").map(|s| s.parse::<i32>().unwrap());
        let y = (pair.next().unwrap(), 1+pair.next().unwrap());
        let mut pair = (iter.next().unwrap())[2..].split("..").map(|s| s.parse::<i32>().unwrap());
        let z = (pair.next().unwrap(), 1+pair.next().unwrap());
        assert!(x.0 < x.1);
        assert!(y.0 < y.1);
        assert!(z.0 < z.1);
        Region { state, x, y, z }
    }
}

fn load<R: std::io::Read>(contents: R) -> Vec<Region> {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        rv.push(Region::from_str(&line.unwrap()));
    }
    return rv;
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("22.in"));
    let sequence = load(std::fs::File::open(fname).unwrap());

    let mut reactor = Reactor::init(&sequence);
    for r in sequence {
        reactor.apply(&r);
    }

    println!("Part 1: {}", reactor.count_inner());
    println!("Part 2: {}", reactor.count());
}
