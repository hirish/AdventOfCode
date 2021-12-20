use std::collections::HashSet;
use std::str::FromStr;
use std::time::Instant;

use aoc_1::{duration, read_stdin};

fn bin_to_usize(bits: &[usize]) -> usize {
    let mut v = 0;
    for &bit in bits.iter() {
        v = v << 1;
        if bit == 1 {
            v += 1
        }
    }
    v
}

#[derive(Clone, Debug)]
struct Input {
    algorithm: Vec<bool>,
    image: HashSet<(isize, isize)>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (a, b) = input.split_once("\n\n").unwrap();
        let algorithm = a.chars().map(|c| c == '#').collect();
        let mut image = HashSet::new();

        b.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    image.insert((x as isize, y as isize));
                }
            })
        });

        Ok(Self {
            algorithm,
            image,
            min_x: 0,
            min_y: 0,
            max_x: b.lines().next().unwrap().len() as isize,
            max_y: b.lines().count() as isize,
        })
    }
}

fn enhance(input: Input, i: usize) -> Option<Input> {
    let Input{image, algorithm, min_x, max_x, min_y, max_y} = input;

    let mut new_image = HashSet::new();
    let outside = if algorithm[0] {
        if i % 2 == 0 {algorithm[0]} else {algorithm[algorithm.len() - 1]}
    } else {
        false
    };

    for y in (min_y - 1)..(max_y + 2) {
        for x in (min_x - 1)..(max_x + 2) {
            let mut bits = vec![];
            for py in (y-1)..(y+2) {
                for px in (x-1)..(x+2) {
                    let oob = py > max_y || py < min_y || px > max_x || px < min_x;
                    if oob {
                        bits.push(if outside {1} else {0})
                    } else if image.contains(&(px, py)) {
                        bits.push(1)
                    } else {
                        bits.push(0)
                    }
                }
            }
            let index = bin_to_usize(&bits);
            if algorithm[index] {
                new_image.insert((x, y));
            }
        }
    }

    Some(Input{
        algorithm,
        image: new_image,
        min_x: min_x - 1,
        min_y: min_y - 1,
        max_x: max_x + 1,
        max_y: max_y + 1,
    })
}

fn display(input: &Input) {
    let Input{image, algorithm: _, min_x, max_x, min_y, max_y} = input;

    for y in (min_y - 1)..(max_y + 2) {
        for x in (min_x - 1)..(max_x + 2) {
            print!("{}", if image.contains(&(x, y)) {'#'} else {'.'});
        }
        println!("");
    }
}

fn part_1(mut input: Input) -> Option<usize> {
    for i in 0..2 {
        input = enhance(input, i+1)?;
    }

    Some(input.image.len())
}

fn part_2(mut input: Input) -> Option<usize> {
    for i in 0..50 {
        input = enhance(input, i+1)?;
    }

    Some(input.image.len())
}

fn main() -> Result<(), ()> {
    let now = Instant::now();
    let input: Input = read_stdin().parse()?;
    println!("Running parsing took {}.", duration(now));
    display(&input);

    let now = Instant::now();
    println!("Answer 1: {}", part_1(input.clone()).ok_or(())?);
    println!("Running part_1 took {}.", duration(now));

    let now = Instant::now();
    println!("Answer 2: {}", part_2(input).ok_or(())?);
    println!("Running part_2 took {}.", duration(now));

    Ok(())
}
