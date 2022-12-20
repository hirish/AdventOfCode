use aoc_19::read_stdin;
use std::time::Instant;

type N = (usize, isize);
type Input = Vec<N>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .enumerate()
        .collect()
}

fn shuffle(mut input: Input) -> Input {
    let mut j = 0;
    for i in 0..input.len() {
        let mut v = input[j];
        while v.0 != i {
            j = (j + 1) % input.len();
            v = input[j]
        }

        input.remove(j);
        let new_pos = ((j as isize) + v.1 - 1).rem_euclid(input.len() as isize) + 1;
        input.insert(new_pos as usize, v);
    }
    input
}

fn part_1(mut input: Input) -> isize {
    input = shuffle(input);
    let l = input.iter().position(|v| v.1 == 0).unwrap();
    input[(l + 1000) % input.len()].1
        + input[(l + 2000) % input.len()].1
        + input[(l + 3000) % input.len()].1
}

fn part_2(mut input: Input) -> isize {
    input = input.into_iter().map(|v| (v.0, v.1 * 811589153)).collect();

    for _ in 0..10 {
        input = shuffle(input);
    }

    let l = input.iter().position(|v| v.1 == 0).unwrap();
    input[(l + 1000) % input.len()].1
        + input[(l + 2000) % input.len()].1
        + input[(l + 3000) % input.len()].1
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
    let elapsed_time = now.elapsed();
    println!("Running parsing took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(lines.clone()));
    let elapsed_time = now.elapsed();
    println!("Running part_1 took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(lines));
    println!("Running part_2 took {}ms.", now.elapsed().as_millis());
}
