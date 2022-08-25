// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

const LIMIT: usize = 12;
type Point = [i32; 3];

fn load(contents: &str) -> Vec<Vec<Point>> {
    let mut rv = Vec::new();
    let mut scan: Vec<Point> = Vec::new();
    for line in contents.lines() {
        if line.starts_with("---") {
            if scan.len() > 0 { rv.push(scan); scan = Vec::new(); }
        }
        else if line.len() > 0 {
            let mut iter = line.split(',').map(|n| n.parse().unwrap());
            scan.push([ iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap() ]);
        }
    }
    rv.push(scan);
    return rv;
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct XForm { sign: (i32, i32, i32), perm: (usize, usize, usize), tran: (i32, i32, i32) }
impl XForm {
    fn apply(&self, p: &Point) -> Point {
        [
            self.sign.0 * p[self.perm.0] + self.tran.0,
            self.sign.1 * p[self.perm.1] + self.tran.1,
            self.sign.2 * p[self.perm.2] + self.tran.2,
        ]
    }
}

/// Find reflection and translation along single axis
fn find_tran(a: &Vec<Point>, b: &Vec<Point>, acol: usize, bcol: usize) -> Vec<(i32, i32)> {
    let mut rv = Vec::new();
    let mut count = HashMap::new();
    for &sign in &[-1, 1] {
        count.clear();
        for a in a.iter() {
            for b in b.iter() {
                *count.entry(a[acol] - sign * b[bcol]).or_insert(0_usize) += 1;
            }
        }
        rv.extend(count.iter().filter_map(|(&k, &n)| if n >= LIMIT { Some((sign, k)) } else { None }));
    }
    return rv;
}

/// Find transformation
fn find_xform(a: &Vec<Point>, b: &Vec<Point>) -> Option<XForm> {
    // Need to find axis permutation fit
    // Very ugly, but whatever; Coordinate-wise pairups filter for 3D overlap counting.
    for i in &[0, 1, 2] {
        for (sign_0, tran_0) in find_tran(&a, &b, 0, *i) {
            for j in &[0, 1, 2] {
                if *i == *j { continue; }
                for (sign_1, tran_1) in find_tran(&a, &b, 1, *j) {
                    for k in &[0, 1, 2] {
                        if *i == *k || *j == *k { continue; }
                        for (sign_2, tran_2) in find_tran(&a, &b, 2, *k) {
                            let xform = XForm {
                                sign: (sign_0, sign_1, sign_2),
                                perm: (*i, *j, *k),
                                tran: (tran_0, tran_1, tran_2),
                            };
                            // Final check for overlap over all three axes:
                            let b_t: HashSet<Point> = b.iter().map(|p| xform.apply(p)).collect();
                            if a.iter().filter(|p| b_t.contains(&p[..])).count() >= LIMIT {
                                return Some(xform); // Assume uniqueness as part of the problem so just return here
                            }
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn merge_scans(scan: &Vec<Vec<Point>>) -> (Vec<(usize, Point)>, Vec<Point>) {
    let mut merge = Vec::new();
    let mut done = HashSet::new();
    let mut new = HashSet::new();
    done.insert(0_usize);
    new.insert(0_usize);
    while done.len() < scan.len() {
        let mut next = HashSet::new();
        for i in new {
            let a = &scan[i];
            for (j, b) in scan.iter().enumerate() {
                if i != j && !done.contains(&j) {
                    if let Some(xform) = find_xform(a, b) {
                        merge.push((i, j, xform));
                        next.insert(j);
                        done.insert(j);
                    }
                }
            }
        }
        new = next;
    }

    assert_eq!(merge.len(), scan.len() - 1, "All scans paired up");

    let mut done = HashMap::new();
    let mut scanner = HashMap::new();
    for (i, j, xform) in merge.iter().rev() {
        let b = done.remove(j).unwrap_or_else(|| scan[*j].clone());
        let a = done.entry(*i).or_insert_with(|| scan[*i].clone());
        let sb = scanner.remove(j).unwrap_or_else(|| vec![(*j, [0; 3])]);
        let sa = scanner.entry(*i).or_insert_with(|| vec![(*i, [0; 3])]);
        a.extend(b.into_iter().map(|p| xform.apply(&p)));
        sa.extend(sb.into_iter().map(|(i, p)| (i, xform.apply(&p))));
    }

    assert_eq!(done.len(), 1, "Fully reduced");
    let beacons: HashSet<Point> = HashSet::from_iter(done.remove(&0).unwrap());
    (scanner.remove(&0).unwrap(), beacons.into_iter().collect())
}


fn l1(a: &Point, b: &Point) -> i32 {
    (a[0]-b[0]).abs() + (a[1]-b[1]).abs() + (a[2]-b[2]).abs()
}


fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("19.in"));
    let scan = load(&std::fs::read_to_string(fname).unwrap());

    let (scanners, beacons) = merge_scans(&scan);
    println!("Part 1: {}", beacons.len());

    let mut diameter = 0;
    for (i, (_, x1)) in scanners.iter().enumerate() {
        for (_, x2) in scanners[i+1..].iter() {
            diameter = diameter.max(l1(x1, x2));
        }
    }
    println!("Part 2: {}", diameter);
}
