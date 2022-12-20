use aoc_16::{
    graph_fns::{dijkstra, Node},
    intset::IntSet,
    read_stdin,
};

use fxhash::FxHashMap;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::{sync::Mutex, time::Instant};

type Hash = u64;

lazy_static! {
    static ref CACHE: Mutex<FxHashMap<Hash, usize>> = Mutex::new(FxHashMap::default());
}

type Id = u8;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Valve {
    id: Id,
    rate: usize,
    tunnels: Vec<Id>,
}

type Input = Vec<Valve>;

fn parse(input: String) -> Input {
    let mut nodes: FxHashMap<String, Id> = FxHashMap::default();
    nodes.insert("AA".to_string(), 0);

    let mut vs: Input = input
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
    vs
}

fn get_shortest_paths(input: &Input) -> FxHashMap<(Id, Id), u8> {
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

fn search(
    input: &Input,
    shortest_paths: &FxHashMap<(Id, Id), u8>,
    mut visited: usize,
    time_left: usize,
    current: &Valve,
    vented: usize,
) -> usize {
    visited |= 1 << current.id;

    input
        .par_iter()
        .filter_map(|v| {
            if v.rate == 0 {
                return None;
            }
            if ((visited >> v.id) & 1) > 0 {
                return None;
            }
            let k = key(&v.id, &current.id);
            let distance_to = shortest_paths.get(&k).unwrap();
            if *distance_to >= time_left as u8 {
                return None;
            }

            Some(search(
                input,
                shortest_paths,
                visited,
                time_left - *distance_to as usize - 1,
                v,
                (current.rate * time_left) + vented,
            ))
        })
        .max()
        .unwrap_or(current.rate * time_left + vented)
}

fn part_1(input: Input) -> usize {
    let shortest_paths = get_shortest_paths(&input);
    let start = &input[0];
    search(&input, &shortest_paths, 0, 30, start, 0)
}

#[derive(Clone, Eq, PartialEq)]
struct State<'a> {
    remaining: IntSet,
    time_left: (u8, u8),
    current: (Id, Id),
    rates: &'a FxHashMap<Id, u8>,
    shortest_paths: &'a FxHashMap<(Id, Id), u8>,
}

impl std::hash::Hash for State<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.remaining.hash(state);
        self.time_left.hash(state);
        self.current.hash(state);
    }
}

impl State<'_> {
    fn new<'a>(
        remaining: IntSet,
        (ta, tb): (u8, u8),
        (ca, cb): (Id, Id),
        rates: &'a FxHashMap<Id, u8>,
        shortest_paths: &'a FxHashMap<(Id, Id), u8>,
    ) -> State<'a> {
        State::<'a> {
            remaining,
            time_left: if ta > tb { (ta, tb) } else { (tb, ta) },
            current: if ta > tb { (ca, cb) } else { (cb, ca) },
            rates,
            shortest_paths,
        }
    }

    fn neighbours(&self) -> Vec<(Self, usize)> {
        let mut neighbours = vec![];

        for id in self.remaining.items() {
            let mut remaining = self.remaining;
            remaining.remove(id);

            let distance_to = self.shortest_paths[&key(&id, &self.current.0)];
            if distance_to < self.time_left.0 {
                let new_time_left = self.time_left.0 - distance_to - 1;
                neighbours.push((
                    State::new(
                        remaining,
                        (new_time_left, self.time_left.1),
                        (id, self.current.1),
                        self.rates,
                        self.shortest_paths,
                    ),
                    (new_time_left as usize * self.rates[&id] as usize),
                ))
            }

            let distance_to = self.shortest_paths[&key(&id, &self.current.1)];
            if distance_to < self.time_left.1 {
                let new_time_left = self.time_left.1 - distance_to - 1;
                neighbours.push((
                    State::new(
                        remaining,
                        (new_time_left, self.time_left.0),
                        (id, self.current.0),
                        self.rates,
                        self.shortest_paths,
                    ),
                    (new_time_left as usize * self.rates[&id] as usize),
                ))
            }
        }

        neighbours
    }

    fn hash(&self) -> Hash {
        ((self.remaining.v as u64) * 100000000)
            + ((self.current.0 as u64) * 1000000)
            + ((self.current.1 as u64) * 10000)
            + ((self.time_left.0 as u64) * 100)
            + (self.time_left.1 as u64)
    }
}

fn search_2(state: State) -> usize {
    let hash = state.hash();
    {
        let x = CACHE.lock().unwrap();
        if x.contains_key(&hash) {
            return x[&hash];
        }
    }

    let v = state
        .neighbours()
        .into_par_iter()
        .map(|(new_state, vented)| vented + search_2(new_state))
        .max()
        .unwrap_or(0);

    {
        CACHE.lock().unwrap().insert(hash, v);
    }

    v
}

fn part_2(input: Input) -> usize {
    let shortest_paths = get_shortest_paths(&input);
    let rates: FxHashMap<Id, u8> = input
        .iter()
        .filter_map(|v| {
            if v.id == 0 || v.rate > 0 {
                Some((v.id, v.rate as u8))
            } else {
                None
            }
        })
        .collect();

    let mut remaining = IntSet::new(&rates.keys().copied().collect());
    remaining.remove(0);

    let s = State::new(remaining, (26, 26), (0, 0), &rates, &shortest_paths);
    search_2(s)
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
