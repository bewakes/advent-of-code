use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

fn parse_line(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .filter_map(|e| e.parse::<u8>().ok())
        .collect()
}

pub fn parse_input() -> Vec<Vec<u8>> {
    let content = read_to_string(INPUT_PATH).unwrap();
    content.trim().split("\n").map(parse_line).collect()
}
