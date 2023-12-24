use std::time::Instant;

use aoc2024::read_stdin;

#[derive(Debug)]
enum Op {
    Minus,
    Equals(usize),
}

#[derive(Debug)]
struct Step(String, Op);

type Input = Vec<Step>;

fn parse(input: String) -> Input {
    input
        .split(",")
        .map(|s| {
            if let Some(_) = s.find('=') {
                let (pattern, n) = s.split_once('=').unwrap();
                Step(pattern.into(), Op::Equals(n.parse().unwrap()))
            } else {
                Step(s.strip_suffix('-').unwrap().into(), Op::Minus)
            }
        })
        .collect()
}

fn hash(s: &str) -> usize {
    let mut v = 0;
    for c in s.chars() {
        v += c as usize;
        v *= 17;
        v %= 256;
    }
    v
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let input = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let mut boxes: Vec<Vec<(String, usize)>> = (0..256).map(|_| Vec::new()).collect();

    for Step(label, op) in input {
        let h = hash(&label);
        match op {
            Op::Minus => {
                if let Some(i) = boxes[h]
                    .clone()
                    .iter()
                    .enumerate()
                    .find(|(_, (l, _))| l == &label)
                {
                    boxes[h].remove(i.0);
                }
            }
            Op::Equals(f) => {
                if let Some(i) = boxes[h]
                    .clone()
                    .iter()
                    .enumerate()
                    .find(|(_, (l, _))| l == &label)
                {
                    *boxes[h].get_mut(i.0).unwrap() = (label, f)
                } else {
                    boxes[h].push((label, f))
                }
            }
        }
    }

    let v: usize = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(|(j, (_, f))| (i + 1) * (j + 1) * f)
                .sum::<usize>()
        })
        .sum();

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
