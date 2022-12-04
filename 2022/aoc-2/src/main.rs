use std::fmt::Debug;
use std::str::FromStr;
use strum_macros::EnumString;

use aoc_2::read_stdin;

#[derive(EnumString, Clone)]
enum Opposition {
    A,
    B,
    C,
}

#[derive(EnumString)]
enum Player {
    X,
    Y,
    Z,
}

impl Player {
    fn score(&self, opp: &Opposition) -> usize {
        match self {
            Self::X => match opp {
                Opposition::A => 1 + 3,
                Opposition::B => 1 + 0,
                Opposition::C => 1 + 6,
            },
            Self::Y => match opp {
                Opposition::A => 2 + 6,
                Opposition::B => 2 + 3,
                Opposition::C => 2 + 0,
            },
            Self::Z => match opp {
                Opposition::A => 3 + 0,
                Opposition::B => 3 + 6,
                Opposition::C => 3 + 3,
            },
        }
    }
}

#[derive(EnumString)]
enum Outcome {
    X,
    Y,
    Z,
}

impl Outcome {
    fn to_player(&self, opp: &Opposition) -> Player {
        match self {
            Self::X => match opp {
                Opposition::A => Player::Z,
                Opposition::B => Player::X,
                Opposition::C => Player::Y,
            },
            Self::Y => match opp {
                Opposition::A => Player::X,
                Opposition::B => Player::Y,
                Opposition::C => Player::Z,
            },
            Self::Z => match opp {
                Opposition::A => Player::Y,
                Opposition::B => Player::Z,
                Opposition::C => Player::X,
            },
        }
    }
}

fn parse<T: FromStr>(line: &str) -> (Opposition, T)
where
    <T as FromStr>::Err: Debug,
{
    let (a, b) = line.split_once(" ").unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn part_1(input: String) -> usize {
    input
        .lines()
        .map(|l| parse::<Player>(l))
        .map(|v| v.1.score(&v.0))
        .sum()
}

fn part_2(input: String) -> usize {
    input
        .lines()
        .map(|l| parse::<Outcome>(l))
        .map(|v| (v.0.clone(), v.1.to_player(&v.0)))
        .map(|v| v.1.score(&v.0))
        .sum()
}

fn main() {
    let input = read_stdin();

    println!("Answer 1: {}", part_1(input.clone()));
    println!("Answer 2: {}", part_2(input));
}
