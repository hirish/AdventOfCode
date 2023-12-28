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
    fn apply(self, p: Part) -> Vec<(Part, Option<Dest>)> {
        let Part { x, m, a, s } = p;
        match self {
            Rule::LT(Cat::X, n, d) => {
                if x.0 > n {
                    vec![]
                } else if x.1 < n {
                    vec![(p, Some(d.clone()))]
                } else {
                    vec![
                        (p.with_x((x.0, n - 1)), Some(d.clone())),
                        (p.with_x((n, x.1)), None),
                    ]
                }
            }
            Rule::LT(Cat::M, n, d) => {
                if m.0 > n {
                    vec![]
                } else if m.1 < n {
                    vec![(p, Some(d))]
                } else {
                    vec![
                        (p.with_m((m.0, n - 1)), Some(d)),
                        (p.with_m((n, m.1)), None),
                    ]
                }
            }
            Rule::LT(Cat::A, n, d) => {
                if a.0 > n {
                    vec![]
                } else if a.1 < n {
                    vec![(p, Some(d))]
                } else {
                    vec![
                        (p.with_a((a.0, n - 1)), Some(d)),
                        (p.with_a((n, a.1)), None),
                    ]
                }
            }
            Rule::LT(Cat::S, n, d) => {
                if s.0 > n {
                    vec![]
                } else if s.1 < n {
                    vec![(p, Some(d))]
                } else {
                    vec![
                        (p.with_s((s.0, n - 1)), Some(d)),
                        (p.with_s((n, s.1)), None),
                    ]
                }
            }
            Rule::GT(Cat::X, n, d) => {
                if x.1 < n {
                    vec![]
                } else if x.0 > n {
                    vec![(p, Some(d))]
                } else {
                    vec![
                        (p.with_x((n + 1, x.1)), Some(d)),
                        (p.with_x((x.0, n)), None),
                    ]
                }
            }
            Rule::GT(Cat::M, n, d) => {
                if m.1 < n {
                    vec![]
                } else if m.0 > n {
                    vec![(p, Some(d))]
                } else {
                    vec![
                        (p.with_m((n + 1, m.1)), Some(d)),
                        (p.with_m((m.0, n)), None),
                    ]
                }
            }
            Rule::GT(Cat::A, n, d) => {
                if a.1 < n {
                    vec![]
                } else if a.0 > n {
                    vec![(p, Some(d))]
                } else {
                    vec![
                        (p.with_a((n + 1, a.1)), Some(d)),
                        (p.with_a((a.0, n)), None),
                    ]
                }
            }
            Rule::GT(Cat::S, n, d) => {
                if s.1 < n {
                    vec![]
                } else if s.0 > n {
                    vec![(p, Some(d))]
                } else {
                    vec![
                        (p.with_s((n + 1, s.1)), Some(d)),
                        (p.with_s((s.0, n)), None),
                    ]
                }
            }
            Rule::Label(d) => vec![(p, Some(d))],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: (isize, isize),
    m: (isize, isize),
    a: (isize, isize),
    s: (isize, isize),
}

impl Part {
    fn with_x(&self, x: (isize, isize)) -> Self {
        Part {
            x,
            m: self.m,
            a: self.a,
            s: self.s,
        }
    }
    fn with_m(&self, m: (isize, isize)) -> Self {
        Part {
            x: self.x,
            m,
            a: self.a,
            s: self.s,
        }
    }
    fn with_a(&self, a: (isize, isize)) -> Self {
        Part {
            x: self.x,
            m: self.m,
            a,
            s: self.s,
        }
    }
    fn with_s(&self, s: (isize, isize)) -> Self {
        Part {
            x: self.x,
            m: self.m,
            a: self.a,
            s,
        }
    }
}

type Input = HashMap<String, Vec<Rule>>;

fn parse(input: String) -> Input {
    let (rules, _) = input.split_once("\n\n").unwrap();
    rules
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
        .collect()
}

fn count(map: &Input, curr: &str, mut p: Part) -> isize {
    let rule = map.get(curr).unwrap();

    let mut sum = 0;

    for r in rule {
        let mut new_part = None;
        for (p, dest) in r.clone().apply(p) {
            if let Some(dest) = dest {
                match dest {
                    Dest::Label(d) => sum += count(map, &d, p),
                    Dest::Acc => {
                        sum += (1 + p.x.1 - p.x.0)
                            * (1 + p.m.1 - p.m.0)
                            * (1 + p.a.1 - p.a.0)
                            * (1 + p.s.1 - p.s.0)
                    }
                    Dest::Rej => {}
                }
            } else {
                new_part = Some(p)
            }
        }
        if let Some(np) = new_part {
            p = np
        } else {
            break;
        }
    }

    sum
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let map = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let part = Part {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let z = count(&map, "in", part);

    println!("{:?}", z);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
