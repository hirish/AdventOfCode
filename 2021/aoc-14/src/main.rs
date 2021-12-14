use std::collections::HashMap;
use std::str::FromStr;

use aoc_1::read_stdin;

#[derive(Clone, Debug)]
struct Input {
    input: HashMap<(char, char), usize>,
    transformations: HashMap<(char, char), char>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (a, b) = input.split_once("\n\n").unwrap();

        let mut a: Vec<char> = a.chars().collect();
        a.push(' ');
        let mut input = HashMap::new();
        for i in 0..a.len() - 1 {
            *input.entry((a[i], a[i + 1])).or_insert(0) += 1
        }

        let mut transformations = HashMap::new();
        b.lines().for_each(|l| {
            let (x, y) = l.split_once(" -> ").unwrap();
            let mut x = x.chars();
            let mut y = y.chars();
            transformations.insert((x.next().unwrap(), x.next().unwrap()), y.next().unwrap());
        });

        Ok(Input {
            input,
            transformations,
        })
    }
}

fn compute(input: Input, rounds: usize) -> Option<usize> {
    let mut current = input.input;
    let transformations = input.transformations;

    for _ in 0..rounds {
        let mut new_input = HashMap::new();

        for (v, c) in current {
            if transformations.contains_key(&v) {
                let x = *transformations.get(&v)?;
                *new_input.entry((v.0, x)).or_insert(0) += c;
                *new_input.entry((x, v.1)).or_insert(0) += c;
            } else {
                *new_input.entry(v).or_insert(0) += c;
            }
        }

        current = new_input;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for (k, v) in current {
        *counts.entry(k.0).or_insert(0) += v;
    }

    Some(counts.values().max()? - counts.values().min()?)
}

fn part_1(input: Input) -> Option<usize> {
    compute(input, 10)
}

fn part_2(input: Input) -> Option<usize> {
    compute(input, 40)
}

fn main() -> Result<(), ()> {
    let input: Input = read_stdin().parse()?;

    println!("Answer 1: {}", part_1(input.clone()).ok_or(())?);
    println!("Answer 2: {}", part_2(input).ok_or(())?);

    Ok(())
}
