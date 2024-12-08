use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

#[derive(Debug, Clone)]
pub enum Op {
    Add = 0,
    Mul = 1,
    Cat = 2,
}

impl From<u64> for Op {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Add,
            1 => Self::Mul,
            _ => Self::Cat,
        }
    }
}

impl Op {
    pub fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Mul => a * b,
            Op::Add => a + b,
            Op::Cat => format!("{}{}", a, b).parse::<u64>().unwrap(),
        }
    }

    pub fn ops_from_u64_base(base: u64, mut n: u64, nbits: u64) -> Vec<Self> {
        let mut ops = Vec::new();
        let mut count = 0;
        while count < nbits {
            let rem = n % base;
            ops.push(rem.into());

            n /= base;
            count += 1;
        }
        ops
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub value: u64,
    pub items: Vec<u64>,
}

pub type Input = Vec<Entry>;

fn parse_line(line: &str) -> Entry {
    let (valstr, rest) = line.split_once(":").unwrap();
    let value = valstr.parse::<u64>().unwrap();
    let items = rest
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    Entry { value, items }
}

pub fn parse_input() -> Input {
    let raw = read_to_string(INPUT_PATH).unwrap();
    raw.split_terminator("\n").map(parse_line).collect()
}
