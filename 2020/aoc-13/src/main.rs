use aoc::read_stdin;
use num::bigint::{BigInt, ToBigInt};

fn part_1(input: String) -> usize {
    let t: usize = input.lines().nth(0).unwrap().parse().unwrap();
    let buses = input.lines().nth(1).unwrap().split(',');

    let mut lowest: (usize, usize) = (1, 0);
    for bus in buses {
        if bus == "x" {
            continue
        }
        let bus: usize = bus.parse().unwrap();
        let next_bus = bus - (t % bus);
        if lowest.1 == 0 || next_bus < lowest.0 {
            lowest = (next_bus, bus)
        }
    }

    lowest.0 * lowest.1
}

fn gcd_extended(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if a == num::zero() {
        (b, num::zero(), num::one())
    } else {
        let (gcd, x1, y1) = gcd_extended(b.clone() % a.clone(), a.clone());
        let x = y1 - (b.clone()/a.clone()) * x1.clone();
        let y = x1  ;

        (gcd, x, y)
    }
}

fn part_2(input: String) -> BigInt {
    let buses = input.lines().nth(1).unwrap().split(',');

    let mut prev: (BigInt, BigInt) = (num::one(), num::one());
    for (i, bus) in buses.enumerate() {
        let i = i.to_bigint().unwrap();
        if bus == "x" {
            continue
        }
        let bus: BigInt = bus.parse().unwrap();
        let (_, a, b) = gcd_extended(bus.clone(), prev.0.clone());
        let next = (bus.clone() * a * prev.1) - (prev.0.clone() * b * i);
        prev = (bus * prev.0, next)
    }
    let mut res = prev.1 % prev.0.clone();
    while res < num::zero() {
        res += prev.0.clone()
    }
    res
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}



// x % A = n
//
// x = cA + n
// x+1 = dB + m
