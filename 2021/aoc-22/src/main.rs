use std::collections::HashSet;
use std::str::FromStr;
use std::time::Instant;

use aoc_1::{duration, read_stdin, Point};

fn max(a: isize, b: isize) -> isize {
    if a > b {a} else {b}
}

fn min(a: isize, b: isize) -> isize {
    if a < b {a} else {b}
}

#[derive(Clone, Debug, Hash, Eq, Copy)]
struct Cuboid {
    from: Point,
    to: Point,
}

impl PartialEq for Cuboid {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, b) = input.split_once(',').unwrap();
        let (y, z) = b.split_once(',').unwrap();

        let x = &x[2..];
        let y = &y[2..];
        let z = &z[2..];

        let (ax, bx) = x.split_once("..").unwrap();
        let (ay, by) = y.split_once("..").unwrap();
        let (az, bz) = z.split_once("..").unwrap();

        let ax = ax.parse().unwrap();
        let ay = ay.parse().unwrap();
        let az = az.parse().unwrap();
        let bx = bx.parse().unwrap();
        let by = by.parse().unwrap();
        let bz = bz.parse().unwrap();
        
        let from = Point::new(min(ax, bx), min(ay, by), min(az, bz));
        let to = Point::new(max(ax, bx), max(ay, by), max(az, bz));

        Ok(Cuboid { from, to })
    }
}

impl Cuboid {
    fn new(from: Point, to: Point) -> Option<Self> {
        if from.x > to.x { return None }
        if from.y > to.y { return None }
        if from.z > to.z { return None }
        Some(Self{from, to})
    }

    fn intersects(&self, other: &Self) -> bool {
        !self.intersection(other).is_none()
    }

    fn intersection(&self, other: &Self) -> Option<Cuboid> {
        Self::new(
            Point::new(
                max(self.from.x, other.from.x),
                max(self.from.y, other.from.y),
                max(self.from.z, other.from.z),
            ),
            Point::new(
                min(self.to.x, other.to.x),
                min(self.to.y, other.to.y),
                min(self.to.z, other.to.z),
            )
        )
    }

    fn split_around(&self, other: &Self) -> Vec<Cuboid> {
        let int = self.intersection(other).unwrap();

        let front = Cuboid::new(
            self.from,
            Point::new(self.to.x, self.to.y, int.from.z - 1),
        );

        let back = Cuboid::new(
            Point::new(self.from.x, self.from.y, int.to.z + 1),
            self.to,
        );

        let left = Cuboid::new(
            Point::new(self.from.x, self.from.y, int.from.z),
            Point::new(other.from.x-1, self.to.y, int.to.z)
        );

        let right = Cuboid::new(
            Point::new(other.to.x+1, self.from.y, int.from.z),
            Point::new(self.to.x, self.to.y, int.to.z)
        );

        let top = Cuboid::new(
            Point::new(int.from.x, self.from.y, int.from.z),
            Point::new(int.to.x, other.from.y-1, int.to.z)
        );

        let bottom = Cuboid::new(
            Point::new(int.from.x, other.to.y+1, int.from.z),
            Point::new(int.to.x, self.to.y, int.to.z)
        );

        let out = vec![front, back, left, right, top, bottom];
        out.into_iter().filter_map(|x| x).collect()
    }

    fn area(&self) -> isize {
        (1 + self.to.x - self.from.x) * (1 + self.to.y - self.from.y) * (1 + self.to.z - self.from.z)
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    command: bool,
    cuboid: Cuboid
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (a, b) = input.split_once(" ").unwrap();
        Ok(Instruction {
            command: a == "on",
            cuboid: b.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
struct Input {
    input: Vec<Instruction>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Input{
            input: input.lines().map(|l| l.parse().unwrap()).collect()
        })
    }
}

fn part_1(input: Input) -> Option<usize> {
    let mut on: HashSet<Point> = HashSet::new();

    for Instruction {command, cuboid} in input.input {
        let from_x = max(cuboid.from.x, -50);
        let from_y = max(cuboid.from.y, -50);
        let from_z = max(cuboid.from.z, -50);

        let to_x = min(cuboid.to.x, 50);
        let to_y = min(cuboid.to.y, 50);
        let to_z = min(cuboid.to.z, 50);

        for x in from_x..to_x+1 {
            for y in from_y..to_y+1 {
                for z in from_z..to_z+1 {
                    if command {
                        on.insert(Point::new(x, y, z));
                    } else {
                        on.remove(&Point::new(x, y, z));
                    }
                }
            }
        }
    }

    Some(on.len())
}

fn part_2(input: Input) -> Option<isize> {
    let mut on: HashSet<Cuboid> = HashSet::new();

    for Instruction {command, cuboid} in input.input {
        let intersects: Vec<Cuboid> = on
            .iter()
            .filter(|r| cuboid.intersects(r))
            .map(|r| *r)
            .collect();

        for other in intersects {
            on.remove(&other);
            for r in other.split_around(&cuboid) {
                on.insert(r);
            }
        }

        if command {
            on.insert(cuboid);
        }
    }
    
    Some(on.iter().map(|r| r.area()).sum())
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
