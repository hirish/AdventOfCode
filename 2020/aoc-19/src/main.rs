use aoc::read_stdin;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct IsRule {
    val: String,
}

#[derive(Debug, Clone)]
pub struct OrRule {
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct AndRule {
    rule_ids: Vec<usize>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    Is(IsRule),
    Or(OrRule),
    And(AndRule),
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.contains("\"") {
            Ok(Rule::Is(IsRule {
                val: input[1..2].to_string(),
            }))
        } else if input.contains("|") {
            Ok(Rule::Or(OrRule {
                rules: input
                    .split(" | ")
                    .map(|x| x.parse().unwrap())
                    .collect()
            }))
        } else {
            Ok(Rule::And(AndRule {
                rule_ids: input
                    .split(" ")
                    .map(|r| r.parse().unwrap())
                    .collect()
            }))
        }
    }
}

fn match_and(a: &Rule, b: &Rule, msg: &str, rules: &HashMap<usize, Rule>) -> Vec<usize> {
    let mut out = vec![];
    for res_a in a.matches(msg, rules).iter() {
         for res_b in b.matches(&msg[*res_a..], rules).iter() {
             out.push(res_a + res_b)
         }
    }
    out
}

impl Rule {
    pub fn matches(&self, msg: &str, rules: &HashMap<usize, Rule>) -> Vec<usize> {
        match self {
            Rule::Is(rule) => {
                if msg.starts_with(&rule.val) {vec![1]} else {vec![]}
            },
            Rule::Or(rule) => {
                let mut eaten = vec![];
                for rule in &rule.rules {
                    for res in  rule.matches(msg, rules).iter() {
                        eaten.push(*res)
                    }
                }
                eaten
            },
            Rule::And(rule) =>  {
                let rule_ids = &rule.rule_ids;
                match rule_ids.len() {
                    1 => rules.get(&rule_ids[0]).unwrap().matches(msg, rules),
                    2 => match_and(
                        rules.get(&rule_ids[0]).unwrap(),
                        rules.get(&rule_ids[1]).unwrap(),
                        msg,
                        rules
                    ),
                    _ => match_and(
                        rules.get(&rule_ids[0]).unwrap(),
                        &Rule::And(AndRule {
                            rule_ids: rule_ids[1..].iter().map(|x| *x).collect()
                        }),
                        msg,
                        rules
                    )
                }
            },
        }
    }
}

fn parse_rule(input: &str) -> (usize, Rule) {
    let mut split = input.split(": ");
    let id = split.next().unwrap().parse().unwrap();
    let rule = split.next().unwrap().parse().unwrap();
    (id, rule)
}

fn parse(input: &str) -> (HashMap<usize, Rule>, Vec<&str>) {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap().lines().map(parse_rule).collect();
    let messages = split.next().unwrap().lines().collect();
    (rules, messages)
}

fn solve(rules: &HashMap<usize, Rule>, messages: &Vec<&str>) -> usize {
    let rule = rules.get(&0).unwrap();

    messages.iter()
        .filter(|msg| {
            rule
                .matches(msg, rules)
                .iter()
                .filter(|m| **m == msg.len())
                .count()  > 0
        })
        .count()
}

fn main() {
    let input = read_stdin();
    let (mut rules, messages) = parse(&input);
    println!("Answer 1: {}", solve(&rules, &messages));

    rules.insert(8, "42 | 42 8".parse().unwrap());
    rules.insert(11, "42 31 | 42 11 31".parse().unwrap());
    println!("Answer 2: {}", solve(&rules, &messages));
}
