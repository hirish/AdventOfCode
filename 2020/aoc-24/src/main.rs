use aoc::read_stdin;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::W => Position  {x: self.x + 1, y: self.y},
            Direction::E => Position  {x: self.x - 1, y: self.y},
            Direction::NW => Position {x: self.x,     y: self.y + 1},
            Direction::NE => Position {x: self.x - 1, y: self.y + 1},
            Direction::SW => Position {x: self.x + 1, y: self.y - 1},
            Direction::SE => Position {x: self.x,     y: self.y - 1},
        }
    }

    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            self.go(Direction::W),
            self.go(Direction::E),
            self.go(Direction::NW),
            self.go(Direction::NE),
            self.go(Direction::SW),
            self.go(Direction::SE),
        ]
    }
}

#[derive(Debug)]
enum Direction {
    E,
    W,
    NW,
    NE,
    SE,
    SW,
}

fn parse_line(line: &str) -> Vec<Direction> {
    if line.len() == 0 {
        return Vec::new();
    }

    let (direction, line) = if let Some(l) = line.strip_prefix("se") {
        (Direction::SE, l)
    } else if let Some(l) = line.strip_prefix("sw") {
        (Direction::SW, l)
    } else if let Some(l) = line.strip_prefix("ne") {
        (Direction::NE, l)
    } else if let Some(l) = line.strip_prefix("nw") {
        (Direction::NW, l)
    } else if let Some(l) = line.strip_prefix("e") {
        (Direction::E, l)
    } else if let Some(l) = line.strip_prefix("w") {
        (Direction::W, l)
    } else {
        panic!("Unknown prefix!");
    };

    let mut curr = vec![direction];
    curr.append(&mut parse_line(line));
    curr
}

fn follow_directions(line: Vec<Direction>) -> Position {
    let mut pos = Position {x: 0, y: 0};
    for direction in line {
        pos = pos.go(direction);
    }
    pos
}

#[derive(Debug, PartialEq)]
enum Colour {
    Black,
    White,
}

struct Floor {
    pub black_tiles: HashSet<Position>,
}

impl Floor {
    pub fn step(&mut self) {
        let mut new_black_tiles: HashSet<Position> = HashSet::new();

        let mut positions: HashSet<Position> = HashSet::new();
        for position in self.black_tiles.iter() {
            for new_pos in position.neighbours() {
                positions.insert(new_pos);
            }
        }

        for position in positions.into_iter() {
            let colour = if self.black_tiles.contains(&position) {Colour::Black} else {Colour::White};
            let black_neighbours = position
                .neighbours()
                .iter()
                .filter(|n| self.black_tiles.contains(n))
                .count();

            let is_black = match colour {
                Colour::Black => black_neighbours == 1 || black_neighbours == 2,
                Colour::White => black_neighbours == 2,
            };

            if is_black {
                new_black_tiles.insert(position);
            }
        }

        self.black_tiles = new_black_tiles;
    }
}

fn make_floor(input: &str) -> Floor {
    let lines: Vec<Vec<Direction>> = input.lines().map(parse_line).collect();
    let flipped_positions: Vec<Position> = lines.into_iter().map(follow_directions).collect();

    let mut black_tiles: HashSet<Position> = HashSet::new();
    for position in flipped_positions.into_iter() {
        if black_tiles.contains(&position) {
            black_tiles.remove(&position);
        } else {
            black_tiles.insert(position);
        }
    }
    
    Floor {black_tiles}
}

fn part_1(input: &str) -> usize {
    let floor = make_floor(input);
    floor.black_tiles.len()
}


fn part_2(input: &str) -> usize {
    let mut floor = make_floor(input);
    for _ in 0..100 {
        floor.step();
    }
    floor.black_tiles.len()
}


fn main() {
    let input = read_stdin();
    println!("Answer 1: {}", part_1(&input));
    println!("Answer 2: {}", part_2(&input));
}
