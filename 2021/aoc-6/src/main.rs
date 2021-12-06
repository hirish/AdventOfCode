use std::collections::HashMap;

use aoc_1::read_stdin_numbers;


fn count_fish(input: Vec<usize>, generations: usize) -> usize {
    let mut fish = vec![0; 9];

    for f in input {
        fish[f] += 1;
    }

    for _ in 0..generations {
        let tmp = fish[0];
        fish[0] = fish[1];
        fish[1] = fish[2];
        fish[2] = fish[3];
        fish[3] = fish[4];
        fish[4] = fish[5];
        fish[5] = fish[6];
        fish[6] = fish[7] + tmp;
        fish[7] = fish[8];
        fish[8] = tmp;
    }

    fish.iter().sum()
}


fn part_1(input: Vec<usize>) -> usize {
    count_fish(input, 80)
}

fn part_2(input: Vec<usize>) -> usize {
    count_fish(input, 256)
}

fn main() {
    let input = read_stdin_numbers(",");

    println!("Answer 1: {}", part_1(input.clone()));
    println!("Answer 2: {}", part_2(input));
}
