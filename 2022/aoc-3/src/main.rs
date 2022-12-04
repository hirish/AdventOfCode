use std::collections::HashSet;

use aoc_3::read_stdin;

type Input = Vec<(HashSet<char>, HashSet<char>)>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| {
            let mut l1: Vec<char> = l.chars().collect();
            let l2 = l1.split_off(l.len() / 2);
            (l1.into_iter().collect(), l2.into_iter().collect())
        })
        .collect()
}

fn score(v: char) -> usize {
    if (v as usize) < 96 {
        // Capitals
        (v as usize) - 38
    } else {
        (v as usize) - 96
    }
}

fn part_1(input: String) -> usize {
    let input = parse(input);
    input
        .into_iter()
        .map(|(a, b)| score(*a.intersection(&b).next().unwrap()))
        .sum()
}

fn part_2(input: String) -> usize {
    let input: Vec<HashSet<char>> = input.lines().map(|l| l.chars().collect()).collect();

    input
        .chunks(3)
        .map(|vs| {
            let tmp = &vs[0].intersection(&vs[1]).map(|c| *c).collect();
            let val = vs[2].intersection(tmp).next().unwrap();
            score(*val)
        })
        .sum()
}

fn main() {
    let input = read_stdin();

    println!("Answer 1: {}", part_1(input.clone()));
    println!("Answer 2: {}", part_2(input));
}
