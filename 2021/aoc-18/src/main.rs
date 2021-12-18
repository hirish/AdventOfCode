use serde_json;
use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use aoc_1::read_stdin;

#[derive(Clone)]
enum Value {
    Pair(Box<Value>, Box<Value>),
    Number(usize),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let v: serde_json::Value = serde_json::from_str(input).unwrap();
        match v {
            serde_json::Value::Array(vs) => {
                let a = vs[0].to_string().parse().unwrap();
                let b = vs[1].to_string().parse().unwrap();

                Ok(Self::Pair(Box::new(a), Box::new(b)))
            }
            serde_json::Value::Number(n) => Ok(Self::Number(n.as_u64().unwrap() as usize)),
            _ => panic!(),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(v) => write!(f, "{}", v),
            Self::Pair(a, b) => write!(f, "[{:?},{:?}]", a, b),
        }
    }
}

impl Value {
    fn add_l(self, x: usize) -> Self {
        match self {
            Self::Number(v) => Self::Number(v + x),
            Self::Pair(a, b) => Self::Pair(Box::new(a.add_l(x)), b),
        }
    }

    fn add_r(self, x: usize) -> Self {
        match self {
            Self::Number(v) => Self::Number(v + x),
            Self::Pair(a, b) => Self::Pair(a, Box::new(b.add_r(x))),
        }
    }

    fn explode(self, depth: usize) -> (Self, bool, Option<usize>, Option<usize>) {
        match self {
            Self::Pair(a, b) => {
                if depth < 4 {
                    let (a, a_exploded, a_left, a_right) = a.explode(depth + 1);

                    let (b, b_exploded, b_left, b_right) = if a_exploded {
                        (*b, false, None, None)
                    } else {
                        b.explode(depth + 1)
                    };

                    let l = match b_left {
                        Some(b_left) => a.add_r(b_left),
                        None => a,
                    };

                    let r = match a_right {
                        Some(a_right) => b.add_l(a_right),
                        None => b,
                    };

                    let new = Self::Pair(Box::new(l), Box::new(r));

                    (new, a_exploded || b_exploded, a_left, b_right)
                } else {
                    match (*a, *b) {
                        (Self::Number(x), Self::Number(y)) => {
                            (Self::Number(0), true, Some(x), Some(y))
                        }
                        _ => panic!("Exploding something with greater depth than 4"),
                    }
                }
            }
            v => (v, false, None, None),
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Self::Pair(a, b) => {
                let (a, a_split) = a.split();
                let (b, b_split) = if a_split { (*b, false) } else { b.split() };
                (Self::Pair(Box::new(a), Box::new(b)), a_split || b_split)
            }
            Self::Number(v) => {
                if v >= 10 {
                    let a = Self::Number(v / 2);
                    let b = Self::Number(v / 2 + v % 2);
                    (Self::Pair(Box::new(a), Box::new(b)), true)
                } else {
                    (Self::Number(v), false)
                }
            }
        }
    }

    fn reduce(self) -> Self {
        let (v, exploded, _, _) = self.explode(0);
        if exploded {
            return v.reduce();
        }

        let (v, split) = v.split();
        if split {
            return v.reduce();
        }

        v
    }

    fn add(self, other: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(other))
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Number(v) => *v,
            Self::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    values: Vec<Value>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let values = input.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self { values })
    }
}

fn part_1(input: Input) -> Option<usize> {
    let mut x = input.values.into_iter();
    let mut c = x.next()?;
    for v in x {
        c = c.add(v);
        c = c.reduce();
    }
    Some(c.magnitude())
}

fn part_2(input: Input) -> Option<usize> {
    let mut max = 0;
    for i in 0..input.values.len() {
        for j in 0..input.values.len() {
            if i == j {
                continue;
            }
            let a = input.values[i].clone();
            let b = input.values[j].clone();
            let v = a.add(b).reduce().magnitude();
            if v > max {
                max = v;
            }
        }
    }
    Some(max)
}

fn main() -> Result<(), ()> {
    let now = Instant::now();
    let input: Input = read_stdin().parse()?;
    println!("Running parsing took {}μs.", now.elapsed().as_micros());

    let now = Instant::now();
    println!("Answer 1: {:?}", part_1(input.clone()).ok_or(())?);
    println!("Running part_1 took {}μs.", now.elapsed().as_micros());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(input).ok_or(())?);
    println!("Running part_2 took {}μs.", now.elapsed().as_micros());

    Ok(())
}
