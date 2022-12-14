use aoc_13::read_stdin;

use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum V {
    Number(usize),
    Array(Vec<V>),
}

impl std::fmt::Debug for V {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            V::Number(v) => write!(fmt, "{:?}", v),
            V::Array(v) => write!(fmt, "{:?}", v),
        }
    }
}

type Input = Vec<V>;

fn parse(input: String) -> Input {
    input
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

impl PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for V {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (V::Number(a), V::Number(b)) => a.cmp(b),
            (V::Number(a), _) => V::Array(vec![V::Number(*a)]).cmp(other),
            (_, V::Number(b)) => self.cmp(&V::Array(vec![V::Number(*b)])),
            (V::Array(a), V::Array(b)) => a.cmp(b),
        }
    }
}

fn part_1(input: Input) -> usize {
    input
        .chunks(2)
        .enumerate()
        .filter_map(|(i, j)| if j[0] < j[1] { Some(i + 1) } else { None })
        .sum()
}

fn part_2(mut input: Input) -> usize {
    let a: V = V::Array(vec![V::Array(vec![V::Number(2)])]);
    let b: V = V::Array(vec![V::Array(vec![V::Number(6)])]);
    input.push(a.clone());
    input.push(b.clone());
    input.sort();
    let x = input.iter().position(|x| x == &a).unwrap() + 1;
    let y = input.iter().position(|x| x == &b).unwrap() + 1;
    x * y
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
