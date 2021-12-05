use aoc_1::read_stdin;


fn parse(input: String) -> Vec<Vec<usize>> {
    input
        .split('\n')
        .map(|x| {
            x
                .trim()
                .chars()
                .map(|y| if y == '1' {1} else {0})
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn bin_to_usize(input: &Vec<usize>) -> usize {
    let mut t = 0;
    let base: usize = 2;
    for i in 0..input.len() {
        if input[i] == 1 {
            t += base.pow((input.len() - 1 - i).try_into().unwrap())
        }
    }
    t
}

fn find_most_common(values: &Vec<Vec<usize>>, reverse: bool) -> Vec<usize> {
    let mut x: Vec<usize> = vec![0; values[0].len()];

    for value in values {
        for (i, c) in value.iter().enumerate() {
            x[i] += c
        }
    }

    x.iter()
        .map(|x| if &(x*2) >= &values.len() {true} else {false})
        .map(|x| if reverse {!x} else {x})
        .map(|x| if x {1} else {0})
        .collect()
}

fn part_1(readings: Vec<Vec<usize>>) -> Option<usize> {
    let y = bin_to_usize(&find_most_common(&readings, false));
    let z = bin_to_usize(&find_most_common(&readings, true));
    Some(y * z)
}

fn find_reading(readings: Vec<Vec<usize>>, reverse: bool) -> usize {
    let mut search_space = readings;
    let l = search_space[0].len();

    for i in 0..l {
        let y = find_most_common(&search_space, reverse);

        search_space = search_space.into_iter()
            .filter(|x| x[i] == y[i])
            .collect();

        if search_space.len() <= 1 {
            break;
        }
    }

    bin_to_usize(&search_space[0])
}


fn part_2(readings: Vec<Vec<usize>>) -> Option<usize> {
    let oxygen = find_reading(readings.clone(), false);
    let co2 = find_reading(readings.clone(), true);
    Some(oxygen * co2)
}

fn main() {
    let input = read_stdin();
    let instructions = parse(input);
    println!("Answer 1: {}", part_1(instructions.clone()).expect("No answer found"));
    println!("Answer 2: {}", part_2(instructions).expect("No answer found"));
}
