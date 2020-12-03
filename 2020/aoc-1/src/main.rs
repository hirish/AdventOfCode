use aoc_1::read_stdin_numbers;

fn part_1(numbers: Vec<u32>) -> Option<u32> {
    for x in &numbers {
        for y in &numbers {
            if x != y {
                if (x + y) == 2020 {
                    return Some(x * y)
                }
            }
        }
    }
    None
}

fn part_2(numbers: Vec<u32>) -> Option<u32> {
    for x in &numbers {
        for y in &numbers {
            for z in &numbers {
                if (x + y + z) == 2020 {
                    return Some(x * y * z)
                }
            }
        }
    }
    None
}

fn main() {
    let numbers = read_stdin_numbers();
    let answer = part_2(numbers);

    println!("Answer: {}", answer.expect("No answer found"));
}
