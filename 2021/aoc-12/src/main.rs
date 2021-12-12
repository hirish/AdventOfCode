use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use aoc_1::read_stdin;

#[derive(Eq, Hash)]
struct Path<'a> {
    v: &'a str,
    cons: Option<Rc<Self>>,
}

impl PartialEq for Path<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v && self.cons == other.cons
    }
}

impl fmt::Debug for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cons.is_none() {
            write!(f, "[{:?}]", self.v)?
        } else {
            write!(f, "[{:?}, {:?}]", self.v, self.cons.as_ref().unwrap())?
        }
        Ok(())
    }
}

fn bfs(map: HashMap<&str, HashSet<&str>>, valid_fn: fn(Rc<Path>) -> bool) -> usize {
    let mut complete_paths = 0;
    let mut incomplete_paths: Vec<Rc<Path>> = vec![];
    incomplete_paths.push(Rc::new(Path {
        v: "end",
        cons: None,
    }));

    while incomplete_paths.len() > 0 {
        incomplete_paths = incomplete_paths
            .into_iter()
            .map(|path| {
                let mut new_paths = vec![];
                for neighbour in map.get(path.v).unwrap() {
                    let new_path = Rc::new(Path {
                        v: neighbour,
                        cons: Some(Rc::clone(&path)),
                    });

                    if neighbour == &"start" {
                        complete_paths += 1
                    } else if valid_fn(Rc::clone(&new_path)) {
                        new_paths.push(new_path);
                    }
                }
                new_paths
            })
            .flatten()
            .collect();
    }

    complete_paths
}

fn is_large(x: &str) -> bool {
    x.chars().all(|c| c.is_ascii_uppercase())
}

fn is_valid(mut path: Rc<Path>) -> bool {
    let mut visited = HashSet::new();

    loop {
        if !is_large(path.v) {
            if visited.contains(path.v) {
                return false;
            }

            visited.insert(path.v);
        }

        if path.cons.is_none() {
            break;
        }

        path = Rc::clone(&path.cons.as_ref().unwrap())
    }

    return true;
}

fn part_1(map: HashMap<&str, HashSet<&str>>) -> usize {
    bfs(map, is_valid)
}

fn is_valid2(mut path: Rc<Path>) -> bool {
    let mut visited = HashSet::new();
    let mut has_double_path = false;

    loop {
        if !is_large(path.v) {
            if visited.contains(path.v) {
                if path.v == "start" || path.v == "end" || has_double_path {
                    return false;
                }

                has_double_path = true
            }

            visited.insert(path.v);
        }

        if path.cons.is_none() {
            break;
        }

        path = Rc::clone(&path.cons.as_ref().unwrap())
    }

    return true;
}

fn part_2(map: HashMap<&str, HashSet<&str>>) -> usize {
    bfs(map, is_valid2)
}

fn main() {
    let input = read_stdin();

    let mut map = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        map.entry(from).or_insert(HashSet::new()).insert(to);
        map.entry(to).or_insert(HashSet::new()).insert(from);
    }

    println!("Answer 1: {}", part_1(map.clone()));
    println!("Answer 2: {}", part_2(map));
}
