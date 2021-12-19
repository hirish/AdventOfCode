use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use aoc_1::{duration, read_stdin};

#[derive(Clone, Copy, Hash, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn origin() -> Self {
        Self::new(0, 0, 0)
    }

    fn rotate_x(&self) -> Self {
        Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    fn rotate_y(&self) -> Self {
        Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    fn rotate_z(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    fn mag(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn euclid(&self, other: &Self) -> isize {
        return (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self::origin() - self
    }
}

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    points: HashSet<Point>,
    scanner: Point,
    diffs: HashSet<isize>,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let title = &lines.next().unwrap()[11..].trim();
        let id = title.split(' ').next().unwrap().parse().unwrap();

        let points: HashSet<Point> = lines
            .map(|l| {
                let p: Vec<&str> = l.trim().split(',').collect();
                Point::new(
                    p[0].parse().unwrap(),
                    p[1].parse().unwrap(),
                    p[2].parse().unwrap(),
                )
            })
            .collect();

        let diffs = points.iter().map(|&p| {
            points.iter().filter_map(|&q| {
                if p == q {None} else {Some(p.euclid(&q))}
            }).min().unwrap()
        }).collect();

        Ok(Self {
            id,
            points,
            scanner: Point::origin(),
            diffs,
        })
    }
}

impl Tile {
    fn rotate_x(&self) -> Self {
        Self {
            id: self.id,
            points: self.points.iter().map(|p| p.rotate_x()).collect(),
            scanner: self.scanner.rotate_x(),
            diffs: self.diffs.clone(),
        }
    }

    fn rotate_y(&self) -> Self {
        Self {
            id: self.id,
            points: self.points.iter().map(|p| p.rotate_y()).collect(),
            scanner: self.scanner.rotate_y(),
            diffs: self.diffs.clone(),
        }
    }

    fn rotate_z(&self) -> Self {
        Self {
            id: self.id,
            points: self.points.iter().map(|p| p.rotate_z()).collect(),
            scanner: self.scanner.rotate_z(),
            diffs: self.diffs.clone(),
        }
    }

    fn matches(&self, other: &Self) -> bool {
        self.points.intersection(&other.points).count() >= 12
    }

    fn heuristic_matches(&self, other: &Self) -> bool {
        self.diffs.intersection(&other.diffs).count() >= 8
    }

    fn orientations(&self) -> Vec<Self> {
        let mut output = Vec::new();

        let mut v = self.clone();
        for _ in 0..4 {
            v = v.rotate_x();
            output.push(v.clone());
        }

        v = self.rotate_y().rotate_y();
        for _ in 0..4 {
            v = v.rotate_x();
            output.push(v.clone());
        }

        v = self.rotate_y();
        for _ in 0..4 {
            v = v.rotate_z();
            output.push(v.clone());
        }

        v = self.rotate_y().rotate_y().rotate_y();
        for _ in 0..4 {
            v = v.rotate_z();
            output.push(v.clone());
        }

        v = self.rotate_z();
        for _ in 0..4 {
            v = v.rotate_y();
            output.push(v.clone());
        }

        v = self.rotate_z().rotate_z().rotate_z();
        for _ in 0..4 {
            v = v.rotate_y();
            output.push(v.clone());
        }

        output
    }

    fn relative_to(&self, r: Point) -> Self {
        Self {
            id: self.id,
            points: self.points.iter().map(|p| *p - r).collect(),
            scanner: self.scanner - r,
            diffs: self.diffs.clone(),
        }
    }

    fn relatives(&self) -> Vec<(Point, Self)> {
        self.points
            .iter()
            .map(|p| (*p, self.relative_to(*p)))
            .collect()
    }

    fn variations(&self) -> Vec<Self> {
        let mut output = Vec::new();
        for a in self.orientations() {
            for (_, b) in a.relatives() {
                output.push(b);
            }
        }
        output
    }
}

#[derive(Clone, Debug)]
struct Input {
    tiles: Vec<Tile>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tiles = input.split("\n\n").map(|t| t.parse().unwrap()).collect();
        Ok(Self { tiles })
    }
}

fn solve(input: Input) -> HashMap<usize, Tile> {
    let tiles = input.tiles;

    let mut relatives_cache: HashMap<usize, Vec<(Point, Tile)>> = HashMap::new();
    let mut variations_cache: HashMap<usize, Vec<Tile>> = HashMap::new();
    for tile in tiles.iter() {
        variations_cache.insert(tile.id, tile.variations());
    }

    let mut searched: HashSet<usize> = HashSet::new();
    let mut orientations: HashMap<usize, Tile> = HashMap::new();
    orientations.insert(tiles[0].id, tiles[0].clone());

    while orientations.len() < tiles.len() {
        let now = Instant::now();
        let target = tiles
            .iter()
            .filter(|t| !searched.contains(&t.id) && orientations.contains_key(&t.id))
            .next()
            .unwrap();
        searched.insert(target.id);

        relatives_cache
            .entry(target.id)
            .or_insert_with(|| orientations[&target.id].relatives());

        let mut i = 0;
        for tile in tiles.iter() {
            if tile.id == target.id {
                continue;
            }
            if orientations.contains_key(&tile.id) {
                continue;
            }
            // if !tile.heuristic_matches(target) {
            //     continue;
            // }
            
            i += 1;

            for (tx, oriented_target) in relatives_cache[&target.id].iter() {
                for t in variations_cache[&tile.id].iter() {
                    if !t.matches(&oriented_target) {
                        continue;
                    }
                    orientations.insert(t.id, t.relative_to(-*tx));
                    break;
                }
            }
        }
        println!("Running tile {} took {}, checked {} tiles.", target.id, duration(now), i);
    }

    orientations
}

fn part_1(input: Input) -> Option<usize> {
    let orientations = solve(input);

    let mut points: HashSet<Point> = HashSet::new();
    for tile in orientations.values() {
        for p in tile.points.iter() {
            points.insert(*p);
        }
    }

    Some(points.len())
}

fn part_2(input: Input) -> Option<usize> {
    let orientations = solve(input);

    let mut max = 0;
    for a in orientations.values() {
        for b in orientations.values() {
            let v = (b.scanner - a.scanner).mag();
            if v > max {
                max = v
            }
        }
    }

    Some(max as usize)
}

fn main() -> Result<(), ()> {
    let now = Instant::now();
    let input: Input = read_stdin().parse()?;
    println!("Running parsing took {}.", duration(now));

    let now = Instant::now();
    println!("Answer 1: {}", part_1(input.clone()).ok_or(())?);
    println!("Running part_1 took {}.", duration(now));

    let now = Instant::now();
    println!("Answer 2: {}", part_2(input).ok_or(())?);
    println!("Running part_2 took {}.", duration(now));

    Ok(())
}
