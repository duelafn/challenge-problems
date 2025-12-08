// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::io::BufRead as _;

type Point = [i64; 3];
type T = Vec<[i64; 3]>;

fn load<R: std::io::Read>(contents: R) -> T {
    let mut rv = Vec::new();
    for line in std::io::BufReader::new(contents).lines() {
        let line = line.unwrap();
        let mut iter = line.split(',');
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        let c = iter.next().unwrap().parse().unwrap();
        rv.push([a,b,c]);
    }
    return rv;
}

fn l22(a: &Point, b: &Point) -> i64 {
    (a[0]-b[0])*(a[0]-b[0])
        + (a[1]-b[1])*(a[1]-b[1])
        + (a[2]-b[2])*(a[2]-b[2])
}

fn dist_vec(data: &T) -> Vec<(i64, usize, usize)> {
    let mut dist = Vec::new();
    for i in 0..data.len() {
        for j in i+1..data.len() {
            dist.push((l22(&data[i], &data[j]), i, j));
        }
    }
    dist.sort_unstable();
    dist
}

fn part1(data: &T, count: usize) -> usize {
    let dist = dist_vec(data);
    let mut circuit: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut tag: Vec<usize> = vec![0; data.len()];

    let mut n = 1;
    for &(_, i, j) in &dist[0..count] {
        if tag[i] > 0 { // Merge into first circuit
            let k = tag[i];
            if tag[j] > 0 { // Merge
                if tag[i] != tag[j] {
                    let boxes = circuit.remove(&tag[j]).unwrap();
                    circuit.get_mut(&k).unwrap().extend_from_slice(&boxes);
                    for m in boxes { tag[m] = k; }
                }
            } else { // Append
                tag[j] = k;
                circuit.get_mut(&k).unwrap().push(j);
            }
        }

        else if tag[j] > 0 { // Append to second circuit
            tag[i] = tag[j];
            circuit.get_mut(&tag[j]).unwrap().push(i);
        }

        else { // Create a new circuit
            tag[i] = n;
            tag[j] = n;
            circuit.insert(n, vec![i, j]);
            n += 1;
        };
    }

    let mut circuits: Vec<_> = circuit.values().collect();
    circuits.sort_unstable_by_key(|v| usize::MAX - v.len());
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

fn part2(data: &T) -> i64 {
    let dist = dist_vec(data);
    let mut circuit: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut tag: Vec<usize> = vec![0; data.len()];

    let mut seen = 0;
    let mut n = 1;
    for (_, i, j) in dist {
        if tag[i] > 0 { // Merge into first circuit
            let k = tag[i];
            if tag[j] > 0 { // Merge
                if tag[i] != tag[j] {
                    let boxes = circuit.remove(&tag[j]).unwrap();
                    circuit.get_mut(&k).unwrap().extend_from_slice(&boxes);
                    for m in boxes { tag[m] = k; }
                }
            } else { // Append
                seen += 1;
                tag[j] = k;
                circuit.get_mut(&k).unwrap().push(j);
            }
            // This connection unified the circuits
            if seen == data.len() && circuit.len() == 1 {
                return data[i][0] * data[j][0];
            }
        }

        else if tag[j] > 0 { // Append to second circuit
            seen += 1;
            tag[i] = tag[j];
            circuit.get_mut(&tag[j]).unwrap().push(i);
            // This connection unified the circuits
            if seen == data.len() && circuit.len() == 1 {
                return data[i][0] * data[j][0];
            }
        }

        else { // Create a new circuit
            seen += 2;
            tag[i] = n;
            tag[j] = n;
            circuit.insert(n, vec![i, j]);
            n += 1;
        };
    }

    panic!("bummer");
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("08.in"));
    let rec = load(std::fs::File::open(fname).unwrap());

    println!("Part 1: {}", part1(&rec, 1000));
    println!("Part 2: {}", part2(&rec));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rec = load(r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#.trim_start().as_bytes());
        assert_eq!(part1(&rec, 10), 40);
        assert_eq!(part2(&rec), 25272);
    }
}
