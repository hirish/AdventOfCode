use fxhash::{FxHashMap, FxHashSet};

pub struct Node {
    _id: u8,
    neighbours: Vec<u8>,
}

impl Node {
    pub fn new(id: u8, neighbours: Vec<u8>) -> Self {
        Self {
            _id: id,
            neighbours,
        }
    }
}

pub type Graph = FxHashMap<u8, Node>;

pub fn dijkstra(input: &Graph, start: &u8) -> FxHashMap<u8, usize> {
    let mut distances: FxHashMap<&u8, usize> = FxHashMap::default();
    let mut visited: FxHashMap<u8, usize> = FxHashMap::default();
    let mut unvisited: FxHashSet<&u8> = FxHashSet::default();

    distances.insert(start, 0);
    unvisited.insert(start);

    while !unvisited.is_empty() {
        let (&p, &d) = distances.iter().min_by_key(|(_, d)| *d).unwrap();
        let node = &input[p];
        unvisited.remove(&p);
        distances.remove(&p);
        visited.insert(*p, d);

        for neighbour in &node.neighbours {
            if visited.contains_key(neighbour) {
                continue;
            }

            if let Some(&e) = distances.get(neighbour) {
                if d + 1 < e {
                    distances.insert(neighbour, d + 1);
                }
            } else {
                unvisited.insert(neighbour);
                distances.insert(neighbour, d + 1);
            }
        }
    }

    visited
}
