use std::io::{self, Read};

pub mod intset;
pub mod graph_fns;

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn read_stdin_numbers() -> Vec<u32> {
    read_stdin()
        .split('\n')
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<u32>>()
}
