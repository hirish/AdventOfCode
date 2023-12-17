use std::{collections::HashMap, time::Instant};

use aoc2024::read_stdin;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::E => Self::W,
            Self::W => Self::E,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    S,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Pos(isize, isize);

impl Pos {
    fn next(&self, t: Tile, entry: Dir) -> Option<(Self, Dir)> {
        match (t, entry) {
            (Tile::NS, Dir::N) => Some(self.apply(Dir::S)),
            (Tile::NS, Dir::S) => Some(self.apply(Dir::N)),
            (Tile::EW, Dir::E) => Some(self.apply(Dir::W)),
            (Tile::EW, Dir::W) => Some(self.apply(Dir::E)),
            (Tile::NE, Dir::N) => Some(self.apply(Dir::E)),
            (Tile::NE, Dir::E) => Some(self.apply(Dir::N)),
            (Tile::NW, Dir::N) => Some(self.apply(Dir::W)),
            (Tile::NW, Dir::W) => Some(self.apply(Dir::N)),
            (Tile::SW, Dir::S) => Some(self.apply(Dir::W)),
            (Tile::SW, Dir::W) => Some(self.apply(Dir::S)),
            (Tile::SE, Dir::S) => Some(self.apply(Dir::E)),
            (Tile::SE, Dir::E) => Some(self.apply(Dir::S)),
            (Tile::S, _) => None,
            _ => panic!("Unreachable: {:?} {:?}", t, entry),
        }
    }

    fn apply(&self, dir: Dir) -> (Self, Dir) {
        match dir {
            Dir::N => (Pos(self.0, self.1 - 1), dir.opposite()),
            Dir::S => (Pos(self.0, self.1 + 1), dir.opposite()),
            Dir::E => (Pos(self.0 + 1, self.1), dir.opposite()),
            Dir::W => (Pos(self.0 - 1, self.1), dir.opposite()),
        }
    }
}

type Input = (Pos, HashMap<Pos, Tile>);

fn parse(input: String) -> Input {
    let map: HashMap<Pos, Tile> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                match c {
                    '|' => Some(Tile::NS),
                    '-' => Some(Tile::EW),
                    'L' => Some(Tile::NE),
                    'J' => Some(Tile::NW),
                    '7' => Some(Tile::SW),
                    'F' => Some(Tile::SE),
                    'S' => Some(Tile::S),
                    _ => None,
                }
                .map(|t| (Pos(x as isize, y as isize), t))
            })
        })
        .collect();

    let start = map
        .iter()
        .find(|(_, t)| **t == Tile::S)
        .map(|(p, _)| *p)
        .unwrap();

    (start, map)
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let (start_pos, map) = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let start_dir = vec![Dir::N, Dir::S, Dir::E, Dir::W]
        .into_iter()
        .find(|d| {
            let v = map.get(&start_pos.apply(*d).0);
            match (v, d) {
                (Some(&v), Dir::S) => v == Tile::NE || v == Tile::NW || v == Tile::NS,
                (Some(&v), Dir::N) => v == Tile::SE || v == Tile::SW || v == Tile::NS,
                (Some(&v), Dir::W) => v == Tile::NE || v == Tile::SE || v == Tile::EW,
                (Some(&v), Dir::E) => v == Tile::SE || v == Tile::SW || v == Tile::EW,
                (None, _) => false,
            }
        })
        .unwrap();

    let mut pos = start_pos.apply(start_dir).0;
    let mut entry = start_dir.opposite();

    let mut perimeter = 1;
    loop {
        perimeter += 1;
        let &t = map.get(&pos).unwrap();
        if pos == start_pos {
            break;
        }
        (pos, entry) = pos.next(t, entry).unwrap()
    }

    perimeter /= 2;

    println!("{:?}", perimeter);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
