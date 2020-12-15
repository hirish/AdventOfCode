use aoc::read_stdin;
use std::collections::HashMap;

fn to_masks(sn: &str) -> (usize, usize) {
    let mut zero_mask = 0u64;
    let mut one_mask = 0u64;
    for (i, c) in sn.chars().rev().enumerate() {
        if c == '1' {
            one_mask += 2u64.pow(i as u32);
            zero_mask += 2u64.pow(i as u32);
        } else if c == 'X' {
            zero_mask += 2u64.pow(i as u32);
        }
    }
    (zero_mask as usize, one_mask as usize)
}

fn parse_line(line: &str) -> (usize, usize) {
    let addr = line
        .split("]").nth(0).unwrap()
        .split("[").nth(1).unwrap()
        .parse().unwrap();

    let val = line
        .split("= ").nth(1).unwrap()
        .parse().unwrap();

    (addr, val)
}

fn process_mask(lines: &str, memory: &mut HashMap<usize, usize>) {
    let (zero_mask, one_mask) = to_masks(lines.lines().nth(0).unwrap());

    for (addr, n) in lines.lines().skip(1).map(parse_line) {
        memory.insert(addr, (n | one_mask) & zero_mask);
    }
}

fn part_1(input: String) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for mask in input.split("mask = ").skip(1) {
        process_mask(mask, &mut memory)
    }

    memory.values().sum()
}

fn get_addresses(address: &str) -> Vec<usize> {
    if address.contains('X') {
        let mut x = get_addresses(&address.replacen("X", "0", 1));
        x.append(&mut get_addresses(&address.replacen("X", "1", 1)));
        x
    } else {
        vec![to_masks(address).0]
    }
}

fn mask_address(mask: &str, address: usize) -> String {
    let mut x = String::new();
    for (i, c) in mask.chars().rev().enumerate() {
        if c == 'X' {
            x = format!("X{}", &x)
        } else if c == '1' {
            x = format!("1{}", &x)
        } else {
            x = format!("{}{}", (address >> i) % 2, &x)
        }
    }
    x
}

fn part_2(input: String) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for lines in input.split("mask = ").skip(1) {
        let mask = lines.lines().nth(0).unwrap();
        for (addr, n) in lines.lines().skip(1).map(parse_line) {
            let addr = mask_address(mask, addr);
            for address in get_addresses(&addr) {
                memory.insert(address, n);
            }
        }
    }

    memory.values().sum()
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
