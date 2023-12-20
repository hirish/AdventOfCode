use regex::Regex;
use std::time::Instant;

use aoc2024::read_stdin;

type Input = Vec<(String, Regex)>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();
            let left = left.replace(".", " ");
            let right = &right
                .split(',')
                .map(|c| c.parse().unwrap())
                .map(|n| "#".repeat(n))
                .collect::<Vec<String>>()
                .join(" +");

            let right = Regex::new(&format!("^ *{} *$", right)).unwrap();

            (left.into(), right)
        })
        .collect()
}

fn expand(target: String) -> Vec<String> {
    if !target.contains('?') {
        return vec![target];
    }

    let mut a = expand(target.replacen('?', "#", 1));
    let mut b = expand(target.replacen('?', " ", 1));

    a.append(&mut b);

    a
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let val: usize = lines
        .into_iter()
        .map(|(target, soln)| {
            expand(target)
                .into_iter()
                .filter(|s| soln.is_match(s))
                .count()
        })
        .sum();

    println!("{:?}", val);
    println!("Parse time\t{}ms.", parse_time.as_millis());
    println!("Execution time\t{}ms.", parsed.elapsed().as_millis());
    println!("Total time\t{}ms.", start.elapsed().as_millis());
}
