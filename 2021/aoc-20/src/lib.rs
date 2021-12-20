use std::io::{self, Read};
use std::time::Instant;

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

pub fn duration(start: Instant) -> String {
    let d = start.elapsed();
    if d.as_micros() <= 1000 {
        format!("{}μs", d.as_micros())
    } else if d.as_millis() <= 1000 {
        format!("{}ms", d.as_millis())
    } else {
        format!("{}s", d.as_secs())
    }
}
