use std::time::Instant;
use std::str::FromStr;

use aoc_1::read_stdin;

#[derive(Clone, Debug)]
struct Input {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = &input[13..].split_once(", ").unwrap();
        let x_str = &x_str[2..];
        let y_str = &y_str[2..];

        let (min_x, max_x) = x_str.split_once("..").unwrap();
        let min_x = min_x.parse().unwrap();
        let max_x = max_x.parse().unwrap();

        let (min_y, max_y) = y_str.split_once("..").unwrap();
        let min_y = min_y.parse().unwrap();
        let max_y = max_y.parse().unwrap();

        Ok(Self{min_x, max_x, min_y, max_y})
    }
}

fn part_1(input: Input) -> Option<isize> {
    let x  = -input.min_y;
    Some((5 * x * (x-1))/10)
}

fn part_2(input: Input) -> Option<usize> {
    let Input {min_x, max_x, min_y, max_y} = input;
    let mut c = 0;

    for x in 1..1+max_x {
        for y in min_y..1-min_y {
            let mut sx = 0;
            let mut sy = 0;
            let mut vx = x;
            let mut vy = y;
            while sx <= max_x && sy >= min_y {
                if sx >= min_x && sx <= max_x && sy >= min_y && sy <= max_y {
                    c +=1;
                    break
                }

                sx += vx;
                if vx > 0 {vx -= 1}
                sy += vy;
                vy -= 1;
            }
        }
    }

    Some(c)
}

fn main() -> Result<(), ()> {
    let now = Instant::now();
    let input: Input = read_stdin().parse()?;
    println!("Running parsing took {}ms.", now.elapsed().as_millis());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(input.clone()).ok_or(())?);
    println!("Running part_1 took {}ms.", now.elapsed().as_millis());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(input).ok_or(())?);
    println!("Running part_2 took {}ms.", now.elapsed().as_millis());

    Ok(())
}
