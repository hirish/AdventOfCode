use std::io::{self, Read};

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer.to_string()
}

pub fn read_stdin_numbers() -> Vec<u32> {
    read_stdin()
        .split('\n')
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u32>>()
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn translate(&self, dx: isize, dy: isize) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
