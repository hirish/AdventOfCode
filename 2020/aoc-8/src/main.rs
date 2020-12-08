use aoc::read_stdin;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
enum AocError {
    OpCodeError(String),
    ProgramError,
}

type Result<T> = std::result::Result<T, AocError>;

#[derive(Copy, Clone)]
enum Opcode {
    Nop,
    Jmp,
    Acc,
}

impl FromStr for Opcode {
    type Err = AocError;

    fn from_str(opcode: &str) -> Result<Self> {
        match opcode {
            "nop" => Ok(Self::Nop),
            "jmp" => Ok(Self::Jmp),
            "acc" => Ok(Self::Acc),
            _ => Err(AocError::OpCodeError(opcode.to_string())),
        }
    }
}

#[derive(Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    n: i32,
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(line: &str) -> Result<Self> {
        let opcode: Opcode = line[0..3].parse()?;
        let sign: char = line.chars().nth(4).unwrap();
        let value = line[5..].parse().unwrap();
        let n: i32 = match sign {
            '-' => 0 - value,
            _ => value,
        };
        Ok(Instruction { opcode, n })
    }
}

#[derive(Copy, Clone)]
struct Patch {
    line: usize,
    instruction: Instruction,
}

impl Patch {
    fn new(line: usize, opcode: &str, n: i32) -> Result<Self> {
        let instruction = Instruction {
            opcode: opcode.parse()?,
            n,
        };
        Ok(Patch { line, instruction })
    }
}

fn parse_lines(input: String) -> Result<Vec<Instruction>> {
    Ok(input.lines()
        .map(|x| x.parse())
        .collect::<Result<Vec<Instruction>>>()?)
}

fn run(lines: &Vec<Instruction>, patch: Option<Patch>) -> (bool, i32) {
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

        let Instruction { opcode, n } = patch
            .filter(|p| p.line == pos)
            .map(|p| p.instruction)
            .unwrap_or(lines[pos]);

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

fn part_1(input: String) -> Result<i32> {
    let instructions = parse_lines(input)?;
    Ok(run(&instructions, None).1)
}

fn part_2(input: String) -> Result<i32> {
    let instructions = parse_lines(input)?;

    for (i, instruction) in instructions.iter().enumerate() {
        let (infinite_loop, acc) = match instruction.opcode {
            Opcode::Acc => continue,
            Opcode::Nop => run(&instructions, Some(Patch::new(i, "jmp", instruction.n)?)),
            Opcode::Jmp => run(&instructions, Some(Patch::new(i, "nop", instruction.n)?)),
        };
        if !infinite_loop {
            return Ok(acc);
        }
    }
    Err(AocError::ProgramError)
}

fn main() -> Result<()> {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone())?);
    println!("Answer 2: {}", part_2(lines.clone())?);
    Ok(())
}
