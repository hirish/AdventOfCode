use std::collections::HashSet;

use aoc2024::read_stdin;

type Input = Vec<(HashSet<usize>, Vec<usize>)>;

fn parse_line(input: &str) -> (HashSet<usize>, Vec<usize>) {
    let (_, input) = input.split_once(": ").unwrap();
    let (winning, input) = input.split_once(" | ").unwrap();
    let winning = winning
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let input = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    (winning, input)
}

fn parse(input: String) -> Input {
    input.lines().map(parse_line).collect()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);
    let n: usize = lines
        .into_iter()
        .map(|(winning, input)| {
            input
                .clone()
                .into_iter()
                .filter(|n| winning.contains(n))
                .count()
        })
        .filter(|n| *n > 0)
        .map(|i| (2 as usize).pow((i - 1) as u32))
        .sum();

    println!("{}", n);
}
