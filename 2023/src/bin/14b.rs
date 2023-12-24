use std::{collections::HashMap, time::Instant};

use aoc2024::read_stdin;
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coord(isize, isize);

#[derive(PartialEq, Clone)]
enum Rock {
    Round,
    Square,
}

#[derive(Clone)]
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

fn up(map: &Map) -> Map {
    let mut new_map = FxHashMap::default();

    for x in 0..=map.w() {
        let mut top = -1;
        let mut no_rounds = 0;

        for y in 0..=map.h() {
            match map.0.get(&Coord(x, y)) {
                Some(Rock::Round) => {
                    new_map.insert(Coord(x, top + no_rounds + 1), Rock::Round);
                    no_rounds += 1;
                }
                Some(Rock::Square) => {
                    new_map.insert(Coord(x, y), Rock::Square);
                    top = y;
                    no_rounds = 0;
                }
                None => {}
            }
        }
    }

    Map(new_map)
}

fn down(map: &Map) -> Map {
    let mut new_map = FxHashMap::default();
    let h = map.h();

    for x in 0..=map.w() {
        let mut bottom = h + 1;
        let mut no_rounds = 0;

        for y in 0..=h {
            let y = h - y;
            match map.0.get(&Coord(x, y)) {
                Some(Rock::Round) => {
                    new_map.insert(Coord(x, bottom - no_rounds - 1), Rock::Round);
                    no_rounds += 1;
                }
                Some(Rock::Square) => {
                    new_map.insert(Coord(x, y), Rock::Square);
                    bottom = y;
                    no_rounds = 0;
                }
                None => {}
            }
        }
    }

    Map(new_map)
}

fn left(map: &Map) -> Map {
    let mut new_map = FxHashMap::default();

    for y in 0..=map.h() {
        let mut left = -1;
        let mut no_rounds = 0;

        for x in 0..=map.w() {
            match map.0.get(&Coord(x, y)) {
                Some(Rock::Round) => {
                    new_map.insert(Coord(left + no_rounds + 1, y), Rock::Round);
                    no_rounds += 1;
                }
                Some(Rock::Square) => {
                    new_map.insert(Coord(x, y), Rock::Square);
                    left = x;
                    no_rounds = 0;
                }
                None => {}
            }
        }
    }

    Map(new_map)
}

fn right(map: &Map) -> Map {
    let mut new_map = FxHashMap::default();
    let w = map.w();

    for y in 0..=map.h() {
        let mut right = w + 1;
        let mut no_rounds = 0;

        for x in 0..=w {
            let x = w - x;
            match map.0.get(&Coord(x, y)) {
                Some(Rock::Round) => {
                    new_map.insert(Coord(right - no_rounds - 1, y), Rock::Round);
                    no_rounds += 1;
                }
                Some(Rock::Square) => {
                    new_map.insert(Coord(x, y), Rock::Square);
                    right = x;
                    no_rounds = 0;
                }
                None => {}
            }
        }
    }

    Map(new_map)
}

fn state(map: &Map) -> String {
    (0..=map.h())
        .map(|y| {
            (0..=map.w())
                .map(|x| match map.0.get(&Coord(x, y)) {
                    Some(Rock::Round) => 'O',
                    Some(Rock::Square) => '#',
                    _ => '.',
                })
                .join("")
        })
        .join("\n")
}

fn load(map: &Map) -> isize {
    map.0
        .iter()
        .filter_map(|(&Coord(_, y), r)| {
            if r == &Rock::Round {
                Some(map.h() + 1 - y)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let mut map = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();
    let count = 1000000000;

    let dirs = [up, left, down, right];
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut states: Vec<Map> = vec![];

    let mut v = 0;
    for i in 0.. {
        for dir in dirs {
            map = dir(&map);
        }

        let s = state(&map);

        if let Some(j) = seen.get(&s) {
            let period = i - j;
            v = j + ((count - j) % period) - 1;
            break;
        }

        seen.insert(s, i);
        states.push(map.clone());
    }

    let state = states.get(v).unwrap();
    let l = load(state);

    println!("{:?}", l);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
