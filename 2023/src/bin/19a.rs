use std::{collections::HashMap, str::FromStr, time::Instant};

use aoc2024::read_stdin;

#[derive(Debug, Clone, Copy)]
enum Cat {
    X,
    M,
    A,
    S,
}

impl FromStr for Cat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum Dest {
    Label(String),
    Acc,
    Rej,
}

impl FromStr for Dest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Acc),
            "R" => Ok(Self::Rej),
            _ => Ok(Self::Label(s.into())),
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    LT(Cat, isize, Dest),
    GT(Cat, isize, Dest),
    Label(Dest),
}

impl Rule {
    fn apply(&self, &Part { x, m, a, s }: &Part) -> Option<&Dest> {
        match self {
            Rule::LT(c, n, d) => match c {
                Cat::X => {
                    if x < *n {
                        Some(d)
                    } else {
                        None
                    }
                }
                Cat::M => {
                    if m < *n {
                        Some(d)
                    } else {
                        None
                    }
                }
                Cat::A => {
                    if a < *n {
                        Some(d)
                    } else {
                        None
                    }
                }
                Cat::S => {
                    if s < *n {
                        Some(d)
                    } else {
                        None
                    }
                }
            },
            Rule::GT(c, n, d) => match c {
                Cat::X => {
                    if x > *n {
                        Some(d)
                    } else {
                        None
                    }
                }
                Cat::M => {
                    if m > *n {
                        Some(d)
                    } else {
                        None
                    }
                }
                Cat::A => {
                    if a > *n {
                        Some(d)
                    } else {
                        None
                    }
                }
                Cat::S => {
                    if s > *n {
                        Some(d)
                    } else {
                        None
                    }
                }
            },
            Rule::Label(d) => Some(d),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

type Input = (HashMap<String, Vec<Rule>>, Vec<Part>);

fn parse(input: String) -> Input {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let l = l.strip_suffix('}').unwrap();
            let (name, rules) = l.split_once('{').unwrap();
            let rules = rules
                .split(",")
                .map(|r| {
                    if r.len() > 1 && (&r[1..2] == ">" || &r[1..2] == "<") {
                        let (guard, label) = r.split_once(":").unwrap();
                        let cat = guard[0..1].parse().unwrap();
                        let n = guard[2..].parse().unwrap();
                        let dest = label.parse().unwrap();
                        if &r[1..2] == ">" {
                            Rule::GT(cat, n, dest)
                        } else {
                            Rule::LT(cat, n, dest)
                        }
                    } else {
                        Rule::Label(r.parse().unwrap())
                    }
                })
                .collect();

            (name.into(), rules)
        })
        .collect();

    let parts = parts
        .lines()
        .map(|l| {
            let l = l.strip_prefix("{x=").unwrap();
            let (x, l) = l.split_once(",m=").unwrap();
            let (m, l) = l.split_once(",a=").unwrap();
            let (a, l) = l.split_once(",s=").unwrap();
            let s = l.strip_suffix("}").unwrap();
            Part {
                x: x.parse().unwrap(),
                m: m.parse().unwrap(),
                a: a.parse().unwrap(),
                s: s.parse().unwrap(),
            }
        })
        .collect();

    (rules, parts)
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let (rules, parts) = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let z: isize = parts
        .into_iter()
        .filter(|p| {
            let mut rule = rules.get("in").unwrap();
            loop {
                for r in rule {
                    if let Some(dest) = r.apply(p) {
                        match dest {
                            Dest::Label(d) => {
                                rule = rules.get(d).unwrap();
                                break;
                            }
                            Dest::Acc => return true,
                            Dest::Rej => return false,
                        }
                    }
                }
            }
        })
        .map(|Part { x, m, a, s }| x + m + a + s)
        .sum();

    println!("{:?}", z);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
