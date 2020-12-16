use aoc::{read_stdin, Ticket, Rule, Constraint};

fn parse(input: String) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut input = input.split("\n\n");

    let rules: Vec<Rule> = input
        .next().unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let ticket: Ticket = input
        .next().unwrap()
        .lines()
        .nth(1).unwrap()
        .parse().unwrap();

    let other_tickets: Vec<Ticket> = input
        .next().unwrap()
        .lines()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    (rules, ticket, other_tickets)
}

fn part_1(rules: &Vec<Rule>, other: &Vec<Ticket>) -> usize {
    let mut total = 0;
    for ticket in other {
        for number in &ticket.numbers {
            let mut valid = false;
            for rule in rules {
                if rule.valid(*number) {
                    valid = true;
                    break
                }
            }
            if !valid {
                total += number;
            }
        }
    }
    total
}

fn solved(constraints: &Vec<Constraint>) -> bool {
    for constraint in constraints {
        if !constraint.locked {
            return false
        }
    }
    return true
}

fn step_solve<'a>(possibilities: &mut Vec<Constraint<'a>>) -> Vec<Constraint<'a>> {
    let first_unitary_unlocked = possibilities
        .iter()
        .filter(|p| !p.locked && p.possibilities.len() == 1)
        .next().unwrap();

    let to_remove = first_unitary_unlocked.possibilities.first().unwrap();

    possibilities.iter()
        .map(|p| {
            if p == first_unitary_unlocked || p.locked {
                Constraint {
                    locked: true,
                    possibilities: p.possibilities.clone(),
                }
            } else {
                Constraint {
                    locked: false,
                    possibilities: p.possibilities
                        .iter()
                        .map(|q| *q)
                        .filter(|q| q != to_remove)
                        .collect()
                }
            }
        })
        .collect()

}

fn part_2(rules: &Vec<Rule>, ticket: &Ticket, other: &Vec<Ticket>) -> usize {
    let other: Vec<&Ticket> = other
        .iter()
        .filter(|t| t.valid(rules))
        .collect();

    let mut possibilities: Vec<Constraint> = (0..ticket.numbers.len()).map(|i| {
        Constraint {
            locked: false,
            possibilities: rules.iter().filter(|rule| {
                for ticket in &other {
                    if !rule.valid(ticket.numbers[i]) {
                        return false
                    }
                }
                true
            }).collect()
        }
    }).collect();

    while !solved(&possibilities) {
        possibilities = step_solve(&mut possibilities);
    }

    let mut x = 1;
    for (i, constraint) in possibilities.iter().enumerate() {
        let rule = constraint.possibilities.first().unwrap();
        if rule.name.starts_with("departure") {
            x *= ticket.numbers.iter().nth(i).unwrap()
        }
    }

    x
}

fn main() {
    let (rules, ticket, other_tickets) = parse(read_stdin());

    println!("Answer 1: {}", part_1(&rules, &other_tickets));
    println!("Answer 2: {}", part_2(&rules, &ticket, &other_tickets));
}
