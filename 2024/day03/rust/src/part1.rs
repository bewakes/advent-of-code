use std::fs::read_to_string;

use regex::Regex;

const INPUT_PATH: &str = "data/input";

fn parse_input(inp: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\(([0-9]{1,3}), *([0-9]{1,3})\)").unwrap();
    re.captures_iter(inp)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect()
}

pub fn solve() -> u32 {
    let input = read_to_string(INPUT_PATH).unwrap();
    let data: Vec<(u32, u32)> = parse_input(&input);
    data.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_input(string), vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }
}
