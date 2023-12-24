use std::time::Instant;

use aoc2024::read_stdin;
use rustc_hash::FxHashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coord(isize, isize);

enum Rock {
    Round,
    Square,
}

struct Map(FxHashMap<Coord, Rock>);

impl Map {
    fn h(&self) -> isize {
        self.0.keys().map(|&Coord(_, y)| y).max().unwrap()
    }
    fn w(&self) -> isize {
        self.0.keys().map(|&Coord(x, _)| x).max().unwrap()
    }
}

type Input = Map;

fn parse(input: String) -> Input {
    Map(input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((Coord(x as isize, y as isize), Rock::Square))
                } else if c == 'O' {
                    Some((Coord(x as isize, y as isize), Rock::Round))
                } else {
                    None
                }
            })
        })
        .collect())
}

fn col_load(map: &Map, x: isize) -> isize {
    let h = map.h() + 1;

    let mut total = 0;
    let mut top = -1;
    let mut no_rounds = 0;

    for y in 0..h {
        match map.0.get(&Coord(x, y)) {
            Some(Rock::Round) => {
                total += h - (top + no_rounds + 1);
                no_rounds += 1;
            }
            Some(Rock::Square) => {
                top = y;
                no_rounds = 0;
            }
            None => {}
        }
    }
    total
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let map = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let v: isize = (0..=map.w()).map(|x| col_load(&map, x)).sum();

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
