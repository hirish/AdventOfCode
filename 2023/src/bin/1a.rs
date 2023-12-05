use aoc2024::read_stdin;

type Input = Vec<String>;

fn parse(input: String) -> Input {
    input.lines().map(|l| l.to_string()).collect()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    let v: u32 = lines
        .into_iter()
        .map(|s| {
            let li = s.find(char::is_numeric).expect("No left digit");
            let ri = s.rfind(char::is_numeric).expect("No right digit");
            let l = s.chars().nth(li).unwrap().to_digit(10).unwrap();
            let r = s.chars().nth(ri).unwrap().to_digit(10).unwrap();
            l * 10 + r
        })
        .sum();

    println!("{}", v);
}
