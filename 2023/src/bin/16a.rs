use std::time::Instant;

use aoc2024::read_stdin;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coord(isize, isize);

impl Coord {
    fn apply(&self, dir: Dir) -> Self {
        match dir {
            Dir::N => Self(self.0, self.1 - 1),
            Dir::S => Self(self.0, self.1 + 1),
            Dir::E => Self(self.0 + 1, self.1),
            Dir::W => Self(self.0 - 1, self.1),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug, Hash, Eq)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug)]
enum Mirror {
    SplitV,
    SplitH,
    DiagTB,
    DiagBT,
}

struct Map(FxHashMap<Coord, Mirror>, isize, isize);

impl Map {
    fn new(map: FxHashMap<Coord, Mirror>) -> Self {
        let h = map.keys().map(|&Coord(_, y)| y).max().unwrap();
        let w = map.keys().map(|&Coord(x, _)| x).max().unwrap();
        Map(map, h, w)
    }
}

type Input = Map;

fn parse(input: String) -> Input {
    Map::new(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '|' {
                        Some((Coord(x as isize, y as isize), Mirror::SplitV))
                    } else if c == '-' {
                        Some((Coord(x as isize, y as isize), Mirror::SplitH))
                    } else if c == '/' {
                        Some((Coord(x as isize, y as isize), Mirror::DiagBT))
                    } else if c == '\\' {
                        Some((Coord(x as isize, y as isize), Mirror::DiagTB))
                    } else {
                        None
                    }
                })
            })
            .collect(),
    )
}

fn trace(map: &Map, pos: Coord, d: Dir) -> FxHashSet<Coord> {
    let mut visited: FxHashSet<(Coord, Dir)> = FxHashSet::default();
    let mut energised: FxHashSet<Coord> = FxHashSet::default();

    let mut to_visit = vec![(pos, d)];
    while let Some((pos, d)) = to_visit.pop() {
        if pos.0 < 0 || pos.0 > map.1 || pos.1 < 0 || pos.1 > map.2 {
            continue;
        }

        if visited.contains(&(pos, d)) {
            continue;
        }

        energised.insert(pos);
        visited.insert((pos, d));

        match map.0.get(&pos) {
            Some(Mirror::SplitV) => {
                if d == Dir::W || d == Dir::E {
                    to_visit.push((pos.apply(Dir::N), Dir::N));
                    to_visit.push((pos.apply(Dir::S), Dir::S));
                } else {
                    to_visit.push((pos.apply(d), d))
                }
            }
            Some(Mirror::SplitH) => {
                if d == Dir::N || d == Dir::S {
                    to_visit.push((pos.apply(Dir::E), Dir::E));
                    to_visit.push((pos.apply(Dir::W), Dir::W));
                } else {
                    to_visit.push((pos.apply(d), d))
                }
            }
            Some(Mirror::DiagTB) => to_visit.push(match d {
                Dir::N => (pos.apply(Dir::W), Dir::W),
                Dir::W => (pos.apply(Dir::N), Dir::N),
                Dir::S => (pos.apply(Dir::E), Dir::E),
                Dir::E => (pos.apply(Dir::S), Dir::S),
            }),
            Some(Mirror::DiagBT) => to_visit.push(match d {
                Dir::N => (pos.apply(Dir::E), Dir::E),
                Dir::E => (pos.apply(Dir::N), Dir::N),
                Dir::S => (pos.apply(Dir::W), Dir::W),
                Dir::W => (pos.apply(Dir::S), Dir::S),
            }),
            None => to_visit.push((pos.apply(d), d)),
        };
    }

    energised
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let map = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let v: usize = trace(&map, Coord(0, 0), Dir::E).len();

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
