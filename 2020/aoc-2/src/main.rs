use aoc::read_stdin;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl Policy {
    fn new(unparsed: &str) -> Self {
        let x: Vec<&str> = unparsed.split(' ').collect();
        let y: Vec<usize> = x[0].split('-').map(|z| z.parse().unwrap()).collect();

        Policy {
            min: y[0],
            max: y[1],
            letter: x[1].chars().next().unwrap(),
        }
    }

    fn is_valid(&self, password: &str) -> bool {
        let count = password
            .chars()
            .filter(|c| *c == self.letter)
            .collect::<Vec<char>>()
            .len();
        count >= self.min && count <= self.max
    }

    fn is_valid_2(&self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();
        (chars[self.min - 1] == self.letter) ^ (chars[self.max - 1] == self.letter)
    }
}

fn parse(line: &str) -> (Policy, &str) {
    let s: Vec<&str> = line.split(':').collect();
    (Policy::new(s[0]), s[1].trim())
}

fn part_1(lines: String) -> usize {
    lines.lines()
        .map(parse)
        .filter(|(p, pw)| p.is_valid(pw))
        .collect::<Vec<(Policy, &str)>>()
        .len()
}

fn part_2(lines: String) -> usize {
    lines.lines()
        .map(parse)
        .filter(|(p, pw)| p.is_valid_2(pw))
        .collect::<Vec<(Policy, &str)>>()
        .len()
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
