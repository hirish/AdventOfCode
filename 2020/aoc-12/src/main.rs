use aoc::{read_stdin, Point, State1, State2, Direction};

fn part_1(input: String) -> usize {
    let mut s = State1 {direction: Direction::East, pos: Point {x: 0, y: 0} };
    for line in input.lines() {
        s.apply(line);
    }
    s.pos.distance()
}

fn part_2(input: String) -> usize {
    let mut s = State2 {
        waypoint: Point {x: 10, y: 1},
        pos: Point {x: 0, y: 0},
    };

    for line in input.lines() {
        s.apply(line);
    }

    s.pos.distance()
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
