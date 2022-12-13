use aoc_12::read_stdin;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Input {
    heights: HashMap<Point, isize>,
    start: Point,
    dest: Point,
}

impl Input {
    fn neighbours(&self, p: Point) -> Vec<Point> {
        let Point { x, y } = p;
        vec![
            Point { x: x - 1, y },
            Point { x: x + 1, y },
            Point { x, y: y - 1 },
            Point { x, y: y + 1 },
        ]
        .into_iter()
        .filter(|q| self.heights.get(q).is_some())
        .filter(|q| self.heights.get(&p).unwrap() - self.heights.get(q).unwrap() <= 1)
        .collect()
    }
}

fn parse(input: String) -> Input {
    let mut heights = HashMap::new();
    let mut start = None;
    let mut dest = None;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let p = Point {
                x: x as isize,
                y: y as isize,
            };
            let height = if char == 'S' {
                dest = Some(p);
                0
            } else if char == 'E' {
                start = Some(p);
                25
            } else {
                (char as isize) - 97
            };

            heights.insert(p, height);
        }
    }

    Input {
        heights,
        start: start.unwrap(),
        dest: dest.unwrap(),
    }
}

fn dijkstra(input: &Input) -> HashMap<Point, usize> {
    let mut distances: HashMap<Point, usize> = HashMap::new();
    let mut visited = HashMap::new();
    let mut unvisited: HashSet<Point> = HashSet::new();

    distances.insert(input.start, 0);
    unvisited.insert(input.start);

    while !unvisited.is_empty() {
        let (&p, &d) = distances.iter().min_by_key(|(_, d)| *d).unwrap();
        unvisited.remove(&p);
        distances.remove(&p);
        visited.insert(p, d);

        for neighbour in input.neighbours(p) {
            if visited.contains_key(&neighbour) {
                continue;
            }

            if let Some(&e) = distances.get(&neighbour) {
                if d + 1 < e {
                    distances.insert(neighbour, d + 1);
                }
            } else {
                unvisited.insert(neighbour);
                distances.insert(neighbour, d + 1);
            }
        }
    }

    visited
}

fn part_1(input: Input) -> usize {
    *dijkstra(&input).get(&input.dest).unwrap()
}

fn part_2(input: Input) -> usize {
    let visited = dijkstra(&input);
    *input.heights
        .keys()
        .filter(|p| *input.heights.get(p).unwrap() == 0)
        .filter(|p| visited.contains_key(p))
        .map(|p| visited.get(p).unwrap())
        .min()
        .unwrap()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
