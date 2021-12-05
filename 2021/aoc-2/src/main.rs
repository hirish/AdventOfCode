use std::str::FromStr;

use aoc_1::read_stdin;

#[derive(Clone, Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => {
                println!("Couldn't parse {}", input);
                Err(())
            },
        }
    }
}



#[derive(Clone, Debug)]
struct Instruction {
    direction: Direction,
    v: u32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let l = input.split(' ').collect::<Vec<&str>>();
        let direction = l[0].parse().unwrap();
        let v = l[1].parse().unwrap();
        Ok(Self{direction, v})
    }
}

fn parse(input: String) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn part_1(instructions: Vec<Instruction>) -> Option<u32> {
    let mut h = 0;
    let mut d = 0;

    for ins in instructions {
        match ins.direction {
            Direction::Forward => h += ins.v,
            Direction::Up => d -= ins.v,
            Direction::Down => d += ins.v,
        }
    }

    Some(h * d)
}

fn part_2(instructions: Vec<Instruction>) -> Option<u32> {
    let mut a = 0;
    let mut h = 0;
    let mut d = 0;

    for ins in instructions {
        match ins.direction {
            Direction::Forward => {
                h += ins.v;
                d += a * ins.v
            },
            Direction::Up => a -= ins.v,
            Direction::Down => a += ins.v,
        }
    }

    Some(h * d)
}

fn main() {
    let input = read_stdin();
    let instructions = parse(input);
    println!("Answer 1: {}", part_1(instructions.clone()).expect("No answer found"));
    println!("Answer 2: {}", part_2(instructions).expect("No answer found"));
}
