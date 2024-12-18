use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

    pub fn move_once(&self, dir: &Dir) -> Self {
        match dir {
            Dir::Left => Self::new(self.x - 1, self.y),
            Dir::Right => Self::new(self.x + 1, self.y),
            Dir::Up => Self::new(self.x, self.y - 1),
            Dir::Down => Self::new(self.x, self.y + 1),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn is_vertical(&self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            _ => false,
        }
    }
    pub fn is_horizontal(&self) -> bool {
        match self {
            Self::Left | Self::Right => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Box,
    Wall,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item2 {
    BoxL,
    BoxR,
    Wall,
    Empty,
}

pub trait Itm {
    const EMPTY: Self;
}

impl Item {
    pub fn draw(&self) -> &str {
        match self {
            Self::Empty => ".",
            Self::Box => "O",
            Self::Wall => "#",
        }
    }
}

impl Item2 {
    pub fn draw(&self) -> &str {
        match self {
            Self::Empty => ".",
            Self::BoxL => "[",
            Self::BoxR => "]",
            Self::Wall => "#",
        }
    }

    pub fn is_box(&self) -> bool {
        match self {
            Self::BoxL | Self::BoxR => true,
            _ => false,
        }
    }
}

impl Itm for Item {
    const EMPTY: Self = Item::Empty;
}

impl Itm for Item2 {
    const EMPTY: Self = Item2::Empty;
}

pub struct Map<I: Itm = Item> {
    pub width: u64,
    pub height: u64,
    pub arr: Vec<Vec<I>>,
    pub curr_position: Vector,
}

impl<I: Copy + Itm> Map<I> {
    pub fn at(&self, pos: &Vector) -> I {
        self.arr[pos.y as usize][pos.x as usize]
    }

    pub fn set_at(&mut self, pos: &Vector, item: I) {
        self.arr[pos.y as usize][pos.x as usize] = item;
    }

    pub fn shift_n_items_by_one(&mut self, n: u64, dir: &Dir, from: &Vector) {
        let mut curr = from.clone();
        let mut prev_item = I::EMPTY;
        for _ in 0..n {
            let tmp = self.at(&curr);
            self.set_at(&curr, prev_item);
            prev_item = tmp;
            curr = curr.move_once(dir);
        }
    }
}

impl Map<Item2> {
    pub fn shift_vertical_by_one(&mut self, n: u64, dir: &Dir, from: &Vector) {
        assert!(*dir == Dir::Up || *dir == Dir::Down);
        let mut curr = from.clone();
        let (mut replace1, mut replace2) = (Item2::Empty, Item2::Empty);

        let (side_dir, side_block) = if self.at(&curr) == Item2::BoxR {
            (Dir::Left, Item2::BoxL)
        } else if self.at(&curr) == Item2::BoxL {
            (Dir::Right, Item2::BoxR)
        } else {
            panic!("unexpeted")
        };
        for _ in 0..n {
            // println!("Curr at {:?}", curr);
            let tmp = self.at(&curr);
            // println!("Self at {:?}", tmp);
            // println!(
            //     "Horiz {:?}, to_set {:?}, rep1 {:?}, rep2 {:?}",
            //     side_dir, side_block, replace1, replace2
            // );
            self.set_at(&curr, replace1);
            self.set_at(&curr.move_once(&side_dir), replace2);
            replace1 = tmp;
            replace2 = side_block;
            curr = curr.move_once(dir);
        }
    }

    pub fn shift_once_in_dir(&mut self, dir: &Dir, level: &[Vector]) {
        // println!("LEVEL {:?}", level);
        // self.show_level(level);
        for p in level {
            let item = self.at(p);
            // println!("{:?} replacing item {:?} with empty", p, item);
            self.set_at(p, Item2::Empty);
            self.set_at(&p.move_once(dir), item);
            // self.show();
        }
    }

    pub fn show_level(&self, level: &[Vector]) {
        println!(
            "level: {:?}",
            level.iter().map(|x| self.at(x)).collect::<Vec<_>>()
        );
    }

    pub fn show(&self) {
        for (j, row) in self.arr.iter().enumerate() {
            for (i, item) in row.iter().enumerate() {
                if self.curr_position == Vector::new(i as i64, j as i64) {
                    print!("@");
                } else {
                    print!("{}", item.draw());
                }
            }
            println!();
        }
    }
}

pub struct Input<I: Itm = Item> {
    pub movements: Vec<Dir>,
    pub map: Map<I>,
}

pub fn parse_input() -> Input {
    let s = read_to_string(INPUT_PATH).unwrap();
    parse_str(&s)
}

pub fn parse_str(s: &str) -> Input {
    let trimmed: Vec<_> = s.trim().split("\n\n").collect();
    let rawmap = trimmed[0];
    let rawmovements = trimmed[1];
    Input {
        map: parse_map(rawmap.trim()),
        movements: parse_momvements(rawmovements.trim()),
    }
}

fn parse_momvements(raw: &str) -> Vec<Dir> {
    raw.chars()
        .filter_map(|c| match c {
            '>' => Some(Dir::Right),
            '^' => Some(Dir::Up),
            '<' => Some(Dir::Left),
            'v' => Some(Dir::Down),
            _ => None,
        })
        .collect()
}

fn parse_map(raw: &str) -> Map {
    let mut initial_position = Vector::new(0, 0);
    let splitted: Vec<_> = raw.split("\n").collect();
    let height = splitted.len() as u64;
    let width = splitted[0].len() as u64;

    let mut arr = vec![vec![Item::Empty; width as usize]; height as usize];
    for (j, line) in splitted.iter().enumerate() {
        for (i, c) in line.trim().chars().enumerate() {
            if c == '#' {
                arr[j][i] = Item::Wall;
            } else if c == 'O' {
                arr[j][i] = Item::Box;
            } else if c == '@' {
                initial_position = Vector::new(i as i64, j as i64);
            }
        }
    }
    Map {
        width,
        height,
        arr,
        curr_position: initial_position,
    }
}
