use aoc_19::read_stdin;

use rayon::prelude::*;
use std::{str::FromStr, time::Instant};

#[derive(Clone, Copy, PartialEq)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl std::fmt::Debug for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[or: {} cl: {} ob: {} ge: {}]",
            self.ore, self.clay, self.obsidian, self.geode
        )
    }
}

impl std::ops::Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl std::ops::Sub for Resources {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl std::ops::Mul<u8> for Resources {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

impl Resources {
    fn empty() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn new(ore: u8, clay: u8, obsidian: u8, geode: u8) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn gte(&self, other: &Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn can_build(&self, blueprint: &Self) -> bool {
        (blueprint.ore == 0 || self.ore > 0)
            && (blueprint.clay == 0 || self.clay > 0)
            && (blueprint.obsidian == 0 || self.obsidian > 0)
            && (blueprint.geode == 0 || self.geode > 0)
    }
}

impl FromStr for Resources {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_suffix('.').unwrap_or(s);
        s = s.split_once(" costs ").unwrap().1;
        let mut v = Self::empty();
        for r in s.split(" and ") {
            let (count, resource) = r.split_once(' ').unwrap();
            let count: u8 = count.parse().unwrap();
            match resource {
                "ore" => v.ore = count,
                "clay" => v.clay = count,
                "obsidian" => v.obsidian = count,
                "geode" => v.geode = count,
                _ => panic!("Unknown resource {}", resource),
            }
        }
        Ok(v)
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
struct Costs {
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

impl FromStr for Costs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<Resources> = s.split(". Each").map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            ore: s[0],
            clay: s[1],
            obsidian: s[2],
            geode: s[3],
        })
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
struct State {
    total_t: u8,
    t: u8,
    max_usage: Resources,
    blueprint: Costs,
    resources: Resources,
    machines: Resources,
}

fn int_time_to_build(cost: isize, res: isize, generation: isize) -> isize {
    if (cost - res) <= 0 {
        return 0;
    }
    ((cost - res) + generation - 1) / generation
}

impl State {
    fn new(blueprint: Costs) -> Self {
        let max_ore = blueprint.ore.ore.max(
            blueprint
                .clay
                .ore
                .max(blueprint.obsidian.ore.max(blueprint.geode.ore)),
        );
        let max_clay = blueprint.ore.clay.max(
            blueprint
                .clay
                .clay
                .max(blueprint.obsidian.clay.max(blueprint.geode.clay)),
        );
        let max_obsidian = blueprint.ore.obsidian.max(
            blueprint
                .clay
                .obsidian
                .max(blueprint.obsidian.obsidian.max(blueprint.geode.obsidian)),
        );
        let max_geode = blueprint.ore.geode.max(
            blueprint
                .clay
                .geode
                .max(blueprint.obsidian.geode.max(blueprint.geode.geode)),
        );

        Self {
            blueprint,
            max_usage: Resources::new(max_ore, max_clay, max_obsidian, max_geode),
            resources: Resources::empty(),
            machines: Resources::new(1, 0, 0, 0),
            t: 24,
            total_t: 24,
        }
    }

    fn time_to_build(&self, machine: &Resources) -> u8 {
        if self.resources.gte(machine) {
            return 1;
        }

        1 + (int_time_to_build(
            machine.ore as isize,
            self.resources.ore as isize,
            self.machines.ore as isize,
        ))
        .max(int_time_to_build(
            machine.clay as isize,
            self.resources.clay as isize,
            self.machines.clay as isize,
        ))
        .max(int_time_to_build(
            machine.obsidian as isize,
            self.resources.obsidian as isize,
            self.machines.obsidian as isize,
        ))
        .max(int_time_to_build(
            machine.geode as isize,
            self.resources.geode as isize,
            self.machines.geode as isize,
        )) as u8
    }

    fn new_from(&self, consumed_resources: Resources, new_machines: Resources, dt: u8) -> Self {
        Self {
            blueprint: self.blueprint,
            max_usage: self.max_usage,
            resources: self.resources + (self.machines * dt) - consumed_resources,
            machines: self.machines + new_machines,
            t: self.t - dt,
            total_t: self.total_t,
        }
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut neighbours = vec![];

        if self.machines.can_build(&self.blueprint.geode) {
            let dt = self.time_to_build(&self.blueprint.geode);
            if dt <= self.t {
                neighbours.push(self.new_from(self.blueprint.geode, Resources::new(0, 0, 0, 1), dt))
            }
        }

        if self.machines.can_build(&self.blueprint.obsidian) {
            let dt = self.time_to_build(&self.blueprint.obsidian);
            if dt <= self.t && self.machines.obsidian < self.max_usage.obsidian {
                neighbours.push(self.new_from(
                    self.blueprint.obsidian,
                    Resources::new(0, 0, 1, 0),
                    dt,
                ))
            }
        }

        if self.machines.can_build(&self.blueprint.ore) {
            let dt = self.time_to_build(&self.blueprint.ore);
            if dt <= self.t && self.machines.ore < self.max_usage.ore {
                neighbours.push(self.new_from(self.blueprint.ore, Resources::new(1, 0, 0, 0), dt))
            }
        }
        if self.machines.can_build(&self.blueprint.clay) {
            let dt = self.time_to_build(&self.blueprint.clay);
            if dt <= self.t && self.machines.clay < self.max_usage.clay {
                neighbours.push(self.new_from(self.blueprint.clay, Resources::new(0, 1, 0, 0), dt))
            }
        }

        if neighbours.is_empty() {
            neighbours.push(Self {
                blueprint: self.blueprint,
                max_usage: self.max_usage,
                resources: self.resources + self.machines,
                machines: self.machines,
                t: self.t - 1,
                total_t: self.total_t,
            });
        }

        neighbours
    }
}

type Input = Vec<State>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| l.parse::<Costs>().unwrap())
        .map(State::new)
        .collect()
}

fn compute(s: State) -> u8 {
    if s.t == 0 {
        return s.resources.geode;
    }

    s.neighbours().into_par_iter().map(compute).max().unwrap()
}

fn part_1(input: Input) -> usize {
    input
        .into_par_iter()
        .enumerate()
        .map(|(i, v)| (i + 1) * compute(v) as usize)
        .sum()
}

fn part_2(input: Input) -> usize {
    input.into_par_iter().take(3).map(|v| compute(v) as usize).product()
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let mut lines = parse(input);
    let elapsed_time = now.elapsed();
    println!("Running parsing took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(lines.clone()));
    let elapsed_time = now.elapsed();
    println!("Running part_1 took {}ms.", elapsed_time.as_millis());

    lines.iter_mut().for_each(|s| {
        s.total_t = 32;
        s.t = 32;
    });

    let now = Instant::now();
    println!("Answer 2: {}", part_2(lines));
    println!("Running part_2 took {}ms.", now.elapsed().as_millis());
}
