use aoc_1::read_stdin_numbers;

fn part_1(numbers: Vec<u32>) -> Option<u32> {
    let mut count = 0;
    let mut prev = 0;
    for x in &numbers {
        if *x > prev {
            count += 1
        }
        prev = *x
    }
    Some(count - 1)
}

fn part_2(numbers: Vec<u32>) -> Option<u32> {
    let mut count = 0;
    let mut prev = 0;
    for x in 2..numbers.len() {
        let window = numbers[x] + numbers[x-1] + numbers[x-2];
        if window > prev {
            count += 1;
        }
        prev = window
    }
    Some(count - 1)
}

fn main() {
    let numbers = read_stdin_numbers();
    println!("Answer 1: {}", part_1(numbers.clone()).expect("No answer found"));
    println!("Answer 2: {}", part_2(numbers).expect("No answer found"));
}
