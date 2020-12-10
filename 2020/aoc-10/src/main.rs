use aoc::read_stdin;
use std::collections::HashMap;

fn convert_str(input: String) -> Vec<i64> {
    let mut out = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    out.sort();
    out
}

fn part_1(input: String) -> i64 {
    let mut prev = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 1;
    let numbers = convert_str(input);
    for i in numbers {
        let diff = i - prev;
        if diff == 1 {
            diff_1 += 1;
        } else if diff == 3 {
            diff_3 += 1
        }
        prev = i;
    }
    diff_1 * diff_3
}

fn count_arrangements(numbers: &Vec<i64>, target: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    let mut count = 0;

    if let Some(v) = cache.get(&target) {
        return *v
    }

    if target <= 3 {
        count += 1
    }

    if numbers.contains(&(target-1)) {
        count += count_arrangements(&numbers, target-1, cache)
    }
    if numbers.contains(&(target-2)) {
        count += count_arrangements(&numbers, target-2, cache)
    }
    if numbers.contains(&(target-3)) {
        count += count_arrangements(&numbers, target-3, cache)
    }

    cache.insert(target, count);
    count
}

fn part_2(input: String) -> i64 {
    let numbers = convert_str(input);
    let target = numbers.iter().max().unwrap() + 3;
    let mut cache: HashMap<i64, i64> = HashMap::new();
    count_arrangements(&numbers, target, &mut cache)
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
