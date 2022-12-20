use aoc_17::read_stdin;

use std::collections::HashSet;
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

fn add_shape(i: usize, input: &Input, map: &mut Vec<usize>, mut j: usize) -> usize {
    let shape = &SHAPES[i % SHAPES.len()];

    let mut gas = true;

    let max_y = *map.iter().max().unwrap_or(&0);
    let mut pos: Coord = (2, max_y + 2 + shape.len());
    // println!("{:?}", shape);

    loop {
        // println!("{}: {:?}", gas, pos);
        if gas {
            let x = match input[j % input.len()] {
                Direction::R => 1,
                Direction::L => -1,
            } + pos.0 as isize;

            // println!("{:?}", input[j % input.len()]);

            pos = if shape.iter().enumerate().all(|(dy, row)| {
                row.chars().enumerate().all(|(dx, c)| {
                    c == '.'
                        || ((x >= 0)
                            && ((x as usize) + dx <= 6)
                            && map[(x as usize) + dx] <= pos.1 - dy)
                })
            }) {
                (x as usize, pos.1)
            } else {
                pos
            };

            j += 1;
        } else {
            if shape.iter().enumerate().any(|(dy, row)| {
                row.chars()
                    .enumerate()
                    .filter(|v| v.1 == '#')
                    .any(|(dx, c)| {
                        c == '#' && ((pos.1 - dy == 0) || map[pos.0 + dx] > pos.1 - dy - 1)
                    })
            }) {
                break;
            };

            pos = (pos.0, pos.1 - 1)
        }

        gas = !gas;
    }

    shape.iter().enumerate().rev().for_each(|(dy, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(dx, _)| {
                map[pos.0 + dx] = pos.1 - dy + 1;
            })
    });

    j
}

fn part_1(input: Input) -> usize {
    let mut map: Vec<usize> = vec![0,0,0,0,0,0,0];
    let mut j = 0;
    for i in 0..2022 {
        j = add_shape(i, &input, &mut map, j);
    }
    map.into_iter().max().unwrap()
}

fn part_2(input: Input) -> usize {
    let mut map: Vec<usize> = vec![0,0,0,0,0,0,0];
    let mut j = 0;
    // for i in 0..(5*10091) {
    for i in 0..3 {
        j = add_shape(i, &input, &mut map, j);
        println!("{:?}", map);
    }
    map.into_iter().max().unwrap()
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
