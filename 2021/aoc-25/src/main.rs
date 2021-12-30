use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

use aoc_1::{duration, read_stdin};

#[derive(Clone, Debug)]
enum Slug {D, R}

#[derive(Clone)]
struct State {
    slugs: HashMap<(usize, usize), Slug>,
    h: usize,
    w: usize,
}

impl FromStr for State {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut slugs = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '>' => {slugs.insert((x, y), Slug::R);},
                    'v' => {slugs.insert((x, y), Slug::D);},
                    _ => {},
                }
            }
        }
        Ok(Self {
            slugs,
            h: input.lines().count(),
            w: input.lines().nth(0).unwrap().len(),
        })
    }
}

impl std::fmt::Debug for State {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.slugs.contains_key(&(x, y)) {
                    match self.slugs[&(x, y)] {
                        Slug::D => write!(f, "v")?,
                        Slug::R => write!(f, ">")?,
                    }
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f, "")?
        }

        Ok(())
    }

}


impl State {
    fn step(&self) -> (Self, bool) {
        let slugs = &self.slugs;
        let mut new_slugs = HashMap::new();
        let mut changed = false;

        for (&(x, y), slug) in slugs.iter() {
            match slug {
                Slug::R => {
                    let nx = (x + 1) % self.w;
                    if !slugs.contains_key(&(nx, y)) {
                        new_slugs.insert((nx, y), Slug::R);
                        changed = true;
                    } else {
                        new_slugs.insert((x, y), Slug::R);
                    }
                },
                Slug::D => {
                    new_slugs.insert((x, y), Slug::D);
                }
            }
        }

        let slugs = new_slugs;
        let mut new_slugs = HashMap::new();

        for (&(x, y), slug) in slugs.iter() {
            match slug {
                Slug::R => {
                    new_slugs.insert((x, y), Slug::R);
                },
                Slug::D => {
                    let ny = (y + 1) % self.h;
                    if !slugs.contains_key(&(x, ny)) {
                        new_slugs.insert((x, ny), Slug::D);
                        changed = true;
                    } else {
                        new_slugs.insert((x, y), Slug::D);
                    }
                }
            }
        }

        (Self {slugs: new_slugs, h: self.h, w: self.w}, changed)
    }
}

#[derive(Clone, Debug)]
struct Input {
    state: State,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {state: input.parse()?})
    }
}

fn part_1(input: Input) -> Option<usize> {
    let mut state = input.state;
    let mut c = 0;


    loop {
        // println!("{:?}", state);
        c += 1;
        let r = state.step();
        state = r.0;
        if !r.1 {
            break
        }
    }

    // println!("{:?}", state);

    Some(c)
}

fn main() -> Result<(), ()> {
    let now = Instant::now();
    let input: Input = read_stdin().parse()?;
    println!("Running parsing took {}.", duration(now));

    let now = Instant::now();
    println!("Answer 1: {}", part_1(input.clone()).ok_or(())?);
    println!("Running part_1 took {}.", duration(now));

    Ok(())
}
