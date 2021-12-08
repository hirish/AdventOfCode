use aoc_1::read_stdin;

fn part_1(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let display = line.split(" | ").nth(1).unwrap();
            display
                .split(' ')
                .filter(|d| d.len() == 2 || d.len() == 3 || d.len() == 4 || d.len() == 7)
                .count()
        })
        .sum()
}

fn contains(num: &str, other_num: &str) -> bool {
    other_num
        .chars()
        .all(|c| num.contains(c))
}

fn common(num: &str, other_num: &str) -> usize {
    other_num
        .chars()
        .map(|c| num.contains(c))
        .filter(|c| *c)
        .count()
}

fn sort_chars(x: &str) -> String {
    let mut c: Vec<char> = x.chars().collect();
    c.sort_by(|a, b| a.cmp(b));
    c.iter().collect()
}

fn load_numbers(x: &str) -> Vec<String> {
    x
        .split(' ')
        .map(sort_chars)
        .collect()
}

fn decode(line: &str) -> Option<usize> {
    let mut t = line.split(" | ");
    let code = load_numbers(t.next()?);
    let display = load_numbers(t.next()?);

    // Simple
    let one = code.iter().filter(|x| x.len() == 2).nth(0)?;
    let four = code.iter().filter(|x| x.len() == 4).nth(0)?;
    let seven = code.iter().filter(|x| x.len() == 3).nth(0)?;
    let eight = code.iter().filter(|x| x.len() == 7).nth(0)?;

    // Computed
    let six = code.iter().filter(|x| x.len() == 6 && !contains(x, &one)).nth(0)?;
    let nine = code.iter().filter(|x| x.len() == 6 && contains(x, &four)).nth(0)?;
    let zero = code.iter().filter(|x| x != &six && x.len() == 6 && !contains(x, &four)).nth(0)?;
    let three = code.iter().filter(|x| x.len() == 5 && contains(x, &one)).nth(0)?;
    let two = code.iter().filter(|x| x.len() == 5 && common(x, &four) == 2).nth(0)?;
    let five = code.iter().filter(|x|  x != &three && x.len() == 5 && common(x, &four) == 3).nth(0)?;

    display
        .iter()
        .map(|v| 
            if v == zero {'0'} else if v == one {'1'} else if v == two {'2'} else if v == three {'3'}
            else if v == four {'4'} else if v == five {'5'} else if v == six {'6'} else if v == seven {'7'}
            else if v == eight {'8'} else if v == nine {'9'} else {panic!()}
        )
        .collect::<String>()
        .parse().ok()
}

fn part_2(input: String) -> usize {
    input
        .lines()
        .map(|x| decode(x).unwrap())
        .sum()
}

fn main() {
    let input = read_stdin();

    println!("Answer 1: {}", part_1(input.clone()));
    println!("Answer 2: {}", part_2(input));
}
