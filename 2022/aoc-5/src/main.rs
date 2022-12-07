use aoc_5::read_stdin;

use std::str::FromStr;

#[derive(Clone)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, rest) = s[5..].split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();

        Ok(Instruction {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
            count: count.parse().unwrap(),
        })
    }
}

type Input = (Vec<Vec<char>>, Vec<Instruction>);

fn parse(input: String) -> Input {
    let (unparsed_stack, unparsed_instructions) = input.split_once("\n\n").unwrap();
    let no_buckets = (input.lines().next().unwrap().len() + 1) / 4;

    let mut stack: Vec<Vec<char>> = (1..=no_buckets).map(|_| Vec::new()).collect();
    unparsed_stack.lines().rev().skip(1).for_each(|l| {
        l.match_indices("[")
            .for_each(|(i, _)| stack[i / 4].push(l.chars().nth(i + 1).unwrap()))
    });

    let instructions = unparsed_instructions
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    (stack, instructions)
}

fn part_1((mut stack, instructions): Input) -> String {
    for Instruction { from, to, count } in instructions {
        for _ in 0..count {
            let x = stack[from - 1].pop().unwrap();
            stack[to - 1].push(x);
        }
    }

    stack.iter().map(|s| s.last().unwrap()).collect()
}

fn part_2((mut stack, instructions): Input) -> String {
    for Instruction { from, to, count } in instructions {
        let mut x: Vec<char> = (0..count).map(|_| stack[from - 1].pop().unwrap()).collect();
        x.reverse();
        stack[to - 1].append(&mut x)
    }

    stack.iter().map(|s| s.last().unwrap()).collect()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
