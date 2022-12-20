use aoc_16::{
    graph_fns::{dijkstra, Node},
    intset::IntSet,
    read_stdin,
};

use fxhash::FxHashMap;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::{sync::Mutex, time::Instant};

lazy_static! {
    static ref CACHE: Mutex<FxHashMap<State, usize>> = Mutex::new(FxHashMap::default());
}

type Id = u8;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Valve {
    id: Id,
    rate: usize,
    tunnels: Vec<Id>,
}

type InputOld = Vec<Valve>;

#[derive(Clone)]
struct Input {
    rates: FxHashMap<Id, u8>,
    shortest_paths: FxHashMap<(Id, Id), u8>,
}

fn parse(input: String) -> Input {
    let mut nodes: FxHashMap<String, Id> = FxHashMap::default();
    nodes.insert("AA".to_string(), 0);

    input
        .lines()
        .filter(|l| !l.contains("rate=0"))
        .for_each(|line| {
            let line = line.strip_prefix("Valve ").unwrap();
            let line = line.split_once(' ').unwrap().0;
            nodes.insert(line.to_string(), nodes.len() as u8);
        });

    let mut vs: InputOld = input
        .lines()
        .map(|l| {
            let l = l.replace("tunnels", "tunnel");
            let l = l.replace("valves", "valve");
            let l = l.replace("leads", "lead");
            let l = l.strip_prefix("Valve ").unwrap();
            let (id, l) = l.split_once(' ').unwrap();
            let l = l.strip_prefix("has flow rate=").unwrap();
            let (rate, tunnels) = l.split_once("; tunnel lead to valve").unwrap();
            let tunnels = tunnels
                .split(", ")
                .map(|s| {
                    let len = nodes.len() as u8;
                    *nodes.entry(s.trim().to_string()).or_insert(len)
                })
                .collect();
            let len = nodes.len() as u8;
            let id = *nodes.entry(id.to_string()).or_insert(len);

            Valve {
                id,
                rate: rate.parse().unwrap(),
                tunnels,
            }
        })
        .collect();

    vs.sort_by_key(|v| v.id);

    let shortest_paths = get_shortest_paths(&vs);
    let rates: FxHashMap<Id, u8> = vs
        .iter()
        .filter_map(|v| {
            if v.id == 0 || v.rate > 0 {
                Some((v.id, v.rate as u8))
            } else {
                None
            }
        })
        .collect();

    Input {
        shortest_paths,
        rates,
    }
}

fn get_shortest_paths(input: &InputOld) -> FxHashMap<(Id, Id), u8> {
    let graph = input
        .iter()
        .map(|v| (v.id, Node::new(v.id, v.tunnels.clone())))
        .collect();

    let mut shortest_paths: FxHashMap<(Id, Id), u8> = FxHashMap::default();
    for a in input
        .iter()
        .filter(|v| v.id == 0 || v.rate > 0)
        .map(|v| &v.id)
    {
        for (b, d) in dijkstra(&graph, a).into_iter() {
            if input[b as usize].rate == 0 {
                continue;
            }
            if a == &b {
                continue;
            }
            let k = key(a, &b);
            shortest_paths.insert(k, d as u8);
        }
    }
    shortest_paths
}

fn key(a: &Id, b: &Id) -> (Id, Id) {
    (*a.min(b), *a.max(b))
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    remaining: IntSet,
    time_left: u8,
    current: Id,
}

impl State {
    fn new(remaining: IntSet, time_left: u8, current: Id) -> State {
        State {
            remaining,
            time_left,
            current,
        }
    }

    fn neighbours(
        &self,
        rates: &FxHashMap<Id, u8>,
        shortest_paths: &FxHashMap<(Id, Id), u8>,
    ) -> Vec<(Self, usize)> {
        let mut neighbours = vec![];

        for id in self.remaining.items() {
            let mut remaining = self.remaining;
            remaining.remove(id);

            let distance_to = shortest_paths[&key(&id, &self.current)];
            if distance_to < self.time_left {
                let new_time_left = self.time_left - distance_to - 1;
                neighbours.push((
                    State::new(remaining, new_time_left, id),
                    (new_time_left as usize * rates[&id] as usize),
                ))
            }
        }

        neighbours
    }
}

fn search(
    state: State,
    rates: &FxHashMap<Id, u8>,
    shortest_paths: &FxHashMap<(Id, Id), u8>,
) -> usize {
    {
        let x = CACHE.lock().unwrap();
        if x.contains_key(&state) {
            return x[&state];
        }
    }

    let v = state
        .neighbours(rates, shortest_paths)
        .into_par_iter()
        .map(|(new_state, vented)| vented + search(new_state, rates, shortest_paths))
        .max()
        .unwrap_or(0);

    {
        CACHE.lock().unwrap().insert(state, v);
    }

    v
}

fn part_1(input: Input) -> usize {
    let mut remaining = IntSet::new(&input.rates.keys().copied().collect());
    remaining.remove(0);

    let s = State::new(remaining, 30, 0);
    search(s, &input.rates, &input.shortest_paths)
}

fn part_2(input: Input) -> usize {
    let mut remaining = IntSet::new(&input.rates.keys().copied().collect());
    remaining.remove(0);

    (1..remaining.v)
        .filter(|v| v % 2 != 1)
        .par_bridge()
        .map(|v| (IntSet::new_with(v), IntSet::new_with(remaining.v - v)))
        .map(|(me, elephant)| {
            search(State::new(me, 26, 0), &input.rates, &input.shortest_paths)
                + search(
                    State::new(elephant, 26, 0),
                    &input.rates,
                    &input.shortest_paths,
                )
        })
        .max()
        .unwrap()
}

fn main() {
    let now = Instant::now();
    let input = read_stdin();
    let lines = parse(input);
    let elapsed_time = now.elapsed();
    println!("Running parsing took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(lines.clone()));
    let elapsed_time = now.elapsed();
    println!("Running part_1 took {}ms.", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(lines));
    println!("Running part_2 took {}ms.", now.elapsed().as_millis());
}
