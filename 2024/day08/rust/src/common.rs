use std::{collections::HashMap, fs::read_to_string};

const INPUT_PATH: &str = "data/input";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub width: isize,
    pub height: isize,
    pub antennas: HashMap<char, Vec<Position>>,
}

impl Input {
    pub fn is_in_bound(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn diff(&self, other: &Self) -> Self {
        Self::new(other.x - self.x, other.y - self.y)
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(other.x + self.x, other.y + self.y)
    }

    pub fn negate(&self) -> Self {
        self.scale(-1)
    }

    pub fn scale(&self, n: isize) -> Self {
        Self::new(self.x * n, self.y * n)
    }
}

pub fn parse_input() -> Input {
    let grid = read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<&str> = grid.split_terminator("\n").collect();
    let mut antennas = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                let pos = Position::new(x as isize, y as isize);
                antennas.entry(char).or_insert_with(Vec::new).push(pos);
            }
        }
    }
    Input {
        width: lines.len() as isize,
        height: lines[0].len() as isize,
        antennas,
    }
}
