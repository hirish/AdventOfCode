use aoc2024::read_stdin;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, VecDeque},
    time::Instant,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum T {
    FlipFlop(bool),
    Conjunction(usize, BTreeSet<String>),
    Broadcaster,
}

type Input = (BTreeMap<String, Vec<String>>, BTreeMap<String, T>);

fn parse(input: String) -> Input {
    let mut dependencies: BTreeMap<String, Vec<String>> = BTreeMap::default();
    let mut subscribers: BTreeMap<String, Vec<String>> = BTreeMap::default();

    let state: BTreeMap<String, T> = input
        .lines()
        .map(|mut l| {
            let t = if let Some(m) = l.strip_prefix('%') {
                l = m;
                T::FlipFlop(false)
            } else if let Some(m) = l.strip_prefix('&') {
                l = m;
                T::Conjunction(0, BTreeSet::default())
            } else {
                T::Broadcaster
            };

            let (label, outputs) = l.split_once(" -> ").unwrap();
            subscribers.insert(
                label.into(),
                outputs
                    .split(", ")
                    .map(|s| {
                        dependencies
                            .entry(s.into())
                            .or_insert_with(|| vec![])
                            .push(label.into());
                        s.into()
                    })
                    .collect(),
            );

            (label.to_string(), t)
        })
        .collect();

    (
        subscribers,
        state
            .into_iter()
            .map(|(l, t)| {
                (
                    l.clone(),
                    (match t {
                        T::Conjunction(_, mem) => T::Conjunction(dependencies[&l].len(), mem),
                        _ => t,
                    }),
                )
            })
            .collect(),
    )
}

struct Pulse {
    from: String,
    to: String,
    val: bool,
}

fn process(
    map: &BTreeMap<String, Vec<String>>,
    mut state: BTreeMap<String, T>,
    mut pulses: VecDeque<Pulse>,
) -> (bool, BTreeMap<String, T>) {
    while !pulses.is_empty() {
        let Pulse { from, to, val } = pulses.pop_front().unwrap();
        let mut new_state = state.clone();

        let s = map.get(&to);

        if s.is_none() {
            continue;
        }

        let subscribers = s.unwrap();
        let t = state.get(&to).unwrap();

        let new_pulse = match t {
            T::FlipFlop(curr) => {
                if val {
                    None
                } else if *curr {
                    new_state.insert(to.clone(), T::FlipFlop(false));
                    Some(false)
                } else {
                    new_state.insert(to.clone(), T::FlipFlop(true));
                    Some(true)
                }
            }
            T::Conjunction(l, memory) => {
                let mut new_memory = memory.clone();
                if val {
                    new_memory.insert(from.clone());
                } else {
                    new_memory.remove(&from);
                }
                new_state.insert(to.clone(), T::Conjunction(*l, new_memory.clone()));

                if new_memory.len() == *l {
                    Some(false)
                } else {
                    Some(true)
                }
            }
            T::Broadcaster => Some(val),
        };

        if let Some(new_pulse) = new_pulse {
            for subscriber in subscribers {
                pulses.push_back(Pulse {
                    from: to.clone(),
                    to: subscriber.clone(),
                    val: new_pulse,
                });
            }
        }

        state = new_state;
    }

    (false, state)
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
    let (map, _) = parse(input.clone());
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let mut vs: Vec<usize> = vec![];
    for route in map.get("broadcaster").unwrap() {
        let (mut map, mut state) = parse(input.clone());
        map.insert("broadcaster".into(), vec![route.to_string()]);

        let mut i = 0;
        let mut first_seen: HashMap<_, usize> = HashMap::default();
        loop {
            i += 1;
            if let Some(j) = first_seen.get(&state) {
                vs.push(i - j);
                break;
            }
            first_seen.insert(state.clone(), i);

            let pulses: VecDeque<_> = vec![Pulse {
                from: "button".into(),
                to: "broadcaster".into(),
                val: false,
            }]
            .into_iter()
            .collect();

            let (rx, new_state) = process(&map, state, pulses);

            if rx {
                break;
            }

            state = new_state;
        }
    }

    let v = vs.into_iter().reduce(lcm).unwrap();

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
