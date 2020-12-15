use std::io::{self, Read};

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn read_stdin_numbers() -> Vec<u32> {
    read_stdin()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn add(&mut self, dx: isize, dy: isize) {
        self.x = self.x + dx;
        self.y  = self.y + dy;
    }

    pub fn distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

#[derive(Debug)]
pub struct State1 {
    pub direction: Direction,
    pub pos: Point,
}

impl State1 {
    pub fn apply(&mut self, x: &str) {
        let mut op = x.chars().nth(0).unwrap();
        let mut val: isize = x[1..].parse().unwrap();

        if op == 'F' {
            op = match self.direction {
                Direction::North => 'N',
                Direction::East => 'E',
                Direction::South => 'S',
                Direction::West => 'W',
            }
        }

        if op == 'L' {
            op = 'R';
            val = 360 - val;
        }

        match op {
            'N' => {self.pos.add(0, val);},
            'S' => {self.pos.add(0, -val);},
            'E' => {self.pos.add(val, 0);},
            'W' => {self.pos.add(-val, 0);},
            'R' => {
                let turns = val / 90;
                for _ in 0..turns {
                    self.direction = self.direction.right()
                }
            },
            _ => panic!("Unknown operation")
        }
    }
}

#[derive(Debug)]
pub struct State2 {
    pub waypoint: Point,
    pub pos: Point,
}

impl State2 {
    pub fn apply(&mut self, x: &str) {
        let mut op = x.chars().nth(0).unwrap();
        let mut val: isize = x[1..].parse().unwrap();

        if op == 'L' {
            op = 'R';
            val = 360 - val;
        }

        match op {
            'F' => {
                let dx = self.waypoint.x * val;
                let dy = self.waypoint.y * val;
                self.pos.add(dx, dy);
            }
            'N' => {self.waypoint.add(0, val);},
            'S' => {self.waypoint.add(0, -val);},
            'E' => {self.waypoint.add(val, 0);},
            'W' => {self.waypoint.add(-val, 0);},
            'R' => {
                let turns = val / 90;
                for _ in 0..turns {
                    self.rotate_right()
                }
            },
            _ => panic!("Unknown operation")
        }
    }

    pub fn rotate_right(&mut self) {
        self.waypoint = Point {
            x: self.waypoint.y,
            y: -self.waypoint.x,
        }
    }
}
