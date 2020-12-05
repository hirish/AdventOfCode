use aoc::read_stdin;
use std::collections::HashSet;

fn parser(sn: &str) -> usize {
    let mut val = 0;
    for (i, c) in sn.chars().rev().enumerate() {
        if c == 'R' || c == 'B' {
            val += 2u32.pow(i as u32)
        }
    }
    val as usize
}

fn part_1(input: String) -> usize {
    input
        .lines()
        .map(parser)
        .max()
        .unwrap()
}

fn part_2(input: String) -> usize {
    let seats: HashSet<usize> = input.lines().map(parser).collect();
    let min = *seats.iter().min().unwrap();
    let max = *seats.iter().max().unwrap();
    for i in min..max {
        if !seats.contains(&i) {
            return i
        }
    }
    panic!("Couldn't find seat")
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
