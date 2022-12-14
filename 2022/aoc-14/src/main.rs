use aoc_14::read_stdin;

use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl Coord {
    fn drop(&self) -> Vec<Coord> {
        let &Coord { x, y } = self;
        vec![
            Coord { x, y: y + 1 },
            Coord { x: x - 1, y: y + 1 },
            Coord { x: x + 1, y: y + 1 },
        ]
    }
}

type Input = HashSet<Coord>;

fn parse(input: String) -> Input {
    let mut map = HashSet::new();

    for line in input.lines() {
        for (from, to) in line.split(" -> ").tuple_windows() {
            let from: Coord = from.parse().unwrap();
            let to: Coord = to.parse().unwrap();
            let f_x = from.x.min(to.x);
            let f_y = from.y.min(to.y);
            let t_x = from.x.max(to.x);
            let t_y = from.y.max(to.y);

            for x in f_x..=t_x {
                for y in f_y..=t_y {
                    map.insert(Coord { x, y });
                }
            }
        }
    }

    map
}

fn part_1(mut input: Input) -> usize {
    let mut overflowing = false;
    let no_stones = input.len();
    let bottom = input.iter().map(|k| k.y).max().unwrap();

    while !overflowing {
        let mut sand = Coord { x: 500, y: 0 };
        loop {
            if sand.y > bottom {
                overflowing = true;
                break;
            }
            match sand
                .drop()
                .into_iter()
                .filter(|n| !input.contains(n))
                .next()
            {
                Some(n) => sand = n,
                None => {
                    input.insert(sand);
                    sand = Coord { x: 500, y: 0 };
                }
            }
        }
    }

    input.len() - no_stones
}

fn part_2(mut input: Input) -> usize {
    let mut overflowing = false;
    let no_stones = input.len();
    let bottom = input.iter().map(|k| k.y).max().unwrap();

    while !overflowing {
        let mut sand = Coord { x: 500, y: 0 };
        loop {
            if input.contains(&sand) {
                overflowing = true;
                break;
            }
            if sand.y > bottom {
                input.insert(sand);
                break;
            }
            match sand
                .drop()
                .into_iter()
                .filter(|n| !input.contains(n))
                .next()
            {
                Some(n) => sand = n,
                None => {
                    input.insert(sand);
                    sand = Coord { x: 500, y: 0 };
                }
            }
        }
    }

    input.len() - no_stones
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
