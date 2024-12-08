use std::fs::read_to_string;

const INPUT_PATH: &str = "data/input";

pub fn parse_input() -> Vec<Vec<char>> {
    let str = read_to_string(INPUT_PATH).expect("can't read input file");
    str.split_whitespace()
        .filter_map(|x| {
            if x.trim() != "" {
                Some(x.to_owned().chars().collect())
            } else {
                None
            }
        })
        .collect()
}
