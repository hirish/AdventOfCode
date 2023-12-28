use std::time::Instant;

use aoc2024::read_stdin;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Instruction {
    U(isize),
    D(isize),
    L(isize),
    R(isize),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coord(isize, isize);

type Input = Vec<Instruction>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| {
            let l = l.strip_suffix(')').unwrap();
            let (_, d) = l.split_once(" (#").unwrap();
            let dist = &d[0..5];
            let dir = &d[5..6];

            match dir {
                "3" => Instruction::U(isize::from_str_radix(dist, 16).unwrap()),
                "1" => Instruction::D(isize::from_str_radix(dist, 16).unwrap()),
                "2" => Instruction::L(isize::from_str_radix(dist, 16).unwrap()),
                "0" => Instruction::R(isize::from_str_radix(dist, 16).unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let instructions = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let mut area = 1;
    let mut pos = Coord(0, 0);

    let mut perimeter = 0;
    for instruction in instructions {
        match instruction {
            Instruction::U(n) => {
                perimeter += n;
                pos = Coord(pos.0 - n, pos.1);
                area -= n * pos.1;
            }
            Instruction::D(n) => {
                perimeter += n;
                pos = Coord(pos.0 + n, pos.1);
                area += n * pos.1;
            }
            Instruction::L(n) => {
                perimeter += n;
                pos = Coord(pos.0, pos.1 - n);
            }
            Instruction::R(n) => {
                perimeter += n;
                pos = Coord(pos.0, pos.1 + n);
            }
        }
    }
    area += perimeter / 2;

    println!("{:?}", area);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
