use aoc_19::read_stdin;

use std::{time::Instant, collections::LinkedList};

type Input = LinkedList<isize>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part_1(mut input: Input) -> usize {
    let j = 0;
    for i in 0..input.len() {
        input.remove(j);
        j += 1;
    }
    1
}

fn part_2(input: Input) -> usize {
    1
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let mut lines = parse(input);
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
