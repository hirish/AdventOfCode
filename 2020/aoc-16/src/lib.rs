use std::io::{self, Read};
use std::str::FromStr;

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn read_stdin_numbers() -> Vec<u32> {
    read_stdin()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[derive(Debug)]
pub struct Ticket {
    pub numbers: Vec<usize>
}

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect()
        })
    }
}

impl Ticket {
    pub fn valid(&self, rules: &Vec<Rule>) -> bool {
        for number in &self.numbers {
            let mut valid = false;
            for rule in rules {
                if rule.valid(*number) {
                    valid = true
                }
            }
            if !valid {
                return false
            }
        }
        true
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rule {
    pub name: String,
    pub ranges: Vec<(usize, usize)>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.split(":").nth(0).unwrap().to_string(),
            ranges: s
                .split(": ")
                .nth(1).unwrap()
                .split(" or ")
                .map(|x| {
                    let mut y = x.split("-");
                    let a: usize = y.next().unwrap().parse().unwrap();
                    let b: usize = y.next().unwrap().parse().unwrap();
                    (a, b)
                })
                .collect()
        })
    }
}

impl Rule {
    pub fn valid(&self, v: usize) -> bool {
        let mut valid = false;
        for (l, h) in &self.ranges {
            valid |= (v >= *l) && (v <= *h)
        }
        valid
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Constraint<'a> {
    pub locked: bool,
    pub possibilities: Vec<&'a Rule>,
}
