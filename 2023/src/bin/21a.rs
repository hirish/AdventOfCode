use aoc2024::read_stdin;
use std::{collections::HashSet, time::Instant};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord(usize, usize);

impl Coord {
    fn neighbours(&self) -> [Self; 4] {
        [
            Coord(self.0 - 1, self.1),
            Coord(self.0 + 1, self.1),
            Coord(self.0, self.1 - 1),
            Coord(self.0, self.1 + 1),
        ]
    }
}

type Graph = HashSet<Coord>;
type Input = (Coord, Graph);

fn parse(input: String) -> Input {
    let start = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                'S' => Some(Coord(x, y)),
                _ => None,
            })
        })
        .next()
        .unwrap();

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Coord(x, y)),
                _ => None,
            })
        })
        .collect();

    (start, map)
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let (spos, map) = parse(input.clone());
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let mut queue: HashSet<Coord> = HashSet::default();
    queue.insert(spos);
    for _ in 0..64 {
        queue = queue
            .iter()
            .flat_map(|p| {
                p.neighbours()
                    .into_iter()
                    .filter_map(|n| if map.contains(&n) { None } else { Some(n) })
            })
            .collect();
    }

    let v = queue.len();

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
