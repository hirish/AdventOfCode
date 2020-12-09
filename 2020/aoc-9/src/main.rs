use aoc::read_stdin;
use std::iter::Sum;

// const N: usize = 25;
const N: usize = 25;

fn convert_str(input: String) -> Vec<i64> {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn valid(numbers: &Vec<i64>, from: usize, to: usize, target: i64) -> bool {
    for x in from..to {
        for y in from..to {
            if x == y {continue}
            if numbers[x] + numbers[y] == target { return true }
        }
    }
    return false
}

fn part_1(input: String) -> i64 {
    let numbers = convert_str(input);
    for i in N..numbers.len() {
        if !valid(&numbers, i-N, i, numbers[i]) {
            return numbers[i]
        }
    }
    panic!("Could not find answer")
}

fn part_2(input: String) -> i64 {
    let target: i64 = part_1(input.clone());
    let numbers = convert_str(input);

    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            let xs = Vec::from(&numbers[i..j+1]);
            let total: i64 = Sum::sum(xs.iter());
            
            if total == target {
                return *xs.iter().min().unwrap() + *xs.iter().max().unwrap()
            }
            if total > target {
                break
            }
        }
    }
    panic!("Could not find answer")
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
