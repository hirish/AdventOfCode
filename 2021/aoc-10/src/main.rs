use aoc_1::read_stdin;

fn get_error(line: &str) -> Option<char> {
    let mut b: Vec<char> = vec![];

    for c in line.chars() {
        if c == '(' || c == '{' || c == '[' || c == '<' {
            b.push(c)
        } else {
            let prev = b.pop()?;
            match (prev, c) {
                ('(', ')') | ('{', '}') | ('[', ']') | ('<', '>') => {}
                _ => return Some(c),
            }
        }
    }

    None
}

fn part_1(input: String) -> usize {
    input
        .lines()
        .filter_map(get_error)
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        })
        .sum()
}

fn incomplete_lines(line: &str) -> Option<usize> {
    let mut b: Vec<char> = vec![];

    for c in line.chars() {
        if c == '(' || c == '{' || c == '[' || c == '<' {
            b.push(c)
        } else {
            let prev = b.pop()?;
            match (prev, c) {
                ('(', ')') | ('{', '}') | ('[', ']') | ('<', '>') => {}
                _ => return None,
            }
        }
    }

    let mut score = 0;
    b.reverse();

    for c in b {
        score = score * 5;
        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!(),
        }
    }

    Some(score)
}

fn part_2(input: String) -> usize {
    let mut scores: Vec<usize> = input.lines().filter_map(incomplete_lines).collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input = read_stdin();

    println!("Answer 1: {}", part_1(input.clone()));
    println!("Answer 2: {}", part_2(input));
}
