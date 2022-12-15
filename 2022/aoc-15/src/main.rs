use aoc_15::read_stdin;

use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Clone)]
struct Input {
    map: HashMap<(isize, isize), isize>,
    beacons: HashSet<(isize, isize)>,
}

fn parse(input: String) -> Input {
    let mut beacons = HashSet::new();
    let map = input
        .lines()
        .filter_map(|l| {
            if let Some(l) = l.strip_prefix("Sensor at x=") {
                let (sensor, beacon) = l.split_once(": closest beacon is at x=").unwrap();
                let (x, y) = sensor.split_once(", y=").unwrap();
                let sensor: (isize, isize) = (x.parse().unwrap(), y.parse().unwrap());
                let (x, y) = beacon.split_once(", y=").unwrap();
                let beacon: (isize, isize) = (x.parse().unwrap(), y.parse().unwrap());
                beacons.insert(beacon);
                let d = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
                Some((sensor, d))
            } else {
                None
            }
        })
        .collect();

    Input { map, beacons }
}

fn part_1(input: Input) -> isize {
    const Y: isize = 2000000;

    let ranges: Vec<(isize, isize)> = input
        .map
        .into_iter()
        .filter_map(|((x, y), d)| {
            let dy = (y - Y).abs();
            let dx = d - dy;
            if dx < 0 {
                None
            } else {
                Some((x - dx, x + dx + 1))
            }
        })
        .collect();

    let min = ranges.iter().map(|x| x.0).min().unwrap();
    let max = ranges.iter().map(|x| x.1).max().unwrap();

    let beacon_count = input.beacons.iter().filter(|b| b.1 == Y).count() as isize;
    max - min - beacon_count
}

fn part_2(input: Input) -> isize {
    let mut ps = HashSet::new();
    let mut qs = HashSet::new();
    let mut rs = HashSet::new();
    let mut ss = HashSet::new();

    let mut a = None;
    let mut b = None;

    for ((x, y), d) in input.map.iter() {
        let p = d + 1 + x + y;
        let q = x + y - d - 1;
        let r = d + 1 + x - y;
        let s = x - y - d - 1;

        ps.insert(p);
        qs.insert(q);
        rs.insert(r);
        ss.insert(s);

        if ps.contains(&q) {
            a = Some(q)
        }
        if qs.contains(&p) {
            a = Some(p)
        }
        if rs.contains(&s) {
            b = Some(s)
        }
        if ss.contains(&r) {
            b = Some(r)
        }
    }

    let a = a.unwrap();
    let b = b.unwrap();

    let y = (a + b) / 2;
    let x = (a - b) / 2;

    y * 4000000 + x
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
    let elapsed_time = now.elapsed();
    println!("Running parsing took {}μs.", elapsed_time.as_micros());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(lines.clone()));
    let elapsed_time = now.elapsed();
    println!("Running part_1 took {}μs.", elapsed_time.as_micros());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(lines));
    println!("Running part_2 took {}μs.", now.elapsed().as_micros());
}
