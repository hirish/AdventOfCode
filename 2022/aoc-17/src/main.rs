use aoc_17::read_stdin;

use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SHAPES: Vec<Vec<&'static str>> = vec![
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ];
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    L,
    R,
}
type Input = Vec<Direction>;

type Coord = (usize, usize);

fn parse(input: String) -> Input {
    input
        .chars()
        .map(|c| match c {
            '>' => Direction::R,
            '<' => Direction::L,
            _ => panic!("Unknown character {}", c),
        })
        .collect()
}

fn add_shape(i: usize, input: &Input, map: &mut HashSet<Coord>, mut j: usize, max_y: usize) -> (usize, usize) {
    let shape = &SHAPES[i % SHAPES.len()];

    let mut gas = true;
    let mut pos: Coord = (2, max_y + 2 + shape.len());

    let to_remove: Vec<Coord> = map
        .iter()
        .filter(|v| v.1 + 40 < max_y)
        .map(|&(x, y)| (x, y))
        .collect();
    for c in to_remove {
        map.remove(&c);
    }

    loop {
        if gas {
            let x = match input[j] {
                Direction::R => 1,
                Direction::L => -1,
            } + pos.0 as isize;

            pos = if shape.iter().enumerate().all(|(dy, row)| {
                row.chars().enumerate().all(|(dx, c)| {
                    c == '.'
                        || ((x >= 0)
                            && ((x as usize) + dx <= 6)
                            && !map.contains(&((x as usize) + dx, pos.1 - dy)))
                })
            }) {
                (x as usize, pos.1)
            } else {
                pos
            };

            j = (j + 1) % input.len();
        } else {
            if shape.iter().enumerate().any(|(dy, row)| {
                row.chars()
                    .enumerate()
                    .filter(|v| v.1 == '#')
                    .any(|(dx, c)| {
                        c == '#'
                            && ((pos.1 - dy == 0) || map.contains(&(pos.0 + dx, pos.1 - dy - 1)))
                    })
            }) {
                break;
            };

            pos = (pos.0, pos.1 - 1)
        }

        gas = !gas;
    }

    shape.iter().enumerate().for_each(|(dy, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(dx, _)| {
                map.insert((pos.0 + dx, pos.1 - dy));
            })
    });

    (j, max_y.max(pos.1 + 1))
}

fn part_1(input: Input) -> usize {
    let mut map: HashSet<Coord> = HashSet::new();
    let mut j = 0;
    let mut max_y = 0;
    for i in 0..2022 {
        (j, max_y) = add_shape(i, &input, &mut map, j, max_y);
    }
    max_y
}

fn signature(map: &HashSet<Coord>, max_y: usize) -> Vec<Coord> {
    let mut v: Vec<Coord> = map.iter().filter_map(|&(x, y)| {
        if max_y - y <= 30 {
            Some((x, max_y - y))
        } else { None }
    }).collect();
    v.sort();
    v
}

fn part_2(input: Input) -> usize {
    let mut map: HashSet<Coord> = HashSet::new();
    let mut j = 0;
    let mut positions: HashMap<(usize, usize, Vec<Coord>), (usize, usize)> = HashMap::new();
    let mut max_y = 0;
    let (mut finish, mut from, mut to) = (None, None, None);
    for i in 0..10000 {
        if let Some(v) = finish {
            finish = Some(v-1);
            if finish.unwrap() == 0 {break}
        }

        (j, max_y) = add_shape(i, &input, &mut map, j, max_y);
        let state = (i % SHAPES.len(), j, signature(&map, max_y));

        if let Some(prev) = positions.get(&state) {
            from = Some(*prev);
            to = Some((i, max_y));
            finish = Some((1000000000000 - prev.0) % (i - prev.0));
        } else {
            positions.insert(state, (i, max_y));
        }
    }

    let (from_i, from_y) = from.unwrap();
    let (to_i, to_y) = to.unwrap();
    let period = to_i - from_i;
    let growth_in_period = to_y - from_y;
    let distance = 1000000000000 - from_i;
    from_y + (growth_in_period * (distance / period)) + (max_y - to_y)
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
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
