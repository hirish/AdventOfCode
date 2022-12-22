use aoc_19::read_stdin;

use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Debug, Copy)]
enum Line<'a> {
    N(isize),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

type Input<'a> = HashMap<&'a str, Line<'a>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (k, l) = l.split_once(": ").unwrap();

            let v = if l.contains(' ') {
                let l: Vec<&str> = l.split(' ').collect();

                match l[1] {
                    "+" => Line::Add(l[0], l[2]),
                    "-" => Line::Sub(l[0], l[2]),
                    "*" => Line::Mul(l[0], l[2]),
                    "/" => Line::Div(l[0], l[2]),
                    _ => panic!("Unknown symbol"),
                }
            } else {
                Line::N(l.parse().unwrap())
            };
            (k, v)
        })
        .collect()
}

fn part_1(input: Input) -> isize {
    eval("root", &input)
}

fn eval(i: &str, input: &Input) -> isize {
    match input[i] {
        Line::N(v) => v,
        Line::Add(a, b) => eval(a, input) + eval(b, input),
        Line::Sub(a, b) => eval(a, input) - eval(b, input),
        Line::Div(a, b) => eval(a, input) / eval(b, input),
        Line::Mul(a, b) => eval(a, input) * eval(b, input),
    }
}

enum Res {
    C(isize),
    S(String),
}

impl std::fmt::Debug for Res {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C(arg0) => write!(f, "{}", arg0),
            Self::S(arg0) => write!(f, "{}", arg0),
        }
    }
}

fn handle(input: &Input, a: &str, b: &str, symbol: char, f: &dyn Fn(isize, isize) -> isize) -> Res {
    let a = eval_2(a, input);
    let b = eval_2(b, input);
    match (a, b) {
        (Res::C(a), Res::C(b)) => Res::C(f(a, b)),
        (Res::S(a), Res::C(b)) => Res::S(format!("({} {} {})", a, symbol, b)),
        (Res::S(a), Res::S(b)) => Res::S(format!("({} {} {})", a, symbol, b)),
        (Res::C(a), Res::S(b)) => Res::S(format!("({} {} {})", a, symbol, b)),
    }
}

fn eval_2(i: &str, input: &Input) -> Res {
    if i == "humn" {
        return Res::S("x".to_string());
    }

    match input[i] {
        Line::N(v) => Res::C(v),
        Line::Add(a, b) => handle(input, a, b, '+', &|a, b| a + b),
        Line::Sub(a, b) => handle(input, a, b, '-', &|a, b| a - b),
        Line::Mul(a, b) => handle(input, a, b, '*', &|a, b| a * b),
        Line::Div(a, b) => handle(input, a, b, '/', &|a, b| a / b),
    }
}

fn part_2(mut input: Input) -> isize {
    let (a, b) = match input["root"] {
        Line::N(_) => panic!(),
        Line::Add(a, b) => (a, b),
        Line::Sub(a, b) => (a, b),
        Line::Mul(a, b) => (a, b),
        Line::Div(a, b) => (a, b),
    };
    input.insert("root", Line::Sub(a, b));

    println!("{:?} = 0", eval_2("root", &input));
    1
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let lines = parse(&input);
    let elapsed_time = now.elapsed();
    println!("Running parsing took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(lines.clone()));
    let elapsed_time = now.elapsed();
    println!("Running part_1 took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(lines));
    println!("Running part_2 took {}ms.", now.elapsed().as_millis());
}
