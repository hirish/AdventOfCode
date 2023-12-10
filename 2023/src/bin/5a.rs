use aoc2024::read_stdin;

type Map = Vec<(usize, usize, usize)>;

fn map(map: &Map, from: usize) -> usize {
    let m = map
        .iter()
        .find(|(_, from_s, l)| from >= *from_s && from < (*from_s + l));

    if let Some((to_s, from_s, _)) = m {
        to_s + (from - from_s)
    } else {
        from
    }
}

type Input = (Vec<usize>, Vec<Map>);

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .skip(1)
        .map(|l| {
            let mut l = l.split_whitespace().map(|n| n.parse::<usize>().unwrap());
            (l.next().unwrap(), l.next().unwrap(), l.next().unwrap())
        })
        .collect()
}

fn parse(input: String) -> Input {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let maps: Vec<Map> = maps.split("\n\n").map(parse_map).collect();

    (seeds, maps)
}

fn main() {
    let input = read_stdin();
    let (seeds, maps) = parse(input);

    let v = seeds
        .iter()
        .map(|s| {
            let mut s = *s;
            for m in &maps {
                s = map(&m, s)
            }
            s
        })
        .min()
        .unwrap();

    println!("{:?}", v)
}
