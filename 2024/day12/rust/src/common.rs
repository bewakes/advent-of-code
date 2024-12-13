use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const INPUT_PATH: &str = "data/input";

#[derive(Clone, Debug)]
pub struct Input {
    pub array: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(i: usize, j: usize) -> Self {
        Self {
            x: i as i32,
            y: j as i32,
        }
    }

    pub fn valid_neighbors(&self, inp: &Input) -> Vec<Self> {
        vec![self.left(), self.right(), self.top(), self.down()]
            .into_iter()
            .filter(|e| inp.within_bounds(e))
            .collect()
    }

    pub fn get_perimeter(&self, char: char, input: &Input) -> u64 {
        [self.left(), self.right(), self.top(), self.down()]
            .iter()
            .filter(|p| !input.within_bounds(p) || input.array[p.y as usize][p.x as usize] != char)
            .count() as u64
    }

    pub fn within_bounds(&self, input: &Input) -> bool {
        input.within_bounds(self)
    }

    pub fn at(&self, input: &Input) -> char {
        let x = self.x as usize;
        let y = self.y as usize;
        input.array[y][x]
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn top(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
}

impl Input {
    pub fn within_bounds(&self, p: &Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width as i32 && p.y < self.height as i32
    }

    pub fn sides(&self, p: &Point, c: &char) -> Vec<Side> {
        let mut sides = Vec::new();
        if !p.left().within_bounds(self) || p.left().at(self) != *c {
            sides.push(Side::L)
        }
        if !p.right().within_bounds(self) || p.right().at(self) != *c {
            sides.push(Side::R)
        }
        if !p.top().within_bounds(self) || p.top().at(self) != *c {
            sides.push(Side::T)
        }
        if !p.down().within_bounds(self) || p.down().at(self) != *c {
            sides.push(Side::D)
        }
        sides
    }
}
pub fn parse_line(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}

pub fn parse_str(raw: &str) -> Input {
    let array: Vec<_> = raw.trim().split_terminator("\n").map(parse_line).collect();
    let width = array[0].len();
    let height = array.len();
    Input {
        array,
        width,
        height,
    }
}

pub fn parse_input() -> Input {
    let raw = read_to_string(INPUT_PATH).unwrap();
    parse_str(&raw)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Side {
    T,
    D,
    L,
    R,
}

pub fn get_sides_count(chr: &char, region: &[Point], input: &Input) -> u64 {
    let mut sides = 0;
    let mut sides_map: HashMap<Point, Vec<Side>> =
        HashMap::from_iter(region.iter().map(|k| (k.clone(), Vec::new())));

    for j in 0..input.height {
        for i in 0..input.width {
            let p = Point::new(i, j);
            if !sides_map.contains_key(&p) {
                continue;
            }
            let psides = input.sides(&p, chr);
            sides_map.insert(p.clone(), psides.clone());
            for side in psides {
                match side {
                    Side::L => {
                        if !sides_map
                            .get(&p.down())
                            .map(|x| x.contains(&Side::L))
                            .unwrap_or(false)
                        {
                            // println!("left increment as down {:?} has none", p.down());
                            sides += 1;
                        }
                    }
                    Side::R => {
                        if !sides_map
                            .get(&p.down())
                            .map(|x| x.contains(&Side::R))
                            .unwrap_or(false)
                        {
                            // println!("right increment as down {:?} has none", p.down());
                            sides += 1;
                        }
                    }
                    Side::T => {
                        if !sides_map
                            .get(&p.left())
                            .map(|x| x.contains(&Side::T))
                            .unwrap_or(false)
                        {
                            // println!("top increment as left {:?} has none", p.left());
                            sides += 1;
                        }
                    }
                    Side::D => {
                        if !sides_map
                            .get(&p.left())
                            .map(|x| x.contains(&Side::D))
                            .unwrap_or(false)
                        {
                            // println!("down increment as left {:?} has none", p.left());
                            sides += 1;
                        }
                    }
                }
            }
        }
    }
    sides
}
