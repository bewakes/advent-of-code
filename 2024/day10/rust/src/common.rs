use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

#[derive(Clone, Debug)]
pub struct Input {
    pub array: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn valid_neighbors(&self, inp: &Input) -> Vec<Self> {
        vec![self.left(), self.right(), self.up(), self.down()]
            .into_iter()
            .filter(|e| inp.within_bounds(e))
            .collect()
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

    pub fn up(&self) -> Self {
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
}
pub fn parse_line(line: &str) -> Vec<u8> {
    line.trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
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
