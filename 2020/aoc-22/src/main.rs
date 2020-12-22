use aoc::{read_stdin, Combat};
use std::str::FromStr;
use std::collections::HashSet;

pub struct RecursiveCombat {
    pub player_1: Vec<usize>,
    pub player_2: Vec<usize>,
    pub completed: bool,
    pub cache: HashSet<String>,
}

impl FromStr for RecursiveCombat {
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

        Ok(Self::new(player_1, player_2))
    }
}

impl RecursiveCombat {

    fn new(player_1: Vec<usize>, player_2: Vec<usize>) -> Self {
        let cache = HashSet::new();
        Self { player_1, player_2, cache, completed: false}
    }

    fn complete(&self) -> bool {
        self.completed || (self.player_1.len() == 0) || (self.player_2.len() == 0)
    }

    fn step(&mut self) {
        let curr_state = format!("{:?}-{:?}", self.player_1, self.player_2);

        if self.cache.contains(&curr_state) {
            self.completed = true;
            return
        }

        let p1 = self.player_1.remove(0);
        let p2 = self.player_2.remove(0);

        let p1_winner = if (self.player_1.len() >= p1) && (self.player_2.len() >= p2) {
            let player_1 = self.player_1[0..p1].iter().cloned().collect();
            let player_2 = self.player_2[0..p2].iter().cloned().collect();
            let mut sub_game = RecursiveCombat::new(player_1, player_2);
            sub_game.play()
        } else {
            p1 > p2
        };

        self.cache.insert(curr_state);

        if p1_winner {
            self.player_1.append(&mut vec![p1, p2])
        } else {
            self.player_2.append(&mut vec![p2, p1])
        }
    }

    pub fn play(&mut self) -> bool {
        while !self.complete() {
            self.step();
        }
    
        self.player_1.len() > 0
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


fn main() {
    let input = read_stdin();
    let mut game: Combat = input.parse().unwrap();
    game.play();

    println!("Answer 1: {}", game.score().unwrap());

    let mut game: RecursiveCombat = input.parse().unwrap();
    game.play();

    println!("Answer 2: {}", game.score().unwrap());
}
