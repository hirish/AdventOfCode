use aoc_9::read_stdin;

use std::collections::HashSet;

#[derive(Clone)]
enum Direction {
    U,
    R,
    D,
    L,
}

type Instruction = (Direction, isize);
type Point = (isize, isize);

type Input = Vec<Instruction>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| match l.split_once(" ").unwrap() {
            ("U", x) => (Direction::U, x.parse().unwrap()),
            ("R", x) => (Direction::R, x.parse().unwrap()),
            ("D", x) => (Direction::D, x.parse().unwrap()),
            ("L", x) => (Direction::L, x.parse().unwrap()),
            _ => panic!("Failed to parse {}", l),
        })
        .collect()
}

fn add((a, b): Point, (x, y): Point) -> Point {
    (a + x, b + y)
}

fn distance(a: Point, b: Point) -> isize {
    (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f32).sqrt() as isize
}

fn solve(input: Input) -> Vec<HashSet<Point>> {
    let mut positions = vec![(0, 0); 10];
    let mut visited = vec![HashSet::new(); 10];

    for (direction, size) in input {
        for _ in 0..size {
            match direction {
                Direction::U => positions[0] = add(positions[0], (1, 0)),
                Direction::D => positions[0] = add(positions[0], (-1, 0)),
                Direction::L => positions[0] = add(positions[0], (0, 1)),
                Direction::R => positions[0] = add(positions[0], (0, -1)),
            }

            for tail in 1..positions.len() {
                let head = tail - 1;
                if distance(positions[head], positions[tail]) >= 2 {
                    positions[tail] = add(
                        positions[tail],
                        (
                            positions[head].0.cmp(&positions[tail].0) as isize,
                            positions[head].1.cmp(&positions[tail].1) as isize,
                        ),
                    )
                }
            }

            for i in 0..visited.len() {
                visited[i].insert(positions[i]);
            }
        }
    }

    visited
}

fn part_1(input: Input) -> usize {
    solve(input)[1].len()
}

fn part_2(input: Input) -> usize {
    solve(input)[9].len()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
