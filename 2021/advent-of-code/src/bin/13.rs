// SPDX-License-Identifier: MIT

#[derive(Copy, Clone, PartialEq, Eq)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Default)]
struct FoldedMap {
    folds: Vec<Fold>,
    map: Vec<Vec<u8>>,
}
impl FoldedMap {
    fn new() -> Self { Default::default() }
    fn fold(&mut self, fold: Fold) { self.folds.push(fold); }
    fn dot(&mut self, (mut x, mut y): &(usize, usize)) {
        for fold in self.folds.iter() {
            // n stays at row/col n, row/col n+1 maps to row/col n-1
            match *fold {
                Fold::X(n) => { if x > n { x = n - (x - n); } }
                Fold::Y(m) => { if y > m { y = m - (y - m); } }
            }
        }
        while self.map.len() <= y { self.map.push(Vec::new()) }
        let row = &mut self.map[y];
        while row.len() <= x { row.push(0) }
        row[x] = 1;
    }
    fn count(&self) -> usize {
        self.map.iter().map(|row| row.iter().map(|v| *v as usize).sum::<usize>()).sum()
    }
}
impl std::fmt::Display for FoldedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for item in row.iter() {
                write!(f, "{}", if *item > 0 { '#' } else { ' ' })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


fn records(fname: &str) -> (Vec<(usize,usize)>, Vec<Fold>) {
    let contents = std::fs::read_to_string(fname).unwrap();

    let mut pts = Vec::new();
    let mut folds = Vec::new();

    for line in contents.lines() {
        if let Some(x) = line.strip_prefix("fold along x=") {
            folds.push(Fold::X(x.parse().unwrap()));
        }
        else if let Some(y) = line.strip_prefix("fold along y=") {
            folds.push(Fold::Y(y.parse().unwrap()));
        }
        else if line.len() > 1 {
            let mut iter = line.split(',');
            pts.push( (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap()) );
        }
    }
    return (pts, folds);
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("13.in"));
    let (pts, folds) = records(&fname);

    let mut map = FoldedMap::new();
    map.fold(folds[0]);
    for pt in pts.iter() { map.dot(pt); }
    println!("Part 1: {}", map.count());

    let mut map = FoldedMap::new();
    for fd in folds.iter() { map.fold(*fd); }
    for pt in pts.iter() { map.dot(pt); }
    println!("Part 2:\n{}", map);
}
