use aoc::read_stdin;
use std::collections::HashMap;

fn solve(numbers: Vec<usize>, n:usize) -> usize {
    let mut prev: usize = *numbers.last().unwrap();
    let mut last_seen: HashMap<usize, usize> = HashMap::new();

    for (i, x) in numbers.iter().enumerate() {
        last_seen.insert(*x, i);
    }

    for i in numbers.len()..n {
        let new = match last_seen.get(&prev) {
            Some(x) => i - x - 1,
            None => 0
        };
        last_seen.insert(prev, i-1);
        prev = new
    }

    prev
}

fn part_1(numbers: Vec<usize>) -> usize {
    solve(numbers, 2020)
}

fn part_2(numbers: Vec<usize>) -> usize {
    solve(numbers, 30000000)
}

fn main() {
    let lines: Vec<usize> = read_stdin()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
