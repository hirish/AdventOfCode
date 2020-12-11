use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;
use std::cmp::max;

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

pub fn neighbours(Point {x, y}: Point) -> Vec<Point> {
    let mut n = Vec::new();

    let from_y = max((y as isize) - 1, 0) as usize;
    let from_x = max((x as isize) - 1, 0) as usize;

    for y0 in from_y..=y+1 {
        for x0 in from_x..=x+1 {
            if (x0 != x) || (y0 != y) {
                n.push(Point{x: x0, y: y0})
            }
        }
    }

    n
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn add(&self, dx: isize, dy: isize) -> Result<Point, ()> {
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;

        if x >= 0 && y >= 0 {
            Ok(Point{x: x as usize, y: y as usize})
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    grid: HashMap<usize, HashMap<usize, char>>,
    wrap_x: bool,
    wrap_y: bool,
    neighbours_cache: HashMap<Point, Vec<Point>>,
}

impl Map {
    pub fn get(&self, Point {x, y}: Point) -> Option<char> {
        let x = if self.wrap_x { x % self.width() } else { x };
        let y = if self.wrap_y { y % self.height() } else { y };
        self.grid.get(&y)
            .and_then(|y| y.get(&x))
            .map(|x| *x)
    }

    pub fn set(&mut self, Point{ x, y }: Point, v: char) -> Result<(), ()> {
        let row = self.grid.get_mut(&y).ok_or(())?;
        row.insert(x, v);
        Ok(())
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.grid.get(&0).unwrap().len()
    }

    fn get_neighbour(&self, from: &Point, direction: (isize, isize)) -> Option<Point> {
        let (dx, dy) = direction;

        let mut d = 0;
        loop {
            d += 1;
            let p = from.add(dx * d, dy * d);
            let v = p.and_then(|p| self.get(p).ok_or(()));

            if let Ok(v) = v {
                if v == 'L' || v == '#' {
                    return Some(p.unwrap())
                }
            } else {
                return None
            }
        }
    }

    fn get_neighbours(&self, p: &Point) -> Vec<Point> {
        let mut neighbours = Vec::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if (dx == 0) && (dy == 0) {continue}
                if let Some(n) = self.get_neighbour(p, (dx, dy)) {
                    neighbours.push(n)
                }
            }
        }

        neighbours
    }

    pub fn occupied(&self) -> usize {
        let mut c = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get(Point{x, y}).unwrap() == '#' {
                    c += 1
                }
            }
        }
        c
    }

    pub fn display(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!("{}", self.get(Point {x, y}).unwrap());
            }
            println!("");
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut m = Map {
            grid: input
                .lines()
                .enumerate()
                .map(|(y, line)| (y, line.chars().enumerate().collect()))
                .collect(),
            wrap_x: false,
            wrap_y: false,
            neighbours_cache: HashMap::new()
        };

        m.build_cache();

        Ok(m)
    }
}


impl Map {

    fn build_cache(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = Point{x, y};
                self.neighbours_cache.insert(p, self.get_neighbours(&p));
            }
        }
    }

    fn next_value(&self, p: Point, part_2: bool) -> Option<char> {
        let curr = self.get(p).unwrap();
        if curr == '.' {
            return None
        }

        let c = if part_2 {
            self
                .neighbours_cache.get(&p).unwrap().iter()
                .map(|p| self.get(*p).unwrap_or('.'))
                .filter(|c| *c == '#')
                .count()
        } else {
            neighbours(p)
                .iter()
                .map(|p| self.get(*p).unwrap_or('.'))
                .filter(|c| *c == '#')
                .count()
        };

        let limit = if part_2 {4} else {3};

        if c == 0 {Some('#')}
        else if c <= limit {None}
        else {Some('L')}
    }

    pub fn next_grid(&self, part_2: bool) -> (Self, bool) {
        let width = self.width();
        let height = self.height();
        let mut made_change = false;

        let mut next = self.clone();

        for y in 0..height {
            for x in 0..width {
                let p = Point{x, y};
                if let Some(v) = self.next_value(p, part_2){
                    made_change |= v != self.get(Point{x, y}).unwrap();
                    next.set(Point{ x, y }, v).unwrap();
                }
            }
        }

        (next, made_change)
    }
}
