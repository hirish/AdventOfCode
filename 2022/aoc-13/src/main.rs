use aoc_13::read_stdin;

use serde::Deserialize;
use std::cmp::Ordering::{self, Equal, Greater, Less};

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
        }?;
        Ok(())
    }
}

type Input = Vec<V>;

fn parse(input: String) -> Input {
    input
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

fn compare(a: &V, b: &V) -> Ordering {
    match (a, b) {
        (V::Number(a), V::Number(b)) => b.cmp(a),
        (V::Number(_), _) => compare(&V::Array(vec![a.clone()]), b),
        (_, V::Number(_)) => compare(a, &V::Array(vec![b.clone()])),
        (V::Array(a), V::Array(b)) => match (&a[..], &b[..]) {
            ([], []) => Equal,
            (_, []) => Less,
            ([], _) => Greater,
            ([a, c @ ..], [b, d @ ..]) => {
                let c = V::Array(c.to_vec());
                let d = V::Array(d.to_vec());
                compare(a, b).then_with(|| compare(&c, &d))
            }
        },
    }
}

fn part_1(input: Input) -> usize {
    input
        .chunks(2)
        .enumerate()
        .filter_map(|(i, j)| {
            if compare(&j[0], &j[1]) == Greater {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(mut input: Input) -> usize {
    let a = V::Array(vec![V::Array(vec![V::Number(2)])]);
    let b = V::Array(vec![V::Array(vec![V::Number(6)])]);
    input.push(a.clone());
    input.push(b.clone());
    input.sort_by(compare);
    input.reverse();
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
