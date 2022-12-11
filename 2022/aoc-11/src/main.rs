use aoc_11::read_stdin;

#[derive(Clone, Debug)]
enum Op {
    Add(usize),
    Mult(usize),
    Square,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    prime: usize,
    if_true: usize,
    if_false: usize,
}

type Input = Vec<Monkey>;

fn parse(input: String) -> Input {
    input
        .split("\n\n")
        .map(|l| {
            let x: Vec<&str> = l.lines().collect();

            let items = x[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|v| v.parse().unwrap())
                .collect();

            let last_value = x[2].split(' ').last().unwrap();
            let op = if last_value == "old" {
                Op::Square
            } else if x[2].contains('+') {
                Op::Add(last_value.parse().unwrap())
            } else {
                Op::Mult(last_value.parse().unwrap())
            };

            Monkey {
                items,
                op,
                prime: x[3].split(" by ").last().unwrap().parse().unwrap(),
                if_true: x[4].chars().last().unwrap().to_digit(10).unwrap() as usize,
                if_false: x[5].chars().last().unwrap().to_digit(10).unwrap() as usize,
            }
        })
        .collect()
}

fn iterate(mut monkeys: Input, count: usize, reducer: &dyn Fn(usize) -> usize) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..count {
        for j in 0..monkeys.len() {
            loop {
                let Monkey { items, op, prime, if_true, if_false, } = &mut monkeys[j];
                if items.is_empty() { break; }
                inspections[j] += 1;
                let item = items.pop().unwrap();
                let val = reducer(match op {
                    Op::Square => item * item,
                    Op::Add(x) => item + *x,
                    Op::Mult(x) => item * *x,
                });
                let throw_to = *if (val % *prime) == 0 { if_true } else { if_false };
                monkeys[throw_to].items.push(val);
            }
        }
    }

    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn part_1(input: Input) -> usize {
    let reducer = |x: usize| x / 3;
    iterate(input, 20, &reducer)
}

fn part_2(input: Input) -> usize {
    let primes: usize = input.iter().map(|i| i.prime).product();
    let reducer = |x: usize| x % primes;
    iterate(input, 10000, &reducer)
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
