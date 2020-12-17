use aoc::{read_stdin, Ticket, Rule, Constraint};
use std::str::FromStr;
use std:: collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub w: isize,
    pub x: isize,
    pub y: isize, 
    pub z: isize, 
}

impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        let Point{w, x, y, z} = *self;
        let mut neighbours = Vec::new();
        for pw in w-1..=w+1 {
            for px in x-1..=x+1 {
                for py in y-1..=y+1 {
                    for pz in z-1..=z+1 {
                        let n = Point{w:pw, x:px, y:py, z:pz};
                        if n != *self {
                            neighbours.push(n)
                        }
                    }
                }
            }
        }
        neighbours
    }
}

pub struct Map {
    pub map: HashMap<Point, bool>,
    pub w_range: (isize, isize),
    pub x_range: (isize, isize),
    pub y_range: (isize, isize),
    pub z_range: (isize, isize),
}


impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<Point, bool> = HashMap::new();
        let height = input.lines().count() as isize;
        let width = input.lines().next().unwrap().len() as isize;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point { w: 0, x: x as isize, y: y as isize, z:0 };
                map.insert(
                    p,
                    match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Unknown char {}", c),
                    },
                );
            }
        }

        Ok(Map {
            map,
            w_range: (0, 1),
            x_range: (0, width),
            y_range: (0, height),
            z_range: (0,1),
        })
    }
}

impl Map {
    pub fn step(self) -> Self {
        let mut map: HashMap<Point, bool> = HashMap::new();
        let (min_w, max_w) = self.w_range;
        let (min_x, max_x) = self.x_range;
        let (min_y, max_y) = self.y_range;
        let (min_z, max_z) = self.z_range;

        for w in min_w-1..max_w+1 {
            for x in min_x-1..max_x+1 {
                for y in min_y-1..max_y+1 {
                    for z in min_z-1..max_z+1 {
                        let p = Point { w, x, y, z };
                        let v = p
                            .neighbours()
                            .iter()
                            .filter(|n| *self.map.get(&n).unwrap_or(&false))
                            .count();

                        let curr_active = *self.map.get(&p).unwrap_or(&false);
                        let active = v==3 || (curr_active && v == 2);
                        map.insert(p, active);
                    }
                }
            }
        }

        Map {
            map,
            w_range: (min_w - 1, max_w + 1),
            x_range: (min_x - 1, max_x + 1),
            y_range: (min_y - 1, max_y + 1),
            z_range: (min_z - 1, max_z + 1),
        }
    }

    pub fn count_active(&self) -> usize {
        self.map.values().filter(|v| **v).count()
    }
}

fn part_1(input: &str) -> usize {
    let mut map: Map = input.parse().unwrap();
    for _ in 0..6 {
        map = map.step();
    }
    map.count_active()
}

fn part_2(input: &str) -> usize {
    let mut map: Map = input.parse().unwrap();
    for _ in 0..6 {
        map = map.step();
    }
    map.count_active()
}

fn main() {
    let input = read_stdin();

    println!("Answer 1: {}", part_1(&input));
    println!("Answer 2: {}", part_2(&input));
}
