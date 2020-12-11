use aoc::{read_stdin, Map};

fn part_1(input: String) -> usize {
    let mut map: Map = input.parse().unwrap();
    let mut made_change = true;
    let mut count = 0;

    while made_change {
        let next = map.next_grid(false);
        map = next.0;
        made_change = next.1;
        count += 1;
    }

    println!("Count {}", count);
    map.occupied()
}

fn part_2(input: String) -> usize {
    let mut map: Map = input.parse().unwrap();
    let mut made_change = true;
    let mut count = 0;

    while made_change {
        let next = map.next_grid(true);
        map = next.0;
        made_change = next.1;
        count += 1;
    }

    println!("Count {}", count);
    map.occupied()
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
