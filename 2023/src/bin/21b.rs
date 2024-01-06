use aoc2024::read_stdin;
use rustc_hash::FxHashSet;
use std::time::Instant;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
struct Coord(isize, isize);

impl Coord {
    fn neighbours(&self) -> [Self; 4] {
        [
            Coord(self.0 - 1, self.1),
            Coord(self.0 + 1, self.1),
            Coord(self.0, self.1 - 1),
            Coord(self.0, self.1 + 1),
        ]
    }
}

struct Graph(FxHashSet<Coord>, isize, isize);

impl Graph {
    fn contains(&self, pos: &Coord) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.2 && pos.1 < self.1 && !self.0.contains(pos)
    }
}

type Input = (Coord, Graph);

fn parse(input: String) -> Input {
    let start = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                'S' => Some(Coord(x as isize, y as isize)),
                _ => None,
            })
        })
        .next()
        .unwrap();

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Coord(x as isize, y as isize)),
                _ => None,
            })
        })
        .collect();

    let h = input.lines().count() as isize;
    let w = input.lines().next().unwrap().chars().count() as isize;

    (start, Graph(map, h, w))
}

fn reachable(graph: &Graph, pos: Coord, steps: isize) -> FxHashSet<Coord> {
    let mut queue = FxHashSet::default();
    queue.insert(pos);
    for _ in 0..steps {
        queue = queue
            .iter()
            .flat_map(|p| {
                p.neighbours().into_iter().filter_map(|n| {
                    if graph.contains(&n) {
                        Some(n)
                    } else {
                        None
                    }
                })
            })
            .collect();
    }
    queue
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let (spos, graph) = parse(input.clone());
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    let steps = 26501365;
    let odds = reachable(&graph, spos, graph.1); //Starting block
    let evens = reachable(&graph, spos, graph.1 + 1);

    let blocks = steps / graph.1 as usize;
    let evens_blocks = (blocks).pow(2);
    let odd_blocks = (blocks - 1).pow(2);

    let points_d = 130;
    let n = reachable(&graph, Coord(65, 130), points_d);
    let s = reachable(&graph, Coord(65, 0), points_d);
    let e = reachable(&graph, Coord(0, 65), points_d);
    let w = reachable(&graph, Coord(130, 65), points_d);

    let odds_d = 64;
    let odds_nw = reachable(&graph, Coord(0, 0), odds_d);
    let odds_sw = reachable(&graph, Coord(130, 0), odds_d);
    let odds_ne = reachable(&graph, Coord(0, 130), odds_d);
    let odds_se = reachable(&graph, Coord(130, 130), odds_d);

    let evens_d = 130 + 65;
    let evens_nw = reachable(&graph, Coord(0, 0), evens_d);
    let evens_sw = reachable(&graph, Coord(130, 0), evens_d);
    let evens_ne = reachable(&graph, Coord(0, 130), evens_d);
    let evens_se = reachable(&graph, Coord(130, 130), evens_d);

    let mut visited: usize = 0;
    visited += evens_blocks * evens.len() + odd_blocks * odds.len();
    visited += blocks * (odds_nw.len() + odds_ne.len() + odds_sw.len() + odds_se.len());
    visited += (blocks - 1) * (evens_nw.len() + evens_ne.len() + evens_sw.len() + evens_se.len());
    visited += e.len() + w.len() + n.len() + s.len();

    println!("{:?}", visited);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
