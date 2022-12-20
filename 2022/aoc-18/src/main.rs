use aoc_18::read_stdin;

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
struct Coord {
    x: u8,
    y: u8,
    z: u8,
}

impl Coord {
    fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }

    fn sides(&self) -> Vec<Face> {
        let &Coord { x, y, z } = self;

        vec![
            Face::new(
                Coord::new(x, y, z),
                Coord::new(x + 1, y + 1, z),
                Coord::new(x + 1, y, z),
                Coord::new(x, y + 1, z),
            ),
            Face::new(
                Coord::new(x, y, z),
                Coord::new(x, y + 1, z),
                Coord::new(x, y, z + 1),
                Coord::new(x, y + 1, z + 1),
            ),
            Face::new(
                Coord::new(x, y, z),
                Coord::new(x + 1, y, z),
                Coord::new(x, y, z + 1),
                Coord::new(x + 1, y, z + 1),
            ),
            Face::new(
                Coord::new(x + 1, y, z),
                Coord::new(x + 1, y + 1, z),
                Coord::new(x + 1, y, z + 1),
                Coord::new(x + 1, y + 1, z + 1),
            ),
            Face::new(
                Coord::new(x, y + 1, z),
                Coord::new(x + 1, y + 1, z),
                Coord::new(x, y + 1, z + 1),
                Coord::new(x + 1, y + 1, z + 1),
            ),
            Face::new(
                Coord::new(x, y, z + 1),
                Coord::new(x + 1, y + 1, z + 1),
                Coord::new(x + 1, y, z + 1),
                Coord::new(x, y + 1, z + 1),
            ),
        ]
    }
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vs: Vec<&str> = s.split(',').collect();
        Ok(Self {
            x: vs[0].parse().unwrap(),
            y: vs[1].parse().unwrap(),
            z: vs[2].parse().unwrap(),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Face {
    a: Coord,
    b: Coord,
    c: Coord,
    d: Coord,
}

impl Face {
    fn new(a: Coord, b: Coord, c: Coord, d: Coord) -> Self {
        let mut v = vec![a, b, c, d];
        v.sort();
        Self {
            a: v[0],
            b: v[1],
            c: v[2],
            d: v[3],
        }
    }
}

type Input = Vec<Coord>;

fn parse(input: String) -> Input {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: Input) -> usize {
    let mut h: HashMap<Face, usize> = HashMap::new();
    for p in input {
        for side in p.sides() {
            *h.entry(side).or_insert(0) += 1;
        }
    }
    h.values().filter(|v| **v == 1).count()
}

fn min_max(vs: &Input, f: &dyn Fn(Coord) -> u8) -> (u8, u8) {
    let (mut max, mut min) = (f(vs[0]), f(vs[0]));
    for v in vs {
        let x = f(*v);
        if x > max {max = x};
        if x < min {min = x};
    }
    (min, max)
}

fn part_2(input: Input) -> usize {
    let (min_x, max_x) = min_max(&input, &|p| p.x);
    let (min_y, max_y) = min_max(&input, &|p| p.y);
    let (min_z, max_z) = min_max(&input, &|p| p.z);
    let mut points: HashSet<Coord> = input.into_iter().collect();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                let c = Coord::new(x, y, z);
                if points.contains(&c) {
                    continue;
                }

                let input: Vec<&Coord> = points
                    .iter()
                    .filter(|p| (p.x == x && (p.y == y || p.z == z)) || (p.y == y && p.z == z))
                    .collect();

                let mut internal = true;
                internal &= input.iter().any(|p| p.x == x && p.y == y && p.z < z);
                internal &= input.iter().any(|p| p.x == x && p.y == y && p.z > z);
                internal &= input.iter().any(|p| p.x == x && p.y < y && p.z == z);
                internal &= input.iter().any(|p| p.x == x && p.y > y && p.z == z);
                internal &= input.iter().any(|p| p.x < x && p.y == y && p.z == z);
                internal &= input.iter().any(|p| p.x > x && p.y == y && p.z == z);

                if internal {
                    points.insert(c);
                }
            }
        }
    }

    part_1(points.into_iter().collect())
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
    let elapsed_time = now.elapsed();
    println!("Running parsing took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(lines.clone()));
    let elapsed_time = now.elapsed();
    println!("Running part_1 took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(lines));
    println!("Running part_2 took {}ms.", now.elapsed().as_millis());
}
