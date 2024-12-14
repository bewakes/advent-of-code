use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

#[derive(Clone, Debug)]
pub struct Machine {
    pub prize: Vector,
    pub button_a: Vector,
    pub button_b: Vector,
}

#[derive(Clone, Debug)]
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
}

pub type Input = Vec<Machine>;

pub fn parse_machine(str: &str) -> Machine {
    let splitted: Vec<_> = str.trim().split("\n").collect();
    let araw = splitted[0];
    let braw = splitted[1];
    let praw = splitted[2];
    let araw = araw
        .replace("Button A: ", "")
        .replace("X+", "")
        .replace("Y+", "");
    let braw = braw
        .replace("Button B: ", "")
        .replace("X+", "")
        .replace("Y+", "");
    let praw = praw
        .replace("Prize: ", "")
        .replace("X=", "")
        .replace("Y=", "");

    let items: Vec<Vec<_>> = [araw, braw, praw]
        .iter()
        .map(|x| {
            x.split(", ")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    Machine {
        button_a: Vector::new(items[0][0], items[0][1]),
        button_b: Vector::new(items[1][0], items[1][1]),
        prize: Vector::new(items[2][0], items[2][1]),
    }
}

pub fn parse_input() -> Input {
    let s = read_to_string(INPUT_PATH).unwrap();
    let machines = s.trim().split("\n\n").map(parse_machine).collect();
    machines
}
