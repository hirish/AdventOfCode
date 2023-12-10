use std::time::Instant;

use aoc2024::read_stdin;

type SeedRange = (isize, isize);
type SeedRanges = Vec<SeedRange>;
type Map = Vec<(isize, isize, isize)>;

fn map(m: &Map, seed: SeedRange) -> SeedRanges {
    let mut seeds = vec![seed];
    let mut new_seeds: SeedRanges = Vec::new();

    for (dest_from, source_from, len) in m {
        let mut old_seeds = vec![];
        for (from, to) in seeds {
            let source_to = source_from + len;
            let dest_to = dest_from + len;
            match (
                *source_from <= from && from <= source_to,
                *source_from <= to && to <= source_to,
            ) {
                (true, true) => {
                    new_seeds.push((dest_from + from - source_from, dest_from + to - source_from))
                }
                (false, true) => {
                    old_seeds.push((from, source_from - 1));
                    new_seeds.push((*dest_from, dest_from + to - source_from))
                }
                (true, false) => {
                    old_seeds.push((source_to + 1, to));
                    new_seeds.push((dest_from + from - source_from, dest_to))
                }
                (false, false) => {
                    if from > source_to || to < *source_from {
                        old_seeds.push((from, to))
                    } else {
                        new_seeds.push((*dest_from, dest_to));
                        old_seeds.push((from, source_from - 1));
                        old_seeds.push((source_to + 1, to));
                    }
                }
            }
        }
        seeds = old_seeds;
    }

    new_seeds.append(&mut seeds);

    new_seeds
}

type Input = (SeedRanges, Vec<Map>);

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .skip(1)
        .map(|l| {
            let mut l = l.split_whitespace().map(|n| n.parse::<isize>().unwrap());
            (l.next().unwrap(), l.next().unwrap(), l.next().unwrap())
        })
        .collect()
}

fn parse(input: String) -> Input {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let mut seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<isize>>()
        .chunks(2)
        .map(|v| (v[0], v[0] + v[1] - 1))
        .collect::<SeedRanges>();

    seeds.sort_by_key(|s| s.0);

    let maps: Vec<Map> = maps.split("\n\n").map(parse_map).collect();

    (seeds, maps)
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let (mut seeds, maps) = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    for m in maps.iter() {
        seeds = seeds.into_iter().flat_map(|s| map(m, s)).collect();
    }

    seeds.sort_by_key(|m| m.0);

    let v = seeds[0].0;

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
