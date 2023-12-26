use std::{collections::BinaryHeap, time::Instant};

use aoc2024::read_stdin;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Dir {
    N(isize),
    S(isize),
    W(isize),
    E(isize),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coord(isize, isize);

impl Coord {
    fn neighbours(&self, dir: Dir) -> Vec<(Self, Dir)> {
        match dir {
            Dir::N(d) => {
                let mut v = vec![
                    (Self(self.0, self.1 - 1), Dir::W(0)),
                    (Self(self.0, self.1 + 1), Dir::E(0)),
                ];
                if d < 2 {
                    v.push((Self(self.0 - 1, self.1), Dir::N(d + 1)))
                }
                v
            }
            Dir::S(d) => {
                let mut v = vec![
                    (Self(self.0, self.1 - 1), Dir::W(0)),
                    (Self(self.0, self.1 + 1), Dir::E(0)),
                ];
                if d < 2 {
                    v.push((Self(self.0 + 1, self.1), Dir::S(d + 1)))
                }
                v
            }
            Dir::W(d) => {
                let mut v = vec![
                    (Self(self.0 - 1, self.1), Dir::N(0)),
                    (Self(self.0 + 1, self.1), Dir::S(0)),
                ];
                if d < 2 {
                    v.push((Self(self.0, self.1 - 1), Dir::W(d + 1)))
                }
                v
            }
            Dir::E(d) => {
                let mut v = vec![
                    (Self(self.0 - 1, self.1), Dir::N(0)),
                    (Self(self.0 + 1, self.1), Dir::S(0)),
                ];
                if d < 2 {
                    v.push((Self(self.0, self.1 + 1), Dir::E(d + 1)))
                }
                v
            }
        }
    }
}

struct Map(FxHashMap<Coord, usize>);

#[derive(Eq, PartialEq)]
struct FVal(Coord, Dir, usize);

impl Ord for FVal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for FVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.2.partial_cmp(&self.2)
    }
}

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
            l.chars().enumerate().map(move |(x, c)| {
                (
                    Coord(x as isize, y as isize),
                    c.to_digit(10).unwrap() as usize,
                )
            })
        })
        .collect())
}

fn astar(map: &Map, &from: &Coord, &Coord(tx, ty): &Coord) -> usize {
    let start1 = (from, Dir::E(-1));
    let start2 = (from, Dir::S(-1));
    let h = |Coord(x, y)| ((x - tx).abs() + (y - ty).abs()) as usize;

    let mut closed: FxHashSet<(Coord, Dir)> = FxHashSet::default();

    let mut g: FxHashMap<(Coord, Dir), usize> = [(start1, 0), (start2, 0)].into_iter().collect();
    let mut f: BinaryHeap<FVal> = [
        FVal(start1.0, start1.1, h(from)),
        FVal(start2.0, start2.1, h(from)),
    ]
    .into_iter()
    .collect();

    while !f.is_empty() {
        let FVal(curr, dir, v) = f.pop().unwrap();

        if (curr.0) == tx && (curr.1) == ty {
            return v;
        }

        closed.insert((curr, dir));

        for neighbour in curr.neighbours(dir) {
            if closed.contains(&neighbour) {
                continue;
            }

            if let Some(d) = map.0.get(&neighbour.0) {
                let new_g = g.get(&(curr, dir)).unwrap() + d;
                if !g.contains_key(&neighbour) || &new_g < g.get(&neighbour).unwrap() {
                    g.insert(neighbour, new_g);
                    f.push(FVal(neighbour.0, neighbour.1, new_g + h(neighbour.0)));
                }
            }
        }
    }

    0
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let map = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let v = astar(&map, &Coord(0, 0), &Coord(map.h(), map.w()));

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
