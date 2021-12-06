use std::str::FromStr;

use aoc_1::read_stdin;


#[derive(Debug)]
struct Pos {
    value: usize,
    selected: bool,
}


#[derive(Debug)]
struct Board {
    board: Vec<Vec<Pos>>,
}


impl Board {
    fn select(self: &mut Self, value: usize) {
        for row in &mut self.board {
            for mut pos in row {
                if pos.value  == value {
                    (*pos).selected = true
                }
            }
        }
    }

    fn winner(self: &Self) -> bool {
        let b = &self.board;

        for row in b {
            if row.iter().all(|p| p.selected) {
                return true
            }
        }

        for i in 0..b.len() {
            if b.iter().all(|r| r[i].selected) {
                return true
            }
        }

        false
    }

    fn score(self: &Self) -> usize {
        self.board
            .iter()
            .map(|r| {
                r
                    .iter()
                    .filter(|p| !p.selected)
                    .map(|p| p.value)
                    .sum::<usize>()
            })
            .sum()
    }
}


impl FromStr for Board {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let board: Vec<Vec<Pos>> = input
            .split('\n')
            .map(|row| {
                row
                    .trim()
                    .split_whitespace()
                    .map(|value| Pos {value: value.parse().unwrap(), selected: false})
                    .collect()
            })
            .collect();

        Ok(Board {board})
    }
}


fn part_1(input: String) -> Option<usize> {
    let a: Vec<&str> = input
        .split("\n\n")
        .map(|x| x.trim())
        .collect();

    let called_numbers: Vec<usize> = a[0]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = a
        .iter()
        .skip(1)
        .map(|b| b.parse().unwrap())
        .collect();

    for n in called_numbers {
        for board in &mut boards {
            board.select(n);
            if board.winner() {
                return Some(n * board.score())
            }
        }
    }

    None
}

fn part_2(input: String) -> Option<usize> {
    let a: Vec<&str> = input
        .split("\n\n")
        .map(|x| x.trim())
        .collect();

    let called_numbers: Vec<usize> = a[0]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = a
        .iter()
        .skip(1)
        .map(|b| b.parse().unwrap())
        .collect();

    for n in called_numbers {
        let no_boards = boards.len();
        for board in &mut boards {
            board.select(n);
            if board.winner() {
                if no_boards == 1 {
                    return Some(n * board.score())
                }
            }
        }

        boards = boards
            .into_iter()
            .filter(|b| !b.winner())
            .collect();
    }

    None
}

fn main() {
    let input = read_stdin();
    println!("Answer 1: {}", part_1(input.clone()).expect("No answer found"));
    println!("Answer 2: {}", part_2(input).expect("No answer found"));
}
