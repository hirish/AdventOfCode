use aoc_19::read_stdin;

use std::collections::HashMap;
use std::time::Instant;

const FACE_SIZE: isize = 50;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn rotate_r(self) -> Self {
        Self::new(FACE_SIZE - self.y - 1, self.x)
    }

    fn rotate_l(self) -> Self {
        Self::new(self.y, FACE_SIZE - self.x - 1)
    }

    fn flip_y(self) -> Self {
        Self::new(FACE_SIZE - self.x - 1, self.y)
    }

    fn flip_x(self) -> Self {
        Self::new(self.x, FACE_SIZE - self.y - 1)
    }

    fn get_face(&self) -> Face {
        let x = self.x / FACE_SIZE;
        let y = self.y / FACE_SIZE;
        match (x, y) {
            (1, 0) => Face::One,
            (2, 0) => Face::Two,
            (0, 2) => Face::Three,
            (1, 1) => Face::Four,
            (1, 2) => Face::Five,
            (0, 3) => Face::Six,
            _ => panic!("Can't find facing for point {:?}", self),
        }
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

#[derive(Clone, Debug, Copy)]
enum Direction {
    Move(usize),
    L,
    R,
}

#[derive(Clone, Debug, Copy)]
enum Facing {
    U,
    R,
    D,
    L,
}

impl Facing {
    fn turn_left(&self) -> Self {
        match self {
            Facing::U => Facing::L,
            Facing::R => Facing::U,
            Facing::D => Facing::R,
            Facing::L => Facing::D,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Facing::U => Facing::R,
            Facing::R => Facing::D,
            Facing::D => Facing::L,
            Facing::L => Facing::U,
        }
    }

    fn val(&self) -> isize {
        match self {
            Facing::R => 0,
            Facing::D => 1,
            Facing::L => 2,
            Facing::U => 3,
        }
    }
}

type Map = HashMap<Coord, char>;

#[derive(Clone, Debug)]
struct Input {
    map: Map,
    directions: Vec<Direction>,
}

fn parse(input: &str) -> Input {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let map: Map = map
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .map(|(x, c)| (Coord::new(x as isize, y as isize), c))
                .collect::<Vec<(Coord, char)>>()
        })
        .collect();

    let directions = directions
        .replace('R', " R ")
        .replace('L', " L ")
        .trim()
        .split(' ')
        .map(|d| match d {
            "R" => Direction::R,
            "L" => Direction::L,
            d => Direction::Move(d.parse().unwrap()),
        })
        .collect();

    Input { map, directions }
}

fn new_pos(mut pos: Coord, dist: usize, facing: Facing, map: &Map) -> Coord {
    for _ in 0..dist {
        let mut new_pos = match facing {
            Facing::U => Coord::new(pos.x, pos.y - 1),
            Facing::R => Coord::new(pos.x + 1, pos.y),
            Facing::D => Coord::new(pos.x, pos.y + 1),
            Facing::L => Coord::new(pos.x - 1, pos.y),
        };

        if !map.contains_key(&new_pos) {
            match facing {
                Facing::U => {
                    new_pos.y = map
                        .iter()
                        .filter_map(|p| if p.0.x == pos.x { Some(p.0.y) } else { None })
                        .max()
                        .unwrap();
                }
                Facing::R => {
                    new_pos.x = map
                        .iter()
                        .filter_map(|p| if p.0.y == pos.y { Some(p.0.x) } else { None })
                        .min()
                        .unwrap();
                }
                Facing::D => {
                    new_pos.y = map
                        .iter()
                        .filter_map(|p| if p.0.x == pos.x { Some(p.0.y) } else { None })
                        .min()
                        .unwrap();
                }
                Facing::L => {
                    new_pos.x = map
                        .iter()
                        .filter_map(|p| if p.0.y == pos.y { Some(p.0.x) } else { None })
                        .max()
                        .unwrap();
                }
            }
        }

        if map[&new_pos] == '.' {
            pos = new_pos
        } else {
            break;
        }
    }

    pos
}

fn part_1(Input { map, directions }: Input) -> isize {
    let mut pos = Coord {
        x: map
            .iter()
            .filter(|(p, c)| p.y == 0 && **c == '.')
            .map(|(p, _)| p.x)
            .min()
            .unwrap(),
        y: 0,
    };

    let mut facing = Facing::R;

    for direction in directions {
        match direction {
            Direction::Move(dist) => pos = new_pos(pos, dist, facing, &map),
            Direction::L => facing = facing.turn_left(),
            Direction::R => facing = facing.turn_right(),
        }
    }

    1000 * (pos.y + 1) + 4 * (pos.x + 1) + facing.val()
}

#[derive(Clone, Debug, Copy)]
enum Face {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Face {
    fn relative(&self) -> Coord {
        match self {
            Face::One => Coord::new(FACE_SIZE, 0),
            Face::Two => Coord::new(2 * FACE_SIZE, 0),
            Face::Three => Coord::new(0, 2 * FACE_SIZE),
            Face::Four => Coord::new(FACE_SIZE, FACE_SIZE),
            Face::Five => Coord::new(FACE_SIZE, 2 * FACE_SIZE),
            Face::Six => Coord::new(0, 3 * FACE_SIZE),
        }
    }

    fn wrap_up(&self, pos: Coord) -> (Coord, Facing) {
        match self {
            Face::One => (
                (pos - self.relative()).flip_y().rotate_l() + Face::Six.relative(),
                Facing::R,
            ),
            Face::Two => (
                (pos - self.relative()).flip_x() + Face::Six.relative(),
                Facing::U,
            ),
            Face::Three => (
                (pos - self.relative()).flip_y().rotate_l() + Face::Four.relative(),
                Facing::R,
            ),
            Face::Four => panic!("Can't exit top of {:?}", self),
            Face::Five => panic!("Can't exit top of {:?}", self),
            Face::Six => panic!("Can't exit top of {:?}", self),
        }
    }

    fn wrap_right(&self, pos: Coord) -> (Coord, Facing) {
        match self {
            Face::One => panic!("Can't exit right of {:?}", self),
            Face::Two => (
                (pos - self.relative()).flip_x() + Face::Five.relative(),
                Facing::L,
            ),
            Face::Three => panic!("Can't exit right of {:?}", self),
            Face::Four => (
                (pos - self.relative()).flip_x().rotate_r() + Face::Two.relative(),
                Facing::U,
            ),
            Face::Five => (
                (pos - self.relative()).flip_x() + Face::Two.relative(),
                Facing::L,
            ),
            Face::Six => (
                (pos - self.relative()).flip_x().rotate_r() + Face::Five.relative(),
                Facing::U,
            ),
        }
    }

    fn wrap_down(&self, pos: Coord) -> (Coord, Facing) {
        match self {
            Face::One => panic!("Can't exit bottom of {:?}", self),
            Face::Two => (
                (pos - self.relative()).flip_y().rotate_l() + Face::Four.relative(),
                Facing::L,
            ),
            Face::Three => panic!("Can't exit bottom of {:?}", self),
            Face::Four => panic!("Can't exit bottom of {:?}", self),
            Face::Five => (
                (pos - self.relative()).flip_y().rotate_l() + Face::Six.relative(),
                Facing::L,
            ),
            Face::Six => (
                (pos - self.relative()).flip_x() + Face::Two.relative(),
                Facing::D,
            ),
        }
    }

    fn wrap_left(&self, pos: Coord) -> (Coord, Facing) {
        match self {
            Face::One => (
                (pos - self.relative()).flip_x() + Face::Three.relative(),
                Facing::R,
            ),
            Face::Two => panic!("Can't exit left of {:?}", self),
            Face::Three => (
                (pos - self.relative()).flip_x() + Face::One.relative(),
                Facing::R,
            ),
            Face::Four => (
                (pos - self.relative()).flip_y().rotate_l() + Face::Three.relative(),
                Facing::D,
            ),
            Face::Five => panic!("Can't exit left of {:?}", self),
            Face::Six => (
                (pos - self.relative()).flip_y().rotate_l() + Face::One.relative(),
                Facing::D,
            ),
        }
    }
}

fn new_pos_2(mut pos: Coord, dist: usize, mut facing: Facing, map: &Map) -> (Coord, Facing) {
    for _ in 0..dist {
        let (mut new_pos, mut new_facing) = match facing {
            Facing::U => (Coord::new(pos.x, pos.y - 1), facing),
            Facing::R => (Coord::new(pos.x + 1, pos.y), facing),
            Facing::D => (Coord::new(pos.x, pos.y + 1), facing),
            Facing::L => (Coord::new(pos.x - 1, pos.y), facing),
        };

        if !map.contains_key(&new_pos) {
            (new_pos, new_facing) = match facing {
                Facing::U => pos.get_face().wrap_up(pos),
                Facing::R => pos.get_face().wrap_right(pos),
                Facing::D => pos.get_face().wrap_down(pos),
                Facing::L => pos.get_face().wrap_left(pos),
            };
        }

        if map[&new_pos] == '.' {
            pos = new_pos;
            facing = new_facing;
        } else {
            break;
        }
    }

    (pos, facing)
}

fn part_2(Input { map, directions }: Input) -> isize {
    let mut pos = Coord {
        x: map
            .iter()
            .filter(|(p, c)| p.y == 0 && **c == '.')
            .map(|(p, _)| p.x)
            .min()
            .unwrap(),
        y: 0,
    };

    let mut facing = Facing::R;

    for direction in directions {
        match direction {
            Direction::Move(dist) => (pos, facing) = new_pos_2(pos, dist, facing, &map),
            Direction::L => facing = facing.turn_left(),
            Direction::R => facing = facing.turn_right(),
        }
    }

    1000 * (pos.y + 1) + 4 * (pos.x + 1) + facing.val()
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let lines = parse(&input);
    let elapsed_time = now.elapsed();
    println!("Running parsing took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(lines.clone()));
    let elapsed_time = now.elapsed();
    println!("Running part_1 took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(lines));
    println!("Running part_2 took {}ms.", now.elapsed().as_millis());
}
