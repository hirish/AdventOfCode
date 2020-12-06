use aoc::read_stdin;
use std::collections::{HashSet, HashMap};

fn part1(lines: &str) -> usize {
    let mut answers: HashSet<char> = HashSet::new();
    for line in lines.lines() {
        for c in line.chars() {
            answers.insert(c);
        }
    }
    answers.len()
}

fn part2(lines: &str) -> usize {
    let mut answers: HashMap<char, usize> = HashMap::new();
    for line in lines.lines() {
        for c in line.chars() {
            let count: usize = *answers.get(&c).unwrap_or(&0);
            answers.insert(c, count+1);
        }
    }
    let group_size = lines.lines().count();
    answers
        .values()
        .filter(|count| **count == group_size)
        .count()
}

fn part_1(input: String) -> usize {
    input
        .split("\n\n")
        .map(part1)
        .sum()
}

fn part_2(input: String) -> usize {
    input
        .split("\n\n")
        .map(part2)
        .sum()
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
