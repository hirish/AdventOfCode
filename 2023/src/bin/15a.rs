use std::time::Instant;

use aoc2024::read_stdin;
use itertools::Itertools;

type Input = Vec<String>;

fn parse(input: String) -> Input {
    input.split(",").map_into().collect()
}

fn hash(s: &str) -> usize {
    let mut v = 0;
    for c in s.chars() {
        v += c as usize;
        v *= 17;
        v %= 256;
    }
    v
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let input = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let v: usize = input.iter().map(|s| hash(s)).sum();

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
