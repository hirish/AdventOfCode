use std::collections::HashSet;

use aoc_1::read_stdin;

fn print(board: &Vec<Vec<usize>>) {
    for row in board {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
    println!("");
}

fn neighbours(px: usize, py: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut n = vec![];
    let px = px as isize;
    let py = py as isize;
    let h = h as isize;
    let w = w as isize;

    for y in py-1..py+2 {
        for x in px-1..px+2 {
            if y < 0 || y >= h || x < 0 || x >= w ||px == x && py == y {
                continue
            }

            n.push((x as usize, y as usize))
        }
    }

    n

}

fn step(board: &mut Vec<Vec<usize>>) -> usize {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..board.len() {
        for x in 0..board[y].len() {
            board[y][x] += 1
        }
    }

    let mut changes = true;
    while changes {
        changes = false;

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] > 9 && !flashed.contains(&(x, y)) {
                    flashed.insert((x, y));

                    for (nx, ny) in neighbours(x, y, board.len(), board[y].len()) {
                        board[ny][nx] += 1
                    }

                    changes = true
                }
            }
        }
    }

    for &(x, y) in flashed.iter() {
        board[y][x] = 0;
    }

    flashed.len()
}

fn part_1(mut board: Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step(&mut board);
    }

    flashes
}

fn part_2(mut board: Vec<Vec<usize>>) -> usize {
    for i in 0..500 {
        if step(&mut board) == 100 {
            return i + 1
        }
    }

    0
}

fn main() {
    let input = read_stdin();
    let board: Vec<Vec<usize>> = input.lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
    }).collect();

    println!("Answer 1: {}", part_1(board.clone()));
    println!("Answer 2: {}", part_2(board));
}
