// SPDX-License-Identifier: MIT

#[derive(Clone)]
struct Board {
    board: Vec<Vec<Option<i32>>>,
    active: bool,
}
impl Board {
    fn mark(&mut self, n: i32) -> bool {
        let mut pos = None;
        'outer:
        for (r, vec) in self.board.iter_mut().enumerate() {
            for (c, item) in vec.iter_mut().enumerate() {
                if *item == Some(n) {
                    item.take();
                    pos = Some((r, c));
                    break 'outer;
                }
            }
        }

        if let Some((row, col)) = pos {
            if (0..5).all(|j| self.board[row][j].is_none()) { self.active = false; return true; }
            if (0..5).all(|i| self.board[i][col].is_none()) { self.active = false; return true; }
        }
        return false;
    }

    fn score(&mut self, call: i32) -> i32 {
        let mut sum: i32 = 0;
        for row in self.board.iter() {
            sum += row.iter().filter_map(|x| *x).sum::<i32>();
        }
        return sum * call;
    }
}

impl From<&str> for Board {
    fn from(src: &str) -> Self {
        let mut board = Vec::new();
        for line in src.lines() {
            board.push(line.split_ascii_whitespace().map(|v| Some(v.parse::<i32>().unwrap())).collect());
        }
        return Board { board, active: true };
    }
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("04.in"));
    let content = std::fs::read_to_string(fname).unwrap();
    let mut blocks = content.split("\n\n");
    let nums: Vec<i32> = blocks.next().unwrap().split(',').map(|v| v.parse().unwrap()).collect();
    let boards: Vec<Board> = blocks.map(|b| Board::from(b)).collect();

    {   let mut boards = boards.clone();
        'outer:
        for &call in nums.iter() {
            for (bnum, board) in boards.iter_mut().enumerate() {
                if board.mark(call) {
                    println!("Part 1: called {}, BINGO on board {} -> score {}", call, bnum, board.score(call));
                    break 'outer;
                }
            }
        }
    }

    {   let mut boards = boards.clone();
        let mut todo = boards.len();
        'outer2:
        for &call in nums.iter() {
            for (bnum, board) in boards.iter_mut().enumerate() {
                if board.active && board.mark(call) {
                    todo -= 1;
                    if todo == 0 {
                        println!("Part 2: Called {}, BINGO on (last) board {} -> score {}", call, bnum, board.score(call));
                        break 'outer2;
                    }
                }
            }
        }
    }
}
