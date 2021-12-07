use aoc_1::read_stdin_numbers;

fn part_1(input: Vec<isize>) -> usize {
    let m = *input.iter().max().unwrap();
    
    let mut min_cost = None;
    for pos in 0..m {
        let cost: usize = input
            .iter()
            .map(|x| (x - pos).abs() as usize)
            .sum();

        if min_cost == None || cost < min_cost.unwrap() {
            min_cost = Some(cost)
        }
    }

    min_cost.unwrap()
}

fn part_2(input: Vec<isize>) -> usize {
    let m = *input.iter().max().unwrap();
    let mut costs: Vec<usize> = Vec::new();

    for i in 0..(m+1) as usize {
        costs.push(if i == 0 {0} else {i + costs[i-1]})
    }
    
    let mut min_cost = None;
    for pos in 0..m {
        let cost: usize = input
            .iter()
            .map(|x| costs[((x - pos).abs()) as usize])
            .sum();

        if min_cost == None || cost < min_cost.unwrap() {
            min_cost = Some(cost)
        }
    }

    min_cost.unwrap()
}

fn main() {
    let input: Vec<isize> = read_stdin_numbers(",")
        .into_iter()
        .map(|i| i as isize)
        .collect();

    println!("Answer 1: {}", part_1(input.clone()));
    println!("Answer 2: {}", part_2(input));
}
