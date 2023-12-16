use aoc2024::read_stdin;
use counter::Counter;
use std::{str::FromStr, time::Instant};

type Cards = [usize; 5];

#[derive(Eq, PartialEq, PartialOrd, Copy, Clone, Debug)]
enum Hand {
    HighCard(Cards),
    Pair(Cards),
    TwoPair(Cards),
    ThreeOfAKind(Cards),
    FullHouse(Cards),
    FourOfAKind(Cards),
    FiveOfAKind(Cards),
}

fn cards_to_hand(cards: Cards) -> Hand {
    let counts = cards.clone().into_iter().collect::<Counter<_>>();
    if *counts.values().max().unwrap() == 5 {
        Hand::FiveOfAKind(cards)
    } else if *counts.values().max().unwrap() == 4 {
        Hand::FourOfAKind(cards)
    } else if *counts.values().max().unwrap() == 3 && *counts.values().min().unwrap() == 2 {
        Hand::FullHouse(cards)
    } else if *counts.values().max().unwrap() == 3 {
        Hand::ThreeOfAKind(cards)
    } else if counts.values().filter(|v| **v == 2).count() == 2 {
        Hand::TwoPair(cards)
    } else if counts.values().filter(|v| **v == 2).count() == 1 {
        Hand::Pair(cards)
    } else {
        Hand::HighCard(cards)
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<usize> = s
            .chars()
            .map(|c| {
                if let Some(n) = c.to_digit(10) {
                    n as usize
                } else {
                    match c {
                        'T' => 10,
                        'J' => 1,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Unknown char {}", c),
                    }
                }
            })
            .collect();

        let cards = [v[0], v[1], v[2], v[3], v[4]];
        let mut hand = Hand::HighCard([1, 1, 1, 1, 1]);

        for a in 2..15 {
            for b in 2..15 {
                for c in 2..15 {
                    for d in 2..15 {
                        for e in 2..15 {
                            let new_hand = cards_to_hand([
                                if cards[0] == 1 { a } else { cards[0] },
                                if cards[1] == 1 { b } else { cards[1] },
                                if cards[2] == 1 { c } else { cards[2] },
                                if cards[3] == 1 { d } else { cards[3] },
                                if cards[4] == 1 { e } else { cards[4] },
                            ]);

                            let new_hand = match new_hand {
                                Hand::FiveOfAKind(_) => Hand::FiveOfAKind(cards),
                                Hand::FourOfAKind(_) => Hand::FourOfAKind(cards),
                                Hand::FullHouse(_) => Hand::FullHouse(cards),
                                Hand::ThreeOfAKind(_) => Hand::ThreeOfAKind(cards),
                                Hand::TwoPair(_) => Hand::TwoPair(cards),
                                Hand::Pair(_) => Hand::Pair(cards),
                                Hand::HighCard(_) => Hand::HighCard(cards),
                            };

                            if new_hand > hand {
                                hand = new_hand
                            }

                            if cards[4] != 1 {
                                break;
                            }
                        }
                        if cards[3] != 1 {
                            break;
                        }
                    }
                    if cards[2] != 1 {
                        break;
                    }
                }
                if cards[1] != 1 {
                    break;
                }
            }
            if cards[0] != 1 {
                break;
            }
        }

        Ok(hand)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or_else(|| {
            self.cards()
                .iter()
                .zip(other.cards())
                .map(|(a, b)| a.cmp(b))
                .filter(|c| *c != std::cmp::Ordering::Equal)
                .next()
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

impl Hand {
    fn cards(&self) -> &Cards {
        match self {
            Hand::FiveOfAKind(cards) => cards,
            Hand::FourOfAKind(cards) => cards,
            Hand::FullHouse(cards) => cards,
            Hand::ThreeOfAKind(cards) => cards,
            Hand::TwoPair(cards) => cards,
            Hand::Pair(cards) => cards,
            Hand::HighCard(cards) => cards,
        }
    }
}

type Input = Vec<(Hand, usize)>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| {
            let (hand, score) = l.split_once(' ').unwrap();
            (hand.parse().unwrap(), score.parse().unwrap())
        })
        .collect()
}

fn main() {
    let start = Instant::now();
    let input = read_stdin();
    let mut hands = parse(input);
    let parse_time = start.elapsed();
    let parsed = Instant::now();

    hands.sort_by_key(|i| i.0);

    let v: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * bet)
        .sum();

    println!("{:?}", v);
    println!("Parse time\t{}μs.", parse_time.as_micros());
    println!("Execution time\t{}μs.", parsed.elapsed().as_micros());
    println!("Total time\t{}μs.", start.elapsed().as_micros());
}
