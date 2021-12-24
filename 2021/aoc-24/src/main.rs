use std::str::FromStr;
use std::time::Instant;

use aoc_1::{duration, read_stdin};

static A: [isize; 14] = [10, 12, 13, 13, 14, -2, 11, -15, -10, 10, -10, -4, -1, -1];
static C: [isize; 14] = [1, 1, 1, 1, 1, 26, 1, 26, 26, 1, 26, 26, 26, 26];

#[derive(Clone, Debug)]
enum Register {W, X, Y, Z, Number(isize)}

impl FromStr for Register {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            v => Self::Number(v.parse().unwrap()),
        })
    }
}

impl Register {
    fn get(&self, state: &State) -> isize {
        match self {
            Self::W => state.w,
            Self::X => state.x,
            Self::Y => state.y,
            Self::Z => state.z,
            Self::Number(v) => *v,
        }
    }

    fn set(&self, state: &mut State, v: isize) {
        match self {
            Self::W => state.w = v,
            Self::X => state.x = v,
            Self::Y => state.y = v,
            Self::Z => state.z = v,
            Self::Number(_) => panic!("Can't set number"),
        };
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Inp (Register),
    Add (Register, Register),
    Mul (Register, Register),
    Div (Register, Register),
    Mod (Register, Register),
    Eql (Register, Register),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (command, args) = input.split_once(' ').unwrap();

        Ok(match command {
            "inp" => Self::Inp(args.parse()?),
            "add" => {
                let (x, y) = args.split_once(' ').unwrap();
                Self::Add(x.parse()?, y.parse()?)
            },
            "mul" => {
                let (x, y) = args.split_once(' ').unwrap();
                Self::Mul(x.parse()?, y.parse()?)
            },
            "div" => {
                let (x, y) = args.split_once(' ').unwrap();
                Self::Div(x.parse()?, y.parse()?)
            },
            "mod" => {
                let (x, y) = args.split_once(' ').unwrap();
                Self::Mod(x.parse()?, y.parse()?)
            },
            "eql" => {
                let (x, y) = args.split_once(' ').unwrap();
                Self::Eql(x.parse()?, y.parse()?)
            },
            _ => panic!("Unknown instruction '{}' with args '{}'", command, args)
        })
    }
}

#[derive(Clone, Debug)]
struct State<'a> {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
    inputs: Vec<isize>,
    i: usize,
    instructions: &'a Vec<Instruction>,
}

impl <'a> State<'a> {
    fn new(instructions: &'a Vec<Instruction>) -> Self {
        let inputs: Vec<isize> = Vec::new();
        Self {w: 0, x: 0, y: 0, z: 0, i:0, inputs, instructions}
    }

    fn solve(mut self, smallest: bool) -> Option<Self> {
        for instruction in &self.instructions[self.i..] {
            match instruction {
                Instruction::Inp(_) => {
                    if C[self.i / 18] == 1 {
                        let mut range: Vec<isize> = (1..10).collect();
                        if !smallest {range.reverse()}

                        for i in range {
                            let mut s = Self {
                                w: i,
                                x: self.x,
                                y: self.y,
                                z: self.z,
                                inputs: self.inputs.clone(),
                                i: self.i + 1,
                                instructions: self.instructions,

                            };
                            s.inputs.push(i);
                            let x = s.solve(smallest);
                            if x.is_some() {
                                return x
                            }
                        }
                        return None
                    } else {
                        let i = (self.z % 26) + A[self.i / 18];
                        if i < 1 || i > 9 {
                            return None
                        }
                        self.w = i;
                        self.inputs.push(i);
                    }
                },
                Instruction::Add(x, y) => {
                    let v = x.get(&self) + y.get(&self);
                    x.set(&mut self, v);
                },
                Instruction::Mul(x, y) => {
                    let v = x.get(&self) * y.get(&self);
                    x.set(&mut self, v);
                },
                Instruction::Div(x, y) => {
                    let v = x.get(&self) / y.get(&self);
                    x.set(&mut self, v);
                },
                Instruction::Mod(x, y) => {
                    let v = x.get(&self) % y.get(&self);
                    x.set(&mut self, v);
                },
                Instruction::Eql(x, y) => {
                    let v = if x.get(&self) == y.get(&self) {1} else {0};
                    x.set(&mut self, v);
                },
            }

            self.i += 1
        }

        if self.z == 0 {return Some(self)}
        None
    }
}

#[derive(Clone, Debug)]
struct Input {
    instructions: Vec<Instruction>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: input.lines().map(|l| l.parse().unwrap()).collect()
        })
    }
}

fn part_1(input: Input) -> Option<usize> {
    let instructions = input.instructions;
    let state = State::new(&instructions);

    let x = state.solve(false)?;

    let output: String = x.inputs.into_iter()
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect();

    Some(output.parse().unwrap())
}

fn part_2(input: Input) -> Option<usize> {
    let instructions = input.instructions;
    let state = State::new(&instructions);

    let x = state.solve(true)?;

    let output: String = x.inputs.into_iter()
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect();

    Some(output.parse().unwrap())
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
