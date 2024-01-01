use aoc2024::read_stdin;
use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    time::Instant,
};

#[derive(Debug, Clone, Hash)]
enum T {
    FlipFlop(bool),
    Conjunction(usize, BTreeSet<String>),
    Broadcaster,
}

type Input = (HashMap<String, Vec<String>>, HashMap<String, T>);

fn parse(input: String) -> Input {
    let mut dependencies: HashMap<String, Vec<String>> = HashMap::default();
    let mut subscribers: HashMap<String, Vec<String>> = HashMap::default();

    let state: HashMap<String, T> = input
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
    map: &HashMap<String, Vec<String>>,
    mut state: HashMap<String, T>,
    mut pulses: VecDeque<Pulse>,
) -> (usize, usize, HashMap<String, T>) {
    let mut high_pulses = 0;
    let mut low_pulses = 0;

    while !pulses.is_empty() {
        let Pulse { from, to, val } = pulses.pop_front().unwrap();
        let mut new_state = state.clone();

        if val {
            high_pulses += 1
        } else {
            low_pulses += 1
        }

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

    (high_pulses, low_pulses, state)
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let (map, mut state) = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let mut tot_high_pulses = 0;
    let mut tot_low_pulses = 0;

    for _ in 0..1000 {
        let pulses: VecDeque<_> = vec![Pulse {
            from: "button".into(),
            to: "broadcaster".into(),
            val: false,
        }]
        .into_iter()
        .collect();

        let (high_pulses, low_pulses, new_state) = process(&map, state, pulses);
        state = new_state;

        tot_high_pulses += high_pulses;
        tot_low_pulses += low_pulses;
    }

    println!("{tot_high_pulses} {tot_low_pulses}");
    let v = tot_high_pulses * tot_low_pulses;

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
