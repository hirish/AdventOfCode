use std::collections::HashSet;

use aoc_1::read_stdin;

fn get_neighbours(px: isize, py: isize, w: isize, h: isize) -> Vec<(isize, isize)> {
    let mut n = vec![];

    for y in py-1..py+2 {
        if y < 0 || y >= h || y == py {
            continue
        }

        n.push((px, y))
    }

    for x in px-1..px+2 {
        if x < 0 || x >= w || x == px {
            continue
        }

        n.push((x, py))
    }

    n
}

fn part_1(board: Vec<Vec<isize>>) -> isize {
    let h = board.len() as isize;
    let w = board[0].len() as isize;
    let mut s = 0;

    for x in 0..w {
        for y in 0..h {
            let v = board[y as usize][x as usize];
            if get_neighbours(x, y, w, h).iter().all(|(nx, ny)| board[*ny as usize][*nx as usize] > v) {
                s += v + 1
            }
        }
    }

    s
}

fn part_2(board: Vec<Vec<isize>>) -> Option<usize> {
    let h = board.len() as isize;
    let w = board[0].len() as isize;

    let mut low_points: Vec<(isize, isize)> = vec!();

    for x in 0..w {
        for y in 0..h {
            let v = board[y as usize][x as usize];
            if get_neighbours(x, y, w, h).iter().all(|(nx, ny)| board[*ny as usize][*nx as usize] > v) {
                low_points.push((x, y));
            }
        }
    }

    let mut basins = vec!();
    for p in low_points {
        let mut explored = HashSet::new();
        let mut unexplored = HashSet::new();
        unexplored.insert(p);

        while !unexplored.is_empty() {
            let next = *unexplored.iter().next()?;
            let (x, y) = unexplored.take(&next)?;
            explored.insert((x, y));

            for n in get_neighbours(x, y, w, h) {
                if explored.contains(&n) || unexplored.contains(&n) {
                    continue
                }

                if board[n.1 as usize][n.0 as usize] == 9 {
                    continue
                }

                unexplored.insert(n);
            }
        }

        basins.push(explored.len());
    }

    basins.sort();
    basins.reverse();
    Some(basins[0] * basins[1] * basins[2])
}

fn main() {
    let input = read_stdin();
    let board: Vec<Vec<isize>> = input
        .split('\n')
        .map(|x| x
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect()
        )
        .collect();

    println!("Answer 1: {}", part_1(board.clone()));
    println!("Answer 2: {}", part_2(board).unwrap());
}
