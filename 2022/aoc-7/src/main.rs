use aoc_6::read_stdin;

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Clone, Debug)]
struct Input {
    files: HashMap<PathBuf, usize>,
    dirs: HashSet<PathBuf>,
}

#[derive(Clone, Debug)]
enum CdArg {
    Root,
    Up,
    Down(String),
}

impl FromStr for CdArg {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/" => Ok(Self::Root),
            ".." => Ok(Self::Up),
            _ => Ok(Self::Down(s.to_string())),
        }
    }
}

#[derive(Clone, Debug)]
enum Cmd {
    Cd(CdArg),
    Ls(Vec<Obj>),
}

#[derive(Clone, Debug)]
enum Obj {
    File(String, usize),
    Dir(String),
}

impl FromStr for Obj {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ").unwrap() {
            ("dir", name) => Ok(Self::Dir(name.to_string())),
            (size, name) => Ok(Self::File(name.to_string(), size.parse().unwrap())),
        }
    }
}

impl FromStr for Cmd {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..2] {
            "cd" => Ok(Self::Cd(s[3..].parse().unwrap())),
            "ls" => Ok(Self::Ls(
                s[3..].lines().map(|s| s.parse().unwrap()).collect(),
            )),
            _ => Err(()),
        }
    }
}

fn construct(input: Vec<Cmd>) -> Input {
    let mut curr = Path::new("/").to_owned();
    let mut files = HashMap::<PathBuf, usize>::new();
    let mut dirs = HashSet::<PathBuf>::new();
    for cmd in input {
        curr = match cmd {
            Cmd::Cd(CdArg::Root) => Path::new("/").to_owned(),
            Cmd::Cd(CdArg::Up) => curr.parent().unwrap().to_owned(),
            Cmd::Cd(CdArg::Down(dir)) => curr.join(Path::new(&dir)),
            Cmd::Ls(objs) => {
                objs.iter().for_each(|obj| match obj {
                    Obj::File(name, size) => {
                        files.insert(curr.join(Path::new(&name)), *size);
                    }
                    Obj::Dir(name) => {
                        dirs.insert(curr.join(Path::new(&name)));
                    }
                });
                curr
            }
        }
    }

    Input{files, dirs}
}

fn parse(input: String) -> Input {
    construct(
        input[2..]
            .split("\n$ ")
            .map(|cmd| cmd.parse().unwrap())
            .collect(),
    )
}

fn size(dir: &Path, files: &HashMap<PathBuf, usize>) -> usize {
    files
        .iter()
        .filter(|(name, _)| name.starts_with(&dir))
        .map(|(_, size)| size)
        .sum()
}

fn part_1(Input { files, dirs }: Input) -> usize {
    dirs.iter()
        .map(|dir| size(dir, &files))
        .filter(|size| *size <= 100000)
        .sum()
}

fn part_2(Input { files, dirs }: Input) -> usize {
    let disk_size = 70000000;
    let update_size = 30000000;
    let free_space = disk_size - size(Path::new("/"), &files);
    let required_space = update_size - free_space;

    dirs.iter()
        .map(|dir| size(dir, &files))
        .filter(|size| *size > required_space)
        .min()
        .unwrap()
}

fn main() {
    let input = read_stdin();
    let lines = parse(input);

    println!("Answer 1: {}", part_1(lines.clone()));
    println!("Answer 2: {}", part_2(lines));
}
