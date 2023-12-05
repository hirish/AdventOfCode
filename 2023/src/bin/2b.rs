use aoc2024::read_stdin;

type Input = Vec<Vec<(u32, u32, u32)>>;

fn parse_hand(input: &str) -> (u32, u32, u32) {
    let mut hand = (0, 0, 0);
    input.split(", ").for_each(|c| {
        let (x, col) = c.split_once(" ").unwrap();
        let x = x.parse().unwrap();
        match col {
            "red" => hand.0 = x,
            "green" => hand.1 = x,
            "blue" => hand.2 = x,
            c => panic!("Unknown colour {}", c),
        }
    });
    hand
}

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| {
            let g = l.split_once(": ").unwrap().1;
            g.split("; ").map(parse_hand).collect()
        })
        .collect()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    let res: u32 = lines
        .iter()
        .map(|games| {
            let r = games.iter().map(|x| x.0).max().unwrap();
            let g = games.iter().map(|x| x.1).max().unwrap();
            let b = games.iter().map(|x| x.2).max().unwrap();
            r * g * b
        })
        .sum();

    println!("{}", res)
}
