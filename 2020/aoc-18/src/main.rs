use aoc::read_stdin;
use regex::Regex;

fn simple_solve(input: Vec<&str>) -> usize {
    let mut op = "+";
    let mut curr: usize = 0;

    for c in input.iter() {
        match *c {
            "+" | "*" => op = c,
            x => {
                let val: usize = x.parse().unwrap();
                match op {
                    "+" => curr += val,
                    "*" => curr *= val,
                    _ => panic!("Unknown op! {}", op)
                }
            }
        };
    }
    curr
}

fn remove_brackets(input: &str, solver: fn(&str)->usize) -> String {
    let within_brackets = Regex::new(r"\(([^\)^(]*)\)").unwrap();

    let mut val = input.to_string();
    while val.contains("(") {
        for x in within_brackets.captures_iter(&val.clone()) {
            let v = solver(&x[1]);
            val = val.replace(&x[0], &format!("{}", v))
        }
    }
    val
}

fn solve(input: &str) -> usize {
    let input = remove_brackets(input, solve);
    simple_solve(input.split(" ").collect())
}

fn solve_with_precedence(input: &str) -> usize {
    let add = Regex::new(r"(\d+ \+ \d+)").unwrap();
    let mut input = remove_brackets(input, solve_with_precedence);
    while input.contains("+") {
        input = add.replace(&input, r"($0)").to_string();
        input = remove_brackets(&input, solve);
    }
    simple_solve(input.split(" ").collect())
}

fn part_1(input: &str) -> usize {
    input.lines().map(solve).sum()
}

fn part_2(input: &str) -> usize {
    input.lines().map(solve_with_precedence).sum()
}

fn main() {
    let input = read_stdin();

    println!("Answer 1: {}", part_1(&input));
    println!("Answer 2: {}", part_2(&input));
}
