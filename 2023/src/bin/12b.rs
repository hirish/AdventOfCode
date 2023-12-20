use std::time::Instant;

use aoc2024::read_stdin;

type Input = Vec<(Vec<String>, Vec<usize>)>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();

            let left = format!("{}?{}?{}?{}?{}", left, left, left, left, left)
                .replace(".", " ")
                .split_whitespace()
                .map(|s| s.into())
                .collect();

            let right = format!("{},{},{},{},{}", right, right, right, right, right)
                .split(',')
                .filter_map(|c| c.parse().ok())
                .collect();

            (left, right)
        })
        .collect()
}

#[memoize::memoize]
fn solve_group(group: String, n: usize) -> Vec<String> {
    let mut new_groups = vec![];

    if n > group.len() {
        return new_groups;
    }

    for i in 0..(group.len() + 1 - n) {
        if let Some("#") = group.get(i - 1..i) {
            break;
        }

        if let Some("#") = group.get(i + n..i + n + 1) {
            continue;
        }

        if let Some(g) = group.get(i + n + 1..) {
            new_groups.push(g.into())
        } else {
            new_groups.push("".into())
        }
    }

    new_groups
}

#[memoize::memoize]
fn solve(target: Vec<String>, soln: Vec<usize>) -> usize {
    if soln.len() == 0 {
        if target.iter().any(|s| s.contains('#')) {
            return 0;
        } else {
            return 1;
        }
    }

    let to_solve = soln[0];

    let mut count = 0;
    for (i, group) in target.iter().enumerate() {
        for new_group in solve_group(group.into(), to_solve) {
            let mut v = if new_group.len() > 0 {
                vec![new_group]
            } else {
                vec![]
            };
            if let Some(x) = target.get((i + 1)..) {
                v.append(&mut x.to_owned());
            }
            count += solve(v, soln[1..].to_owned());
        }

        if group.contains('#') {
            break;
        }
    }

    count
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let val: usize = lines.into_iter().map(|l| solve(l.0, l.1)).sum();

    println!("{:?}", val);
    println!("Parse time\t{}ms.", parse_time.as_millis());
    println!("Execution time\t{}ms.", parsed.elapsed().as_millis());
    println!("Total time\t{}ms.", start.elapsed().as_millis());
}
