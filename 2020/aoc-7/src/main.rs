use aoc::read_stdin;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn parse_bag(bag: &str) -> String {
    let re = Regex::new(r" bags?.?$").unwrap();
    re.replace_all(bag.trim(), "").to_string()
}

fn parse_rule(input: &str) -> (String, HashMap<String, usize>) {
    let s: Vec<&str> = input.split(" contain ").collect();
    let mut can_contain = HashMap::new();

    for bag in s[1].split(", ") {
        if bag == "no other bags." {
            continue;
        }

        let count: usize = bag[0..1].parse().unwrap();
        let bag = parse_bag(&bag[2..]);
        can_contain.insert(bag, count);
    }

    (parse_bag(s[0]), can_contain)
}

fn construct_rules(input: &str) -> HashMap<String, HashMap<String, usize>> {
    input.lines().map(parse_rule).collect()
}

fn can_contain_gold(
    bag: &str,
    rules: &HashMap<String, HashMap<String, usize>>,
    cache: &HashSet<&str>,
) -> bool {
    if cache.contains(bag) {
        return true;
    }

    for b in rules.get(bag).unwrap().keys() {
        if can_contain_gold(b, rules, cache) {
            return true;
        }
    }

    return false;
}

fn count_subbags(bag: &str, rules: &HashMap<String, HashMap<String, usize>>) -> usize {
    let x: usize = rules[bag]
        .iter()
        .map(|(b, c)| c * count_subbags(b, rules))
        .sum();
    1 + x
}

fn part_1(input: String) -> usize {
    let rules = construct_rules(&input);
    let mut bags_which_can_contain_gold: HashSet<&str> = HashSet::new();
    bags_which_can_contain_gold.insert("shiny gold");

    for bag in rules.keys() {
        if can_contain_gold(bag, &rules, &bags_which_can_contain_gold) {
            bags_which_can_contain_gold.insert(bag);
        }
    }
    bags_which_can_contain_gold.len() - 1
}

fn part_2(input: String) -> usize {
    let rules = construct_rules(&input);
    count_subbags("shiny gold", &rules) - 1
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
