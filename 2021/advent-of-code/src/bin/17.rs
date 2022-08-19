// SPDX-License-Identifier: MIT

use std::collections::HashSet;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct BBox {
    xmin: i64, xmax: i64,
    ymin: i64, ymax: i64,
}

impl BBox {
    fn new(xmin: i64, xmax: i64, ymin: i64, ymax: i64) -> Self {
        BBox{ xmin, xmax, ymin, ymax }
    }

    fn contains(&self, x: i64, y: i64) -> bool {
        return x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax
    }

    fn shoot_length(&self, n: i64, seen: &mut HashSet<(i64, i64)>) {
        // Y = n v - degrade   ==>  v = (Y + degrade) / n
        let degrade = (n-1)*n/2;
        let mut xv = Vec::new();

        // Solve: v(v+1)/2 = xmin  for minimum velocity to reach xmin
        let v_x = ((((1 + 8 * self.xmin) as f64).sqrt() - 1.0)/2.0) as i64;
        for v in v_x.. {
            let x = if v < n { v*(v+1)/2 } else { n*v - degrade };
            if x >  self.xmax { break; }       // Too fast!
            if x >= self.xmin { xv.push((v, x)); }  // Hit!
        }

        if xv.is_empty() { return; }

        let v_y = (self.ymin + degrade) / n;
        for v in v_y.. {
            let y = n*v - degrade;
            if y >  self.ymax { return; }
            if y >= self.ymin {
                for &(v_x, x) in xv.iter() {
                    if self.contains(x, y) { seen.insert((v_x, v)); }
                }
            }
        }
    }
}

impl From<&str> for BBox {
    fn from(src: &str) -> Self {
        let (mut xmin, mut xmax, mut ymin, mut ymax) = (0, 0, 0, 0);
        for chunk in src.split_ascii_whitespace() {
            if let Some(x) = chunk.strip_prefix("x=") {
                let mut range = x.trim_matches(',').split("..");
                xmin = range.next().unwrap().parse().unwrap();
                xmax = range.next().unwrap().parse().unwrap();
            }
            else if let Some(y) = chunk.strip_prefix("y=") {
                let mut range = y.trim_matches(',').split("..");
                ymin = range.next().unwrap().parse().unwrap();
                ymax = range.next().unwrap().parse().unwrap();
            }
        }
        assert!(xmin > 0);
        assert!(ymax < 0);
        return BBox::new(xmin, xmax, ymin, ymax);
    }
}


fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("17.in"));
    let contents = std::fs::read_to_string(fname).unwrap();

    let bbox = BBox::from(contents.as_ref());
    let mut seen = HashSet::new();
    for i in 1..100_000 { // Magic number / Trial and error  (actual requirement was around 300)
        bbox.shoot_length(i, &mut seen);
    }
    let mut max = 0;
    for (_, y) in seen.iter() {
        if *y > max { max = *y; }
    }
    println!("Part 1: {}", max * (max+1) / 2);
    println!("Part 2: {}", seen.len());
}
