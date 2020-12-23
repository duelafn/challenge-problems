// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellGame {
    vec: Vec<u32>,
    cur: usize,
}
impl ShellGame {
    pub fn new(init: &[u32], max: usize) -> ShellGame {
        let mut vec = Vec::with_capacity(max+1);
        vec.resize(init.len()+1, 0);
        let first = init[0];
        let mut last = first;
        for i in 1..init.len() {
            vec[last as usize] = init[i];
            last = init[i];
        }
        if max > init.len() {
            vec[last as usize] = (init.len()+1) as u32;
            for i in init.len()+2..=max {
                vec.push(i as u32)
            }
            vec.push(first);
        }
        else {
            vec[last as usize] = first;
        }
        ShellGame { vec, cur: first as usize }
    }

    pub fn shuffle(&mut self) {
        let old = self.cur;
        let a = self.vec[old] as usize;
        let b = self.vec[a] as usize;
        let c = self.vec[b] as usize;
        let mut insert = old;
        loop {
            insert = if insert == 1 { self.vec.len()-1 } else { insert - 1 };
            if insert != a && insert != b && insert != c { break; }
        }
        self.cur = self.vec[c] as usize;
        self.vec[c] = self.vec[insert];
        self.vec[insert] = a as u32;
        self.vec[old] = self.cur as u32;
    }

    pub fn ident(&self) -> String {
        let mut rv = String::with_capacity(8);
        let mut idx = 1 as usize;
        for _ in 1..9 {
            idx = self.vec[idx] as usize;
            rv.push(std::char::from_digit(idx as u32, 36).unwrap());
        }
        return rv;
    }

    pub fn hash(&self) -> u64 { (self.vec[1] as u64) * (self.vec[self.vec[1] as usize] as u64) }
}

impl std::fmt::Display for ShellGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut idx = self.cur;
        for _ in 1..self.vec.len() {
            write!(f, "{},", idx)?;
            idx = self.vec[idx] as usize;
        }
        Ok(())
    }
}



fn main() {
    let mut game = ShellGame::new(&[3,8,9,5,4,7,6,1,2], 9);
    for _ in 1..=100 {
        game.shuffle();
    }
    println!("Part 1: identifier {}", game.ident());

    let mut game = ShellGame::new(&[3,8,9,5,4,7,6,1,2], 1_000_000);
    for _ in 1..=10_000_000 {
        game.shuffle();
    }
    println!("Part 2: hash {}", game.hash());
}
