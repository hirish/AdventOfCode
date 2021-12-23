use std::collections::{HashMap, BinaryHeap};
use std::str::FromStr;
use std::time::Instant;
use std::fmt;

use aoc_1::{duration, read_stdin, min, max, diff};

#[derive(Clone, Debug, Copy, Hash, Eq, Ord)]
struct Crab {
    t: char,
    x: usize,
    y: usize,
    moves: usize,
}

impl PartialEq for Crab {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y &&
        self.t == other.t && self.moves == other.moves
    }
}

impl PartialOrd for Crab {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Crab {
    fn new(x: usize, y: usize, t: char) -> Self {
        Self {x, y, moves: 0, t}
    }

    fn dest(&self) -> usize {
        match self.t {
            'A' => 3,
            'B' => 5,
            'C' => 7,
            'D' => 9,
            t => panic!("Unknown type {}", t)
        }
    }

    fn move_cost(&self) -> usize {
        match self.t {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            t => panic!("Unknown type {}", t)
        }
    }

    fn score(&self) -> usize {
        1000 * self.moves +
        100 * self.x +
        10 * self.y +
        self.dest()
    }
}

#[derive(Clone, Hash, Eq)]
struct State {
    crabs: Vec<Crab>,
    depth: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.crabs == other.crabs && self.depth == other.depth
    }
}

impl Ord for State {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = vec![
            "#############".to_string(),
            "#           #".to_string(),
            "### # # # ###".to_string(),
        ];

        for _ in 0..self.depth-1 {
            output.push("  # # # # #".to_string())
        }

        output.push("  #########".to_string());

        for crab in self.crabs.iter() {
            output[crab.y].replace_range(crab.x..crab.x+1, &crab.t.to_string());
        }

        for line in output {
            writeln!(f, "{}", line.replace("#", "█"))?;
        }

        Ok(())
    }
}

impl State {
    fn crab_at(&self, x: usize, y: usize) -> Option<&Crab> {
        self.crabs.iter().find(|&c| c.x == x && c.y == y)
    }

    fn has_crab_at(&self, x: usize, y: usize) -> bool {
        self.crab_at(x, y).is_some()
    }

    fn can_reach(&self, from: (usize, usize), to: (usize, usize), t: char) -> bool {
        if self.has_crab_at(to.0, to.1) {
            return false
        }

        if from.1 >= 3 { // If we're leaving the bottom row then make sure there's a path out
            for y in 2..from.1 {
                if self.has_crab_at(from.0, y) {
                    return false
                }
            }
        }

        if to.1 > 1 { // If we're entering a room
            for y in 2..to.1 { // check there's a path in to it
                if self.has_crab_at(to.0, y) {
                    return false
                }
            }

            for y in to.1+1..self.depth+2 { // check everything below is full
                let c = self.crab_at(to.0, y);
                if c.is_none() || c.unwrap().t != t { // If there's no crab there, or a crab of a different type
                    return false
                }
            }
        }

        for x in min(from.0, to.0+1)..max(from.0, to.0+1) { // Check the corridor
            if from.0 == x && from.1 == 1 { // Ignore the start square!
                continue
            }

            if self.has_crab_at(x, 1) {
                return false
            }
        }

        true
    }
    
    fn new_positions(&self, crab: &Crab) -> Vec<(Crab, usize)> {
        let &Crab{x, y, t, moves} = crab;
        if moves >= 2 {return vec![]}

        let possibles: Vec<(usize, usize)> = if moves == 0 {
            vec![1, 2, 4, 6, 8, 10, 11].into_iter().map(|x| (x, 1)).collect()
        } else { // If it's already moved it can only move into a room
            let dest = crab.dest();
            (2..self.depth+2).map(|y| (dest, y)).collect()
        };

        possibles
            .into_iter()
            .filter(|p| self.can_reach((x, y), *p, t))
            .map(|(px, py)| (
                Crab {x: px, y: py, t, moves: moves + 1},
                (diff(x, px) + y + py - 2) * crab.move_cost()
            ))
            .collect()
    }

    fn next_states(&self) -> Vec<(State, usize)> {
        let mut next_states: Vec<(State, usize)> = vec![];

        for crab in self.crabs.iter() {
            for (new_crab, cost) in self.new_positions(&crab) {
                let mut crabs: Vec<Crab> = self.crabs.clone().into_iter().filter(|c| c != crab).collect();
                crabs.push(new_crab);
                crabs.sort_by(|a, b| a.score().cmp(&b.score()));
                next_states.push((Self{crabs, depth: self.depth}, cost))
            }
        }

        next_states
    }

    fn is_finished(&self) -> bool {
        if self.crabs.iter().any(|c| c.y == 1) {
            return false
        }

        self.crabs.iter().all(|c| c.x == c.dest())
    }

    fn h(&self) -> usize {
        self.crabs
            .iter()
            .map(|c| c.move_cost() * diff(c.x, c.dest()))
            .sum()
    }

    fn hash(&self) -> String {
        self.crabs.iter().map(|c| format!("{}", c.score())).collect()
    }
}

impl FromStr for State {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut crabs = vec![];

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'A' || c == 'B' || c == 'C' || c == 'D' {
                    crabs.push(Crab::new(x, y, c))
                }
            }
        }

        crabs.sort_by(|a, b| a.score().cmp(&b.score()));

        Ok(Self {
            crabs,
            depth: input.lines().count() - 3,
        })
    }
}

#[derive(Ord, Debug, Eq)]
struct ScoredState {
    state: State,
    score: usize,
}

impl PartialEq for ScoredState {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.score == other.score
    }
}

impl PartialOrd for ScoredState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

fn a_star(start: State) -> Option<usize> {
    let mut open: BinaryHeap<ScoredState> = BinaryHeap::new();
    let mut g: HashMap<String, usize> = HashMap::new();
    let mut f: HashMap<String, usize> = HashMap::new();
    
    g.insert(start.hash(), 0);
    f.insert(start.hash(), start.h());
    open.push(ScoredState{state: start, score: 0});

    while !open.is_empty(){
        let ScoredState{state, score: _} = open.pop().unwrap();

        if state.is_finished() {
            return Some(g[&state.hash()])
        }

        for (next_state, cost) in state.next_states() {
            let new_g = g[&state.hash()] + cost;
            if !g.contains_key(&next_state.hash()) || new_g < g[&next_state.hash()] {
                let h = next_state.h();
                g.insert(next_state.hash(), new_g);
                f.insert(next_state.hash(), new_g + h);
                let scored_state = ScoredState{state: next_state, score: new_g + h};
                open.push(scored_state);
            }

        }
    }

    None
}

#[derive(Clone, Debug)]
struct Input {
    input: String,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {input: input.to_string()})
    }
}

fn part_1(input: Input) -> Option<usize> {
    let state = input.input.parse().unwrap();
    a_star(state)
}

fn unfold(input: String) -> String {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.insert(3, "  #D#B#A#C#");
    lines.insert(3, "  #D#C#B#A#");
    lines.join("\n")
}

fn part_2(input: Input) -> Option<usize> {
    let input = unfold(input.input);
    let state: State = input.parse().unwrap();
    a_star(state)
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
