use aoc_10::read_stdin;

type Input = Vec<isize>;

fn parse(input: String) -> Input {
    let instructions: Vec<&str> = input.trim().lines().collect();

    let mut x = 1;
    let mut ins = 0;
    let mut inc: Option<isize> = None;
    let mut positions = vec![x];

    for _ in 1..240 {
        positions.push(x);

        if let Some(v) = inc {
            x += v;
            inc = None
        } else {
            if let Some(val) = instructions[ins].strip_prefix("addx ") {
                inc = Some(val.parse().unwrap())
            };
            ins += 1
        }
    }

    positions
}

fn part_1(input: Input) -> isize {
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&i| input[i as usize] * i)
        .sum()
}

fn part_2(input: Input) {
    for cycle in 1..240 {
        if (input[cycle] - ((cycle - 1) % 40) as isize).abs() <= 1 {
            print!("█")
        } else {
            print!(" ")
        }

        if (cycle % 40) == 0 {
            println!("")
        }
    }
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    part_2(lines);
}
