use std::time::Instant;
use std::str::FromStr;

use aoc_1::read_stdin;

fn bin_to_usize(bits: &[usize]) -> usize {
    let mut v = 0;
    for &bit in bits.iter() {
        v = v << 1;
        if bit == 1 {
            v += 1
        }
    }
    v
}

#[derive(Clone, Debug)]
struct Input {
    data: Vec<usize>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data = input.chars().map(|c| match c {
            '0' => vec![0,0,0,0],
            '1' => vec![0,0,0,1],
            '2' => vec![0,0,1,0],
            '3' => vec![0,0,1,1],
            '4' => vec![0,1,0,0],
            '5' => vec![0,1,0,1],
            '6' => vec![0,1,1,0],
            '7' => vec![0,1,1,1],
            '8' => vec![1,0,0,0],
            '9' => vec![1,0,0,1],
            'A' => vec![1,0,1,0],
            'B' => vec![1,0,1,1],
            'C' => vec![1,1,0,0],
            'D' => vec![1,1,0,1],
            'E' => vec![1,1,1,0],
            'F' => vec![1,1,1,1],
            _ => panic!("Unknown character '{:?}'", c)
        }).flatten().collect();

        Ok(Input {data})
    }
}

#[derive(Clone, Debug)]
enum Payload {
    Literal {value: usize},
    Operator {
        t: usize,
        packets: Vec<Packet>,
    },
}

#[derive(Clone, Debug)]
struct Packet {
    version: usize,
    payload: Payload,
}

impl Packet {
    fn new(bits: &[usize]) -> Self {
        Packet::inner_new(bits).0
    }

    fn inner_new(bits: &[usize]) -> (Self, usize) {
        let mut eaten = 6;
        let version = bin_to_usize(&bits[0..3]);

        let payload = match bin_to_usize(&bits[3..6]) {
            4 => {
                let mut value = 0;
                loop {
                    value = value << 4;
                    value += bin_to_usize(&bits[eaten+1..eaten+5]);
                    eaten += 5;
                    if bits[eaten - 5] == 0 {
                        break
                    }
                }
                Payload::Literal {value}
            },
            t => {
                let length = bits[6];
                let mut packets = vec![];
                eaten += 1;

                if length == 0 {
                    let t = bin_to_usize(&bits[7..22]);
                    eaten += 15;
                    let to_eat = eaten + t;
                    while eaten < to_eat {
                        let (packet, e) = Packet::inner_new(&bits[eaten..]);
                        eaten += e;
                        packets.push(packet);
                    }
                } else {
                    let no_packets = bin_to_usize(&bits[7..18]);
                    eaten += 11;
                    for _ in 0..no_packets {
                        let (packet, e) = Packet::inner_new(&bits[eaten..]);
                        eaten += e;
                        packets.push(packet);
                    }
                }

                Payload::Operator {packets, t}
            },
        };

        (Packet {version, payload}, eaten)
    }

    fn version_sum(&self) -> usize {
        self.version + match &self.payload {
            Payload::Literal{value: _} => 0,
            Payload::Operator{packets, t: _} => packets.iter().map(|p| p.version_sum()).sum()
        }
    }

    fn calculate(&self) -> usize {
        match &self.payload {
            Payload::Literal{value} => *value,
            Payload::Operator{packets, t} => match t {
                0 => packets.iter().map(|p| p.calculate()).sum(),
                1 => packets.iter().map(|p| p.calculate()).fold(1, |x, y| x * y),
                2 => packets.iter().map(|p| p.calculate()).min().unwrap(),
                3 => packets.iter().map(|p| p.calculate()).max().unwrap(),
                5 => if packets[0].calculate() > packets[1].calculate() {1} else {0},
                6 => if packets[0].calculate() < packets[1].calculate() {1} else {0},
                7 => if packets[0].calculate() == packets[1].calculate() {1} else {0},
                _ => panic!("Unknown type {}", t),
            },
        }
    }
}

fn part_1(input: Input) -> Option<usize> {
    let packet = Packet::new(&input.data);
    Some(packet.version_sum())
}

fn part_2(input: Input) -> Option<usize> {
    let packet = Packet::new(&input.data);
    Some(packet.calculate())
}

fn main() -> Result<(), ()> {
    let now = Instant::now();
    let input: Input = read_stdin().parse()?;
    println!("Running parsing took {}μs.", now.elapsed().as_micros());

    let now = Instant::now();
    println!("Answer 1: {}", part_1(input.clone()).ok_or(())?);
    println!("Running part_1 took {}μs.", now.elapsed().as_micros());

    let now = Instant::now();
    println!("Answer 2: {}", part_2(input).ok_or(())?);
    println!("Running part_2 took {}μs.", now.elapsed().as_micros());

    Ok(())
}
