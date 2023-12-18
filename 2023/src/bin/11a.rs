use std::{collections::HashSet, time::Instant};

use aoc2024::read_stdin;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, Debug)]
struct Pos(isize, isize);

impl Pos {
    fn dist(&self, other: &Self) -> usize {
        return (self.0 - other.0).abs() as usize + (self.1 - other.1).abs() as usize;
    }
}

#[derive(Clone, Debug)]
struct Map(HashSet<Pos>);

impl Map {
    fn h(&self) -> isize {
        self.0.iter().map(|p| p.1).max().unwrap()
    }

    fn w(&self) -> isize {
        self.0.iter().map(|p| p.0).max().unwrap()
    }

    fn expand(self) -> Self {
        let h = self.h();
        let w = self.w();

        let non_blank_rows: HashSet<_> = self.0.iter().map(|p| p.1).collect();
        let blank_rows: Vec<_> = (0..h)
            .into_iter()
            .filter(|y| !non_blank_rows.contains(y))
            .collect();

        let non_blank_cols: HashSet<_> = self.0.iter().map(|p| p.0).collect();
        let blank_cols: Vec<_> = (0..w)
            .into_iter()
            .filter(|x| !non_blank_cols.contains(x))
            .collect();

        Self(
            self.0
                .into_iter()
                .map(|Pos(x, y)| {
                    let dx = blank_cols.iter().filter(|rx| **rx < x).count() as isize;
                    let dy = blank_rows.iter().filter(|ry| **ry < y).count() as isize;
                    Pos(x + dx, y + dy)
                })
                .collect(),
        )
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
                    Some(Pos(x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect())
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let map = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let map = map.expand();
    let val: usize = map
        .0
        .iter()
        .flat_map(|from| {
            map.0
                .iter()
                .filter_map(move |to| if from < to { Some(from.dist(to)) } else { None })
        })
        .sum();

    println!("{:?}", val);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
