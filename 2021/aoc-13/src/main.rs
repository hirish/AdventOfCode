use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use aoc_1::read_stdin;

#[derive(Clone)]
struct Paper {
    points: HashSet<(usize, usize)>,
    h: usize,
    w: usize,
}

impl fmt::Debug for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.points.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Paper {
    fn fold_x(&self, v: usize) -> Self {
        let mut points = HashSet::new();

        self.points.iter().for_each(|&(x, y)| {
            let x = if x > v { 2 * v - x } else { x };
            points.insert((x, y));
        });

        Paper {
            points,
            h: self.h,
            w: v,
        }
    }

    fn fold_y(&self, v: usize) -> Self {
        let mut points = HashSet::new();

        self.points.iter().for_each(|&(x, y)| {
            let y = if y > v { 2 * v - y } else { y };
            points.insert((x, y));
        });

        Paper {
            points,
            h: v,
            w: self.w,
        }
    }
}

impl FromStr for Paper {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let (mut h, mut w) = (0, 0);

        for point in input.lines() {
            let (x, y) = point.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            set.insert((x, y));
            if x >= w {
                w = x
            }
            if y >= h {
                h = y
            }
        }

        Ok(Paper { points: set, h, w })
    }
}

fn part_1(paper: Paper, folds: Vec<(&str, usize)>) -> usize {
    let &(axis, v) = folds.first().unwrap();

    match axis {
        "x" => paper.fold_x(v).points.len(),
        "y" => paper.fold_y(v).points.len(),
        _ => panic!(),
    }
}

fn part_2(mut paper: Paper, folds: Vec<(&str, usize)>) -> usize {
    for (axis, v) in folds {
        match axis {
            "x" => paper = paper.fold_x(v),
            "y" => paper = paper.fold_y(v),
            _ => panic!(),
        }
    }

    println!("{:?}", paper);
    1
}

fn main() {
    let input = read_stdin();

    let (points, folds) = input.split_once("\n\n").unwrap();
    let paper: Paper = points.parse().unwrap();

    let folds: Vec<(&str, usize)> = folds
        .lines()
        .filter_map(|l| {
            let (axis, v) = l.strip_prefix("fold along ")?.split_once('=')?;
            Some((axis, v.parse().unwrap()))
        })
        .collect();

    println!("Answer 1: {}", part_1(paper.clone(), folds.clone()));
    println!("Answer 2: {}", part_2(paper, folds));
}
