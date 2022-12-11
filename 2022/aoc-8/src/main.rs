use aoc_8::read_stdin;

type Input = Vec<Vec<usize>>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| l.chars().map(|v| (v as usize) - 48).collect())
        .collect()
}

fn is_visible(input: &Input, i: usize, j: usize) -> bool {
    let val = input[i][j];
    (0..i).all(|x| input[x][j] < val)
        || (i + 1..input.len()).all(|x| input[x][j] < val)
        || (0..j).all(|y| input[i][y] < val)
        || (j + 1..input[0].len()).all(|y| input[i][y] < val)
}

fn part_1(input: Input) -> usize {
    let exterior = 2 * (input.len() + input[0].len()) - 4;
    exterior
        + (1..input.len() - 1)
            .map(|i| {
                (1..input[0].len() - 1)
                    .filter(|j| is_visible(&input, i, *j))
                    .count()
            })
            .sum::<usize>()
}

fn viewing_range(input: &Input, i: usize, j: usize) -> usize {
    let val = input[i][j];
    let l = (i + 1..input.len())
        .filter(|l| input[*l][j] >= val)
        .next()
        .unwrap_or(input.len() - 1);
    let r = (0..=i - 1)
        .rev()
        .filter(|r| input[*r][j] >= val)
        .next()
        .unwrap_or(0);
    let u = (j + 1..input[i].len())
        .filter(|u| input[i][*u] >= val)
        .next()
        .unwrap_or(input[0].len() - 1);
    let d = (0..=j - 1)
        .rev()
        .filter(|d| input[i][*d] >= val)
        .next()
        .unwrap_or(0);
    (u - j) * (l - i) * (j - d) * (i - r)
}

fn part_2(input: Input) -> usize {
    (1..input.len() - 1)
        .map(|i| {
            (1..input[0].len() - 1)
                .map(|j| viewing_range(&input, i, j))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
