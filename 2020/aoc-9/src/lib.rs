use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

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
pub struct Map {
    grid: HashMap<usize, HashMap<usize, char>>,
    wrap_x: bool,
    wrap_y: bool,
}

impl Map {
    pub fn get(&self, y: usize, x: usize) -> char {
        let x = if self.wrap_x { x % self.width() } else { x };
        let y = if self.wrap_y { y % self.height() } else { y };
        *self.grid.get(&y).unwrap().get(&x).unwrap()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.grid.get(&0).unwrap().len()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            grid: input
                .lines()
                .enumerate()
                .map(|(y, line)| (y, line.chars().enumerate().collect()))
                .collect(),
            wrap_x: true,
            wrap_y: false,
        })
    }
}
