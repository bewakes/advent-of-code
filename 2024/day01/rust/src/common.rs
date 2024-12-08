use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

pub fn parse_line(s: &str) -> Option<(u32, u32)> {
    let mut items = s.split_whitespace();
    let a = items.next()?.parse::<u32>().ok()?;
    let b = items.next()?.parse::<u32>().ok()?;
    if items.next().is_some() {
        // more than 2 items
        return None;
    }
    Some((a, b))
}

pub fn parse_input() -> Vec<(u32, u32)> {
    let content = read_to_string(INPUT_PATH).unwrap();
    content.split("\n").filter_map(parse_line).collect()
}
