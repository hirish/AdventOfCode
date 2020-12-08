use aoc::read_stdin;
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Opcode {
    Nop,
    Jmp,
    Acc,
}

impl Opcode {
    fn new(opcode: &str) -> Self {
        match opcode {
            "nop" => Self::Nop,
            "jmp" => Self::Jmp,
            "acc" => Self::Acc,
            _ => panic!("Unknown opcode {}", opcode),
        }
    }
}

#[derive(Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    n: i32,
}

#[derive(Copy, Clone)]
struct Patch {
    line: usize,
    instruction: Instruction,
}

impl Patch {
    fn new(line: usize, opcode: &str, n: i32) -> Self {
        let instruction = Instruction {
            opcode: Opcode::new(opcode),
            n,
        };
        Patch { line, instruction }
    }
}

fn parse_line(line: &str) -> Instruction {
    let opcode: Opcode = Opcode::new(&line[0..3]);
    let sign: char = line.chars().nth(4).unwrap();
    let value = line[5..].parse().unwrap();
    let n: i32 = match sign {
        '-' => 0 - value,
        _ => value,
    };
    Instruction { opcode, n }
}

fn run(lines: Vec<Instruction>, patch: Option<Patch>) -> (bool, i32) {
    let mut pos: usize = 0;
    let mut acc = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    let mut infinite_loop: bool = false;
    let len = lines.len();

    loop {
        if pos == len {
            break;
        } else if visited.contains(&pos) {
            infinite_loop = true;
            break;
        } else {
            visited.insert(pos);
        }

        let Instruction { opcode, n } = if let Some(p) = patch {
            if p.line == pos {
                p.instruction
            } else {
                lines[pos]
            }
        } else {
            lines[pos]
        };

        match opcode {
            Opcode::Nop => pos += 1,
            Opcode::Jmp => pos = (pos as i32 + n) as usize,
            Opcode::Acc => {
                pos += 1;
                acc += n
            }
        }
    }

    (infinite_loop, acc)
}

fn part_1(input: String) -> i32 {
    let lines: Vec<Instruction> = input.lines().map(parse_line).collect();
    run(lines, None).1
}

fn part_2(input: String) -> i32 {
    let lines: Vec<Instruction> = input.lines().map(parse_line).collect();

    for (i, instruction) in lines.iter().enumerate() {
        let (infinite_loop, acc) = match instruction.opcode {
            Opcode::Acc => continue,
            Opcode::Nop => run(lines.clone(), Some(Patch::new(i, "jmp", instruction.n))),
            Opcode::Jmp => run(lines.clone(), Some(Patch::new(i, "nop", instruction.n))),
        };
        if !infinite_loop {
            return acc;
        }
    }
    panic!("Couldn't solve")
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
