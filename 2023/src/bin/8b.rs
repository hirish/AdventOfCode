use aoc2024::read_stdin;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

enum Dir {
    L,
    R,
}

type Input = (Vec<Dir>, HashMap<String, (String, String)>);

fn parse(input: String) -> Input {
    let (dirs, network) = input.split_once("\n\n").unwrap();

    let dirs = dirs
        .chars()
        .map(|c| if c == 'L' { Dir::L } else { Dir::R })
        .collect();

    let network = network
        .lines()
        .map(|l| {
            let l = l.strip_suffix(')').unwrap();
            let (from, dirs) = l.split_once(" = (").unwrap();
            let (l, r) = dirs.split_once(", ").unwrap();
            (from.into(), (l.into(), r.into()))
        })
        .collect();

    (dirs, network)
}

fn solve_start(
    dirs: &Vec<Dir>,
    network: &HashMap<String, (String, String)>,
    start: String,
) -> Vec<usize> {
    let mut seen: HashSet<(usize, String)> = HashSet::new();
    let mut curr = start.clone();
    let mut output = Vec::new();

    for i in 0.. {
        let j = i % dirs.len();
        if seen.contains(&(j, curr.clone())) {
            break;
        } else {
            seen.insert((j, curr.clone()));
        }

        if curr.ends_with('Z') {
            output.push(i);
        }

        let (l, r) = network.get(&curr).unwrap();
        curr = match dirs[i % dirs.len()] {
            Dir::L => l,
            Dir::R => r,
        }
        .into()
    }

    output
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn lcm(x: usize, y: usize) -> usize {
    x * (y / gcd(x, y))
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let (dirs, network) = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let z: usize = network
        .keys()
        .filter_map(|pos| {
            if pos.ends_with('A') {
                Some(pos.into())
            } else {
                None
            }
        })
        .flat_map(|start| solve_start(&dirs, &network, start))
        .reduce(|x, y| lcm(x, y))
        .unwrap();

    println!("{:?}", z);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
