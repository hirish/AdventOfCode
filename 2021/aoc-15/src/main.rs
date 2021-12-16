use std::time::Instant;
use std::collections::{HashMap, BinaryHeap};
use std::str::FromStr;

use aoc_1::read_stdin;

#[derive(Clone, Debug, Hash, Eq, Copy, Ord)]
struct Pos {
    x: usize,
    y: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Pos {
    fn neighbours(&self) -> Vec<Pos> {
        vec![
            Pos{x: self.x-1, y: self.y},
            Pos{x: self.x+1, y: self.y},
            Pos{x: self.x, y: self.y-1},
            Pos{x: self.x, y: self.y+1},
        ]
    }

    fn dist(&self, other: &Self) -> usize {
        let x1 = self.x as isize;
        let x2 = other.x as isize;
        let y1 = self.y as isize;
        let y2 = other.y as isize;

        (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt() as usize
    }
}


#[derive(Clone, Debug)]
struct Input {
    map: HashMap<Pos, usize>,
    start: Pos,
    end: Pos,
    tile_size: usize,
    part_2: bool,
}

impl Input {
    fn d(&self, p: &Pos) -> Option<usize> {
        if !self.part_2 {
            Some(*self.map.get(p)?)
        } else {
            let Pos {x, y} = p;
            let vx = x / self.tile_size;
            let rx = x % self.tile_size;

            let vy = y / self.tile_size;
            let ry = y % self.tile_size;

            let tile_pos = Pos{x: rx, y: ry};

            if vx > 4 || vy > 4 {
                return None
            }

            let v = self.map.get(&tile_pos)? + vx + vy;
            let t = ((v - 1) % 9) + 1;
            Some(t)
        }
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let h = input.lines().count();

        let mut map = HashMap::new();
        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, v)| {
                let v = v.to_digit(10).unwrap() as usize;
                let p = Pos{x, y};
                map.insert(p, v);
            })
        });


        Ok(Input {
            map,
            start: Pos{x: 0, y: 0},
            end: Pos{x: h - 1, y: h - 1},
            tile_size: h,
            part_2: false,
        })
    }
}

#[derive(Ord, Debug, Eq)]
struct FPos {
    pos: Pos,
    f: usize,
}

impl PartialEq for FPos {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.f == other.f
    }
}

impl PartialOrd for FPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.f.cmp(&self.f))
    }
}

fn compute(input: Input) -> Option<usize> {
    let mut prev: HashMap<Pos, Pos> = HashMap::new();
    let mut open: BinaryHeap<FPos> = BinaryHeap::new();
    let mut g: HashMap<Pos, usize> = HashMap::new();
    let mut f: HashMap<Pos, usize> = HashMap::new();
    let mut h: HashMap<Pos, usize> = HashMap::new();
    
    open.push(FPos{pos: input.start, f: 0});
    g.insert(input.start, 0);
    f.insert(input.start, input.start.dist(&input.end));
    h.insert(input.start, input.start.dist(&input.end));

    while !open.is_empty(){
        let FPos{pos: p, f: _} = open.pop().unwrap();

        if p == input.end {
            return Some(g[&input.end])
        }

        for neighbour in p.neighbours() {
            let d = input.d(&neighbour);

            if d.is_none() {
                continue
            }

            let new_g = g[&p] + d?;
            if !g.contains_key(&neighbour) || new_g < g[&neighbour] {
                prev.insert(neighbour, p);
                g.insert(neighbour, new_g);
                let hv = *h.entry(neighbour).or_insert_with(|| neighbour.dist(&input.end));
                f.insert(neighbour, new_g + hv);
                open.push(FPos{pos: neighbour, f: new_g + hv});
            }

        }
    }

    None
}

fn part_1(input: Input) -> Option<usize> {
    compute(input)
}

fn part_2(mut input: Input) -> Option<usize> {
    input.part_2 = true;
    let h = input.tile_size * 5 - 1;
    input.end = Pos {x: h, y: h};
    compute(input)
}

fn main() -> Result<(), ()> {
    let now = Instant::now();
    let input: Input = read_stdin().parse()?;
    let elapsed_time = now.elapsed();
    println!("Running parsing took {} ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(input.clone()).ok_or(())?);
    let elapsed_time = now.elapsed();
    println!("Running part_1 took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(input).ok_or(())?);
    println!("Running part_2 took {}ms.", now.elapsed().as_millis());

    Ok(())
}
