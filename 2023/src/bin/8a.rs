use aoc2024::read_stdin;
use std::{collections::HashMap, time::Instant};

enum Dir {
    L,
    R,
}

type Input = (Vec<Dir>, HashMap<String, (String, String)>);

fn parse(input: String) -> Input {
    let (dirs, network) = input.split_once("\n\n").unwrap();

    let dirs = dirs
        .chars()
        .map(|c| if c == 'L' { Dir::L } else { Dir::R })
        .collect();

    let network = network
        .lines()
        .map(|l| {
            let l = l.strip_suffix(')').unwrap();
            let (from, dirs) = l.split_once(" = (").unwrap();
            let (l, r) = dirs.split_once(", ").unwrap();
            (from.into(), (l.into(), r.into()))
        })
        .collect();

    (dirs, network)
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let (dirs, network) = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let mut curr = "AAA";
    let mut z = 0;
    for i in 0.. {
        if curr == "ZZZ" {
            z = i;
            break;
        }
        let (l, r) = network.get(curr).unwrap();
        curr = match dirs[i % dirs.len()] {
            Dir::L => l,
            Dir::R => r,
        }
    }

    println!("{:?}", z);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
