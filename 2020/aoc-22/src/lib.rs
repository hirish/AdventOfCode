use std::io::{self, Read};
use std::str::FromStr;

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn read_stdin_numbers() -> Vec<u32> {
    read_stdin()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

pub struct Combat {
    pub player_1: Vec<usize>,
    pub player_2: Vec<usize>
}

impl FromStr for Combat {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut s = input.split("\n\n");

        let player_1: Vec<usize> = s
            .next().unwrap()
            .lines()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect();

        let player_2: Vec<usize> = s
            .next().unwrap()
            .lines()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Self { player_1, player_2 })
    }
}

impl Combat {

    fn complete(&self) -> bool {
        (self.player_1.len() == 0) || (self.player_2.len() == 0)
    }

    fn step(&mut self) {
        let p1 = self.player_1.remove(0);
        let p2 = self.player_2.remove(0);

        if p1 > p2 {
            self.player_1.append(&mut vec![p1, p2])
        } else {
            self.player_2.append(&mut vec![p2, p1])
        }
    }

    pub fn play(&mut self) {
        while !self.complete() {
            self.step();
        }
    }

    pub fn score(&self) -> Result<usize, ()> {
        if !self.complete() {
            Err(())
        } else {
            let cards = if self.player_1.len() > 0 {&self.player_1} else {&self.player_2};

            Ok(
                cards
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, v)| (i+1) * v)
                    .sum()
            )
        }
    }

}
