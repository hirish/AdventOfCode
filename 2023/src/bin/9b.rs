use aoc2024::read_stdin;
use std::time::Instant;

type Input = Vec<Vec<isize>>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn predict(seq: Vec<isize>) -> isize {
    let mut diffs: Vec<isize> = Vec::new();
    let mut all_zero = true;
    for i in 0..(seq.len() - 1) {
        let diff = seq[i + 1] - seq[i];
        all_zero = all_zero && (diff == 0);
        diffs.push(diff);
    }

    if all_zero {
        seq[0]
    } else {
        seq.first().unwrap() - predict(diffs)
    }
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let z: isize = lines.into_iter().map(|l| predict(l)).sum();

    println!("{:?}", z);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
