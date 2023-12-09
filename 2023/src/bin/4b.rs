use std::collections::{HashMap, HashSet};

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

    let mut counts: HashMap<usize, usize> = (0..lines.len()).into_iter().map(|i| (i, 1)).collect();

    lines
        .into_iter()
        .map(|(winning, input)| input.into_iter().filter(|n| winning.contains(n)).count())
        .enumerate()
        .for_each(|(i, n)| {
            let v = *counts.get(&i).unwrap();
            for j in i + 1..=i + n {
                counts.entry(j).and_modify(|w| *w += v);
            }
        });

    let n: usize = counts.values().sum();

    println!("{}", n);
}
