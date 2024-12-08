use std::{fmt::Write, fs::read_to_string};

const INPUT_PATH: &str = "data/input";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DirectedPos {
    pub pos: Pos,
    pub dir: Dir,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn right(&self) -> Self {
        match self {
            Dir::Up => Self::Right,
            Dir::Right => Self::Down,
            Dir::Down => Self::Left,
            Dir::Left => Self::Up,
        }
    }
}

/// Position
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn up(mut self) -> Self {
        self.y -= 1;
        self
    }
    pub fn down(mut self) -> Self {
        self.y += 1;
        self
    }
    pub fn right(mut self) -> Self {
        self.x += 1;
        self
    }
    pub fn left(mut self) -> Self {
        self.x -= 1;
        self
    }

    pub fn step(self, dir: &Dir) -> Self {
        match dir {
            Dir::Up => self.up(),
            Dir::Down => self.down(),
            Dir::Left => self.left(),
            Dir::Right => self.right(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    pub width: isize,
    pub height: isize,
    pub arr: Vec<Vec<char>>,
}

impl Input {
    pub fn start_position(&self) -> Pos {
        for (y, row) in self.arr.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == '^' {
                    return Pos {
                        x: x as isize,
                        y: y as isize,
                    };
                }
            }
        }
        panic!("Could not find starting point");
    }

    pub fn at(&self, pos: &Pos) -> char {
        self.arr[pos.y as usize][pos.x as usize]
    }

    pub fn in_bound(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    pub fn move_until_obstacle(&self, pos: &Pos, dir: &Dir) -> (Pos, Vec<Pos>) {
        let mut curr = pos.clone();
        let mut visited = Vec::new();
        loop {
            visited.push(curr.clone());
            let next = curr.clone().step(dir);

            if !self.in_bound(&next) || self.at(&next) == '#' {
                return (curr, visited);
            }
            curr = next.clone();
        }
    }
}

pub fn parse_input() -> Input {
    let s = read_to_string(INPUT_PATH).unwrap();
    let arr: Vec<Vec<_>> = s.split_whitespace().map(|x| x.chars().collect()).collect();
    let width = arr[0].len() as isize;
    let height = arr.len() as isize;
    Input { arr, width, height }
}
