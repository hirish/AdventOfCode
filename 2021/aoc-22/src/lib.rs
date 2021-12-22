use std::io::{self, Read};
use std::time::Instant;
use std::fmt;

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn read_stdin_numbers(splitter: &str) -> Vec<usize> {
    read_stdin()
        .split(splitter)
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

pub fn duration(start: Instant) -> String {
    let d = start.elapsed();
    if d.as_micros() <= 1000 {
        format!("{}μs", d.as_micros())
    } else if d.as_millis() <= 1000 {
        format!("{}ms", d.as_millis())
    } else {
        format!("{}s", d.as_secs())
    }
}

#[derive(Clone, Copy, Hash, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
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
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn rotate_x(&self) -> Self {
        Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    pub fn rotate_y(&self) -> Self {
        Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    pub fn rotate_z(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    pub fn mag(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn euclid(&self, other: &Self) -> isize {
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
