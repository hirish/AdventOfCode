use std::io::{self, Read};

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn read_stdin_numbers(splitter: &str) -> Vec<usize> {
    read_stdin()
        .split(splitter)
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
