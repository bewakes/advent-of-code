use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

#[derive(Clone, Debug)]
pub struct Location {
    pub position: Vector,
    pub velocity: Vector,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Vector {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(&self, f: i64) -> Self {
        Self {
            x: self.x * f,
            y: self.y * f,
        }
    }

    pub fn rem(&self, x: i64, y: i64) -> Self {
        Self {
            x: (self.x % x + x) % x,
            y: (self.y % y + y) % y,
        }
    }
}

pub struct Input {
    pub locations: Vec<Location>,
    pub width: u64,
    pub height: u64,
}

pub fn parse_location(str: &str) -> Location {
    let splitted: Vec<_> = str.trim().split(" ").collect();
    let praw = splitted[0];
    let vraw = splitted[1];
    let praw = praw.replace("p=", "");
    let vraw = vraw.replace("v=", "");

    let items: Vec<Vec<_>> = [praw, vraw]
        .iter()
        .map(|x| {
            x.split(",")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    Location {
        position: Vector::new(items[0][0], items[0][1]),
        velocity: Vector::new(items[1][0], items[1][1]),
    }
}

pub fn parse_input(w: u64, h: u64) -> Input {
    let s = read_to_string(INPUT_PATH).unwrap();
    parse_str(&s, w, h)
}

pub fn parse_str(s: &str, w: u64, h: u64) -> Input {
    let locations = s.trim().split("\n").map(parse_location).collect();
    Input {
        width: w,
        height: h,
        locations,
    }
}
