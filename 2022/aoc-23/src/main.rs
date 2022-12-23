use aoc_19::{read_stdin, Coord};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::time::Instant;

type Input = FxHashSet<Coord>;

lazy_static::lazy_static!(
    static ref DIRECTIONS: [[Coord; 3]; 4] = [
        [Coord::new(-1, -1), Coord::new(0, -1), Coord::new(1, -1)],
        [Coord::new(-1, 1), Coord::new(0, 1), Coord::new(1, 1)],
        [Coord::new(-1, -1), Coord::new(-1, 0), Coord::new(-1, 1)],
        [Coord::new(1, -1), Coord::new(1, 0), Coord::new(1, 1)],
    ];
    static ref SURROUNDINGS: [Coord; 8] = [
        Coord::new(-1, -1), Coord::new(0, -1), Coord::new(1, -1),
        Coord::new(-1, 1), Coord::new(0, 1), Coord::new(1, 1),
        Coord::new(-1, 0), Coord::new(1, 0)
    ];
);

fn parse(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Coord::new(x as isize, y as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Coord>>()
        })
        .collect()
}

fn round(input: &mut Input, i: usize) -> bool {
    let mut proposed_moves: FxHashMap<Coord, Vec<Coord>> = FxHashMap::default();

    for &c in input.iter() {
        if SURROUNDINGS.iter().all(|&s| !input.contains(&(c + s))) {
            continue;
        }

        for j in 0..4 {
            let direction = DIRECTIONS[(i + j) % 4];
            let proposal = [c + direction[0], c + direction[1], c + direction[2]];
            if !proposal.iter().any(|p| input.contains(p)) {
                proposed_moves.entry(proposal[1]).or_default().push(c);
                break;
            }
        }
    }

    // input = input.clone();
    let mut changed = false;
    for (m, fs) in proposed_moves {
        if fs.len() == 1 {
            changed = true;
            input.remove(&fs[0]);
            input.insert(m);
        }
    }

    changed
}

fn part_1(mut input: Input) -> isize {
    for i in 0..10 {
        round(&mut input, i);
    }

    let (min_x, max_x) = input.iter().map(|c| c.x).minmax().into_option().unwrap();
    let (min_y, max_y) = input.iter().map(|c| c.y).minmax().into_option().unwrap();
    (1 + max_y - min_y) * (1 + max_x - min_x) - (input.len() as isize)
}

fn part_2(mut input: Input) -> usize {
    let mut i = 0;
    loop {
        if !round(&mut input, i) {
            return i + 1;
        }
        i += 1;
    }
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
