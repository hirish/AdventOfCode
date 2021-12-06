use std::char::from_digit;
use std::str::FromStr;
use std::collections::HashMap;

use aoc_1::read_stdin;


#[derive(Debug, Clone, Hash, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let x = input.split(",").nth(0).unwrap().parse().unwrap();
        let y = input.split(",").nth(1).unwrap().parse().unwrap();
        Ok(Self {x, y})
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


#[derive(Debug, Clone)]
struct Line {
    from: Point,
    to: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let from = input.split(" -> ").nth(0).unwrap().parse().unwrap();
        let to = input.split(" -> ").nth(1).unwrap().parse().unwrap();
        
        Ok(Self {from, to})
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }

    fn points(&self) -> Vec<Point> {
        let dx: isize = self.to.x - self.from.x;
        let dy: isize = self.to.y - self.from.y;

        let gx = if dx == 0 {0} else if dx > 0 {1} else {-1};
        let gy = if dy == 0 {0} else if dy > 0 {1} else {-1};

        let mut points = vec![self.from.clone()];
        let mut p = self.from.clone();
        while !(p.x == self.to.x && p.y == self.to.y) {
            p.x += gx;
            p.y += gy;
            points.push(p.clone())
        }

        points
    }
}


fn parse(input: String) -> Vec<Line> {
    input
        .split("\n")
        .map(|l| l.parse().unwrap())
        .collect()
}


fn print_field(field: &HashMap<Point, usize>) {
    for y in 0..10 {
        for x in 0..10 {
            let p = Point{x, y};
            let v = field.get(&p).unwrap_or(&0);
            let c = from_digit(*v as u32, 10).unwrap();
            print!("{}", if c == '0' {'.'} else {c})
        }
        println!("");
    }
}


fn part_1(input: Vec<Line>) -> usize {
    let mut field: HashMap<Point, usize> = HashMap::new();
    for line in input {
        if line.is_straight() {
            for p in line.points() {
                *field.entry(p).or_insert(0) += 1
            }
        }
    }

    field
        .values()
        .filter(|v| **v >= 2)
        .count()
}

fn part_2(input: Vec<Line>) -> usize {
    let mut field: HashMap<Point, usize> = HashMap::new();
    for line in input {
        for p in line.points() {
            *field.entry(p).or_insert(0) += 1
        }
    }

    field
        .values()
        .filter(|v| **v >= 2)
        .count()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
