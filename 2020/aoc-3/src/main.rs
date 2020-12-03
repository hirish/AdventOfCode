use aoc::{read_stdin, Map};

pub fn no_trees(map: &Map, angle: (usize, usize)) -> usize {
    let mut pos = (0, 0);
    let mut trees = 0;
    while pos.0 < map.height() {
        if map.get(pos.0, pos.1) == '#' {
            trees += 1
        }
        pos.0 += angle.0;
        pos.1 += angle.1
    }
    trees
}

fn part_1(input: String) -> usize {
    let map: Map = input.parse().unwrap();
    let angle = (1, 3);
    no_trees(&map, angle)
}

fn part_2(input: String) -> usize {
    let map: Map = input.parse().unwrap();
    let mut answer = 1;
    let angles = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    for angle in angles {
        answer *= no_trees(&map, angle)
    }

    answer
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
