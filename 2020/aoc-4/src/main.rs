use aoc::read_stdin;

fn len(l:usize) -> impl Fn(&str) -> bool {
    move |input: &str| input.len() == l
}

fn min(min:usize) -> impl Fn(&str) -> bool {
    move |input: &str| {
        let i: usize = input.parse().unwrap();
        i >= min
    }
}

fn max(min:usize) -> impl Fn(&str) -> bool {
    move |input: &str| {
        let i: usize = input.parse().unwrap();
        i <= min
    }
}

fn height(input: &str) -> bool {
    if let Some(x) = input.strip_suffix("in") {
        min(59)(x) && max(76)(x)
    } else if let Some(x) = input.strip_suffix("cm") {
        min(150)(x) && max(193)(x)
    } else {
        false
    }
}

fn hair_color(input: &str) -> bool {
    let valid_chars = "0123456789abcdef";
    if let Some(x) = input.strip_prefix('#') {
        for c in x.chars() {
            if !valid_chars.contains(c) {
                return false
            }
        }
        true
    } else {
        false
    }
}

fn eye_color(input: &str) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input)
}

fn is_number(input: &str) -> bool {
    input.parse::<u32>().map_or(false, |_| true)
}

fn is_valid(key: &str, val: &str, validate: bool) -> bool {
    if validate {
        match key {
            "byr" => len(4)(val) && min(1920)(val) && max(2002)(val),
            "iyr" => len(4)(val) && min(2010)(val) && max(2020)(val),
            "eyr" => len(4)(val) && min(2020)(val) && max(2030)(val),
            "hgt" => height(val),
            "hcl" => hair_color(val),
            "ecl" => eye_color(val),
            "pid" => len(9)(val) && is_number(val),
            "cid" => true,
            _ => false
        }
    } else {
        match key {
            "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" | "cid" => true,
            _ => false
        }
    }
}

fn get_args(passport: &str) -> Vec<(String, String)> {
    passport
        .replace('\n', " ")
        .split(' ')
        .map(|x| x.split(':').collect::<Vec<&str>>())
        .map(|x| (x[0].to_string(), x[1].to_string()))
        .collect()
}


pub fn count_valid(input: &str, validate_fields: bool) -> usize {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid = 0;
    for passport in input.split("\n\n") {
        let args = get_args(passport);
        let keys: Vec<&str> = args.iter().map(|x| x.0.as_ref()).collect();

        let has_all_keys = required_keys
            .iter()
            .filter(|key| !keys.contains(key))
            .count() == 0;

        let is_valid = args
            .iter()
            .filter(|(x, y)| !is_valid(x, y, validate_fields))
            .count() == 0;

        if is_valid && has_all_keys {
            valid += 1
        }
    }
    valid
}

fn part_1(input: String) -> usize {
    count_valid(&input, false)
}

fn part_2(input: String) -> usize {
    count_valid(&input, true)
}

fn main() {
    let lines = read_stdin();
    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines.clone()));
}
