use aoc_1::read_stdin;

type Input = Vec<Vec<usize>>;

fn parse(input: String) -> Input {
    input
        .split("\n\n")
        .map(|l| l.lines().map(|v| v.parse().unwrap()).collect())
        .collect()
}

fn part_1(input: Input) -> usize {
    input.iter().map(|v| v.iter().sum()).max().unwrap()
}

fn part_2(input: Input) -> usize {
    let mut vs: Vec<usize> = input.iter().map(|v| v.iter().sum()).collect();

    vs.sort();
    vs.reverse();
    vs.resize(3, 0);

    vs.iter().sum()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
