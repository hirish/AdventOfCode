use std::io::{self, Read};
use std::str::FromStr;

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn read_stdin_numbers() -> Vec<u32> {
    read_stdin()
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
