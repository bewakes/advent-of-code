use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

#[derive(Clone, Debug)]
pub struct Input {
    pub writes: Vec<u64>,
    pub blanks: Vec<u64>,
}

impl Input {
    pub fn to_entries(&self) -> Vec<Entry> {
        let mut entries = vec![];
        for (i, (w, inp)) in self.writes.iter().zip(self.blanks.clone()).enumerate() {
            entries.push(Entry::from_write_item(i as u64, w.clone() as u64));
            entries.push(Entry::from_blank_count(inp as u64));
        }
        // append the last one, if last one is write
        let writes_len = self.writes.len() as u64;
        if self.writes.len() > self.blanks.len() {
            entries.push(Entry::from_write_item(
                writes_len - 1,
                self.writes[writes_len as usize - 1] as u64,
            ));
        }
        entries
    }
}

#[derive(Clone, Debug)]
pub enum Entry {
    Blank {
        remaining: u64,
        occupied: Vec<(Idx, Count)>,
    },
    Write(Idx, Count),
}

pub type Idx = u64;
pub type Count = u64;

impl Entry {
    pub fn from_write_item(idx: u64, val: u64) -> Self {
        Self::Write(idx, val)
    }

    pub fn from_blank_count(c: u64) -> Self {
        Self::Blank {
            remaining: c,
            occupied: vec![],
        }
    }
}

pub fn parse_string(raw: &str) -> Input {
    let mut writes = Vec::new();
    let mut blanks = Vec::new();
    for (i, c) in raw.chars().filter_map(|c| c.to_digit(10)).enumerate() {
        if i % 2 == 0 {
            writes.push(c as u64);
        } else {
            blanks.push(c as u64);
        }
    }
    Input { writes, blanks }
}

pub fn parse_input() -> Input {
    let raw = read_to_string(INPUT_PATH).unwrap();
    parse_string(&raw)
}
