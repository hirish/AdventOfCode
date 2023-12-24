use std::{collections::HashSet, time::Instant};

use aoc2024::read_stdin;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coord(isize, isize);

struct Map(HashSet<Coord>);

impl Map {
    fn h(&self) -> isize {
        self.0.iter().map(|&Coord(_, y)| y).max().unwrap()
    }
    fn w(&self) -> isize {
        self.0.iter().map(|&Coord(x, _)| x).max().unwrap()
    }
}

type Input = Vec<Map>;

fn parse(input: String) -> Input {
    input
        .split("\n\n")
        .map(|map| {
            Map(map
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().filter_map(move |(x, c)| {
                        if c == '#' {
                            Some(Coord(x as isize, y as isize))
                        } else {
                            None
                        }
                    })
                })
                .collect())
        })
        .collect()
}

fn row_reflection(map: &Map) -> Option<isize> {
    let h = map.h();
    for y_line in 0..h {
        let mut all_work = true;
        for &Coord(x, y) in map.0.iter() {
            let dy = y_line - y + 1;
            let py = y_line + dy;

            if py >= 0 && py <= h && !map.0.contains(&Coord(x, py)) {
                all_work = false;
                break;
            }
        }
        if all_work {
            return Some(y_line);
        }
    }
    None
}

fn col_reflection(map: &Map) -> Option<isize> {
    let w = map.w();
    for x_line in 0..w {
        let mut all_work = true;
        for &Coord(x, y) in map.0.iter() {
            let dx = x_line - x + 1;
            let px = x_line + dx;

            if px >= 0 && px <= w && !map.0.contains(&Coord(px, y)) {
                all_work = false;
                break;
            }
        }
        if all_work {
            return Some(x_line);
        }
    }
    None
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let maps = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let r: isize = maps.iter().filter_map(row_reflection).map(|v| v + 1).sum();
    let c: isize = maps.iter().filter_map(col_reflection).map(|v| v + 1).sum();

    println!("{:?}", 100 * r + c);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
