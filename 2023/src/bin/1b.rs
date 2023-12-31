use std::time::Instant;

use aoc2024::read_stdin;

type Input = Vec<String>;

fn parse(input: String) -> Input {
    input.lines().map(|l| l.to_string()).collect()
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn lfind(s: &str) -> u32 {
    let i = s.find(char::is_numeric);
    let j = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(x, d)| s.find(d).map(|i| (i, (x + 1) as u32)))
        .min_by_key(|(i, _)| i.clone());

    match (i, j) {
        (None, None) => panic!("No Left Digit"),
        (None, Some((_, x))) => x,
        (Some(i), None) => s.chars().nth(i).unwrap().to_digit(10).unwrap(),
        (Some(i), Some((j, x))) => {
            if i < j {
                s.chars().nth(i).unwrap().to_digit(10).unwrap()
            } else {
                x
            }
        }
    }
}

fn rfind(s: &str) -> u32 {
    let i = s.rfind(char::is_numeric);
    let j = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(x, d)| s.rfind(d).map(|i| (i, (x + 1) as u32)))
        .max_by_key(|(i, _)| i.clone());

    match (i, j) {
        (None, None) => panic!("No Right Digit"),
        (None, Some((_, x))) => x,
        (Some(i), None) => s.chars().nth(i).unwrap().to_digit(10).unwrap(),
        (Some(i), Some((j, x))) => {
            if i > j {
                s.chars().nth(i).unwrap().to_digit(10).unwrap()
            } else {
                x
            }
        }
    }
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let v: u32 = lines
        .into_iter()
        .map(|s| {
            let l = lfind(&s);
            let r = rfind(&s);
            l * 10 + r
        })
        .sum();

    println!("{}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
