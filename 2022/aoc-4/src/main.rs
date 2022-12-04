use std::str::FromStr;

use aoc_1::read_stdin;

type Input = Vec<(Range, Range)>;

#[derive(Clone)]
struct Range {
    from: usize,
    to: usize,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").unwrap();
        Ok(Self {
            from: a.parse().unwrap(),
            to: b.parse().unwrap(),
        })
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.from <= other.to && self.to >= other.from
    }
}

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn part_1(input: Input) -> usize {
    input
        .iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count()
}

fn part_2(input: Input) -> usize {
    input.iter().filter(|(a, b)| a.overlaps(b)).count()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
