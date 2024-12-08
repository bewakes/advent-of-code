use std::fs::read_to_string;

use regex::Regex;

const INPUT_PATH: &str = "data/input";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_input(inp: &str) -> Vec<Action> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}), *([0-9]{1,3})\)").unwrap();
    re.find_iter(inp)
        .map(|m| {
            let caps = m.as_str();
            if caps == "do()" {
                Action::Do
            } else if caps == "don't()" {
                Action::Dont
            } else {
                let raw = caps.replace("mul(", "").replace(")", "");
                let items: Vec<_> = raw.split(",").collect();
                let [a, b] = items.as_slice() else {
                    panic!("invalid parse")
                };
                Action::Mul(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
            }
        })
        .collect()
}

fn run_actions(actions: Vec<Action>) -> u32 {
    let mut curr = Action::Do;
    let mut sum = 0;
    for act in actions {
        match act {
            Action::Mul(a, b) => {
                if curr == Action::Do {
                    sum += a * b
                }
            }
            _ => curr = act,
        }
    }
    sum
}

pub fn solve() -> u32 {
    let input = read_to_string(INPUT_PATH).unwrap();
    let data: Vec<Action> = parse_input(&input);
    run_actions(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let actions = parse_input(string);
        let sm = run_actions(actions);
        assert_eq!(sm, 48);
    }
}
