use aoc_6::read_stdin;
use itertools::Itertools;

type Input = String;

fn solve(n: usize, input: Input) -> Option<usize> {
    for i in n..=(input.len()) {
        if input[i-n..i].chars().unique().count() == n {
            return Some(i)
        }
    }
    None
}

fn part_1(input: Input) -> usize {
    solve(4, input).unwrap()
}

fn part_2(input: Input) -> usize {
    solve(14, input).unwrap()
}

fn main() {
    let input = read_stdin();

    println!("Answer 1: {}", part_1(input.clone()));
    println!("Answer 2: {}", part_2(input));
}
