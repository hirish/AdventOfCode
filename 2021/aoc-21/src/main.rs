use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

use aoc_1::{duration, read_stdin};

#[derive(Clone, Debug)]
struct Input {
    p1: usize,
    p2: usize,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (a, b) = input.split_once("\n").unwrap();

        let p1 = a.chars().last().unwrap().to_digit(10).unwrap() as usize - 1;
        let p2 = b.chars().last().unwrap().to_digit(10).unwrap() as usize - 1;

        Ok(Self { p1, p2 })
    }
}

fn part_1(input: Input) -> Option<usize> {
    let Input { mut p1, mut p2 } = input;
    let mut dice = 1;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut p1_turn = true;

    while p1_score < 1000 && p2_score < 1000 {
        if p1_turn {
            p1 = (p1 + dice + dice + 1 + dice + 2) % 10;
            p1_score += p1 + 1;
        } else {
            p2 = (p2 + dice + dice + 1 + dice + 2) % 10;
            p2_score += p2 + 1;
        }

        dice += 3;
        p1_turn = !p1_turn;
    }

    if p1_turn {
        Some(p1_score * (dice - 1))
    } else {
        Some(p2_score * (dice - 1))
    }
}

#[derive(Copy, Debug, Clone, Hash, Eq)]
struct State {
    p1_pos: usize,
    p1_score: usize,
    p2_pos: usize,
    p2_score: usize,
    p1_turn: bool,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.p1_pos == other.p1_pos
            && self.p1_score == other.p1_score
            && self.p2_pos == other.p2_pos
            && self.p2_score == other.p2_score
            && self.p1_turn == other.p1_turn
    }
}

fn count_wins(state: State, cache: &mut HashMap<State, (usize, usize)>) -> (usize, usize) {
    let State {
        p1_pos,
        p1_score,
        p2_pos,
        p2_score,
        p1_turn,
    } = state;

    if p1_score >= 21 {
        return (1, 0);
    }

    if p2_score >= 21 {
        return (0, 1);
    }

    if cache.contains_key(&state) {
        return cache[&state];
    }

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let counts = vec![0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

    if p1_turn {
        for dice in 3..10 {
            let new_pos = (p1_pos + dice) % 10;
            let (p1, p2) = count_wins(
                State {
                    p1_pos: new_pos,
                    p1_score: p1_score + new_pos + 1,
                    p2_pos,
                    p2_score,
                    p1_turn: false,
                },
                cache,
            );
            p1_wins += counts[dice] * p1;
            p2_wins += counts[dice] * p2;
        }
    } else {
        for dice in 3..10 {
            let new_pos = (p2_pos + dice) % 10;
            let (p1, p2) = count_wins(
                State {
                    p1_pos,
                    p1_score,
                    p2_pos: new_pos,
                    p2_score: p2_score + new_pos + 1,
                    p1_turn: true,
                },
                cache,
            );
            p1_wins += counts[dice] * p1;
            p2_wins += counts[dice] * p2;
        }
    };

    cache.insert(state, (p1_wins, p2_wins));
    (p1_wins, p2_wins)
}

fn part_2(input: Input) -> Option<usize> {
    let Input { p1, p2 } = input;
    let mut cache: HashMap<State, (usize, usize)> = HashMap::new();

    let (p1_wins, p2_wins) = count_wins(
        State {
            p1_pos: p1,
            p1_score: 0,
            p2_pos: p2,
            p2_score: 0,
            p1_turn: true,
        },
        &mut cache,
    );

    Some(if p1_wins > p2_wins { p1_wins } else { p2_wins })
}

fn main() -> Result<(), ()> {
    let now = Instant::now();
    let input: Input = read_stdin().parse()?;
    println!("Running parsing took {}.", duration(now));

    let now = Instant::now();
    println!("Answer 1: {}", part_1(input.clone()).ok_or(())?);
    println!("Running part_1 took {}.", duration(now));

    let now = Instant::now();
    println!("Answer 2: {}", part_2(input).ok_or(())?);
    println!("Running part_2 took {}.", duration(now));

    Ok(())
}
