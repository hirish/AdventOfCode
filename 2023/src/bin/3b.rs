use std::collections::HashMap;

use aoc2024::read_stdin;

type Input = HashMap<(isize, isize), char>;

fn parse(input: String) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect()
}

fn neighbours((x, y): (isize, isize), len: isize) -> Vec<(isize, isize)> {
    let mut ns = vec![];
    for i in -1..len + 1 {
        ns.push((x + i, y - 1));
        ns.push((x + i, y));
        ns.push((x + i, y + 1));
    }
    ns
}

fn main() {
    let input = read_stdin();
    let map = parse(input.clone());
    let mut gears: HashMap<(isize, isize), u32> = HashMap::new();

    let res: u32 = map
        .clone()
        .iter()
        .filter(|(_, c)| char::is_numeric(**c))
        .filter(|((x, y), _)| {
            map.get(&(x - 1, *y))
                .filter(|d| char::is_numeric(**d))
                .is_none()
        })
        .filter_map(|((x, y), _)| {
            let mut v = 0;
            for i in 0.. {
                let d = map.get(&(x + i, *y));
                if d.is_some() && char::is_numeric(*d.unwrap()) {
                    v = v * 10 + d.unwrap().to_digit(10).unwrap();
                } else {
                    break;
                }
            }

            for neighbour in neighbours((*x, *y), format!("{}", v).len() as isize) {
                if Some(&'*') != map.get(&neighbour) {
                    continue;
                }

                if let Some(w) = gears.get(&neighbour) {
                    return Some(w * v);
                } else {
                    gears.insert(neighbour, v);
                }
            }

            return None;
        })
        .sum();

    println!("{}", res)
}
