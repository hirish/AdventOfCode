use aoc::read_stdin;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: usize,
    pub tile: Vec<Vec<char>>,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.splitn(2, "\n");
        let id = lines.next().unwrap().replace(":", "")[5..].parse().unwrap();
        let tile = lines.next().unwrap();

        Ok(
            Tile::new(id, tile.lines().map(|l| l.chars().collect()).collect())
        )
    }
}

impl Tile {

    pub fn new(id: usize, tile: Vec<Vec<char>>) -> Self {
        Tile {
            id,
            tile,
        }
    }

    pub fn borderless(&self) -> Tile {
        let mut borderless_tile = vec![];

        for i in 1..self.tile.len()-1 {
            let mut row = vec![];
            for j in 1..self.tile.len()-1 {
                row.push(self.tile[i][j]);
            }
            borderless_tile.push(row);
        }

        Tile {
            id: self.id,
            tile: borderless_tile,
        }
    }

    pub fn top(&self) -> String {
        self.tile[0].iter().collect()
    }

    pub fn left(&self) -> String {
        self.tile.iter().map(|r| r[0]).rev().collect()
    }

    pub fn bottom(&self) -> String {
        self.tile[self.tile.len() - 1].iter().rev().collect()
    }

    pub fn right(&self) -> String {
        self.tile.iter().map(|r| r[r.len() - 1]).collect()
    }

    pub fn borders(&self) -> Vec<String> {
        vec![
            self.top(),
            self.right(),
            self.bottom(),
            self.left(),
        ]
    }

    pub fn normalised_borders(&self) -> Vec<String> {
        vec![
            normalise(self.top()),
            normalise(self.right()),
            normalise(self.bottom()),
            normalise(self.left()),
        ]
    }

    fn read_column(&self, col: usize) -> Vec<char> {
        self.tile.iter().map(|r| r[col]).collect()
    }

    pub fn rotate(&mut self) {
        let mut new: Vec<Vec<char>> = Vec::new();
        for i in 0..self.tile.len() {
            let mut col = self.read_column(i);
            col.reverse();
            new.push(col);
        }
        self.tile = new
    }

    pub fn flip(&mut self) {
        for row in self.tile.iter_mut() {
            row.reverse();
        }
    }

    pub fn orient_to_left(&mut self, border: &String) {
        let flip = !self.borders().contains(border);

        if flip {
            self.flip();
        }
        
        while self.borders()[3] != *border {
            self.rotate()
        }
    }

    pub fn display(&self) {
        for row in self.tile.iter() {
            for v in row.iter() {
                print!("{}", v);
            }
            println!("");
        }
    }
}

fn normalise(s: String) -> String {
    let reversed: String = s.chars().rev().collect();
    if s > reversed {s} else {reversed}
}

fn build_borders(tiles: &HashMap<usize, Tile>) -> HashMap<String, Vec<usize>> {
    let mut borders: HashMap<String, Vec<usize>> = HashMap::new();
    for tile in tiles.values() {
        for border in tile.normalised_borders() {
            let c = borders.entry(border).or_insert_with(|| vec![]);
            c.push(tile.id);
        }
    }

    return borders
}

fn find_corners(tiles: &HashMap<usize, Tile>, borders: &HashMap<String, Vec<usize>>) -> (usize, usize, usize, usize) {
    let corners: Vec<usize> = tiles.iter()
        .filter(|(_, t)| {
            t.normalised_borders().iter()
                .filter(|b| {
                    borders.get(*b).unwrap().len() == 1
                }).count() == 2
        })
        .map(|(k, _)| *k)
        .collect();

    assert!(corners.len() == 4);
    (corners[0], corners[1], corners[2], corners[3])

}


fn part_1(tiles: &HashMap<usize, Tile>) -> usize {
    let borders = build_borders(tiles);
    let corners = find_corners(tiles, &borders);
    corners.0 * corners.1 * corners.2 * corners.3
}

fn orient_corner(tile: &mut Tile, borders: &HashMap<String, Vec<usize>>) {
    loop {
        let right = normalise(tile.right());
        let bottom = normalise(tile.bottom());
        if borders.get(&right).unwrap().len() > 1 && borders.get(&bottom).unwrap().len() > 1 {
            break
        }
        tile.rotate();
    }
}

fn orient_map(start_corner: usize, tiles: &HashMap<usize, Tile>, borders: HashMap<String, Vec<usize>>) -> Vec<Vec<Tile>> {
    let mut prev_tile = tiles.get(&start_corner).unwrap().clone();
    orient_corner(&mut prev_tile, &borders);

    let mut map: Vec<Vec<Tile>> = vec![];
    let mut row: Vec<Tile> = vec![prev_tile];

    loop {
        let prev_tile = row.last().unwrap();

        let right: &String = &prev_tile.borders()[1].chars().rev().collect();
        let normalised_right = &prev_tile.normalised_borders()[1];

        let tiles_with_border = borders.get(normalised_right).unwrap();
        if tiles_with_border.len() == 1 {
            let first = &row[0];
            let bottom: String = first.bottom().chars().rev().collect();
            let normalised_bottom = normalise(first.bottom());
            
            let tiles_with_border = borders.get(&normalised_bottom).unwrap();

            if tiles_with_border.len() == 1 {
                map.push(row);
                break;
            }

            let next_tile_id = tiles_with_border.iter().filter(|t| **t != first.id).next().unwrap();
            let mut next_tile = tiles.get(next_tile_id).unwrap().clone();

            next_tile.orient_to_left(&bottom);
            next_tile.rotate();

            map.push(row);
            row = vec![next_tile];
        } else {
            let next_tile_id = tiles_with_border.iter().filter(|t| **t != prev_tile.id).next().unwrap();
            let mut next_tile = tiles.get(next_tile_id).unwrap().clone();

            next_tile.orient_to_left(right);

            row.push(next_tile);
        }
    }

    map
}

fn construct_map(map: Vec<Vec<Tile>>) -> Vec<Vec<char>> {
    let mut constructed: Vec<Vec<char>> = vec![];
    for row in map {
        let height = row[0].borderless().tile.len();
        for i in 0..height {
            let mut constructed_row = vec![];
            for tile in row.iter() {
                for v in tile.borderless().tile[i].iter() {
                    constructed_row.push(*v)
                }
            }
            constructed.push(constructed_row);
        }
    }
    constructed
}

fn is_monster(map: &Tile, x: usize, y: usize) -> bool {
    map.tile[y][x+18] == '#' &&
    map.tile[y+1][x] == '#' &&
    map.tile[y+1][x+5] == '#' &&
    map.tile[y+1][x+6] == '#' &&
    map.tile[y+1][x+11] == '#' &&
    map.tile[y+1][x+12] == '#' &&
    map.tile[y+1][x+17] == '#' &&
    map.tile[y+1][x+18] == '#' &&
    map.tile[y+1][x+19] == '#' &&
    map.tile[y+2][x+1] == '#' &&
    map.tile[y+2][x+4] == '#' &&
    map.tile[y+2][x+7] == '#' &&
    map.tile[y+2][x+10] == '#' &&
    map.tile[y+2][x+13] == '#' &&
    map.tile[y+2][x+16] == '#'
}

fn search_map(map: &mut Tile) -> usize {
    let size = map.tile.len();

    for _ in 0..=1 {
        map.flip();
        for _ in 0..4 {
            map.rotate();

            let mut count = 0;
            for y in 0..size-1 {
                for x in 0..size-18 {
                    if is_monster(map, x, y) {
                        println!("({}, {})", x, y);
                        count += 1
                    }
                }
            }
            
            if count > 0 {
                return count
            }
        }
    }
    0
}

fn part_2(tiles: &HashMap<usize, Tile>) -> usize {
    let borders = build_borders(&tiles);
    let corners = find_corners(&tiles, &borders);
    let corner_id = *vec![corners.0, corners.1, corners.2, corners.3].iter().min().unwrap();
    let map = orient_map(corner_id, tiles, borders);

    let mut map = Tile::new(1, construct_map(map));
    let no_monsters = search_map(&mut map);

    let no_hash: usize = map.tile.iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum();

    no_hash - (15 * no_monsters)
}


fn main() {
    let input = read_stdin();
    let tiles = input
        .split("\n\n")
        .map(|t| t.parse().unwrap())
        .map(|t: Tile| (t.id, t))
        .collect();

    println!("Answer 1: {}", part_1(&tiles));
    println!("Answer 2: {}", part_2(&tiles));
}
