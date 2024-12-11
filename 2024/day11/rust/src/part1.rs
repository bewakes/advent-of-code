use crate::common::parse_input;

fn blink_one(stone: u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        n if even_digits(n) => split(n),
        _ => vec![stone * 2024],
    }
}

fn split(n: u64) -> Vec<u64> {
    let str = format!("{}", n);
    let len = str.len();
    vec![
        str[0..len / 2].parse::<u64>().unwrap(),
        str[len / 2..].parse::<u64>().unwrap(),
    ]
}

fn even_digits(n: u64) -> bool {
    format!("{}", n).len() % 2 == 0
}

fn blink_all(inp: Vec<u64>) -> Vec<u64> {
    inp.into_iter().flat_map(blink_one).collect()
}

pub fn solve() -> u64 {
    let mut inp = parse_input();
    for _ in 0..25 {
        inp = blink_all(inp);
    }
    inp.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let inp = vec![0, 1, 10, 99, 999];
        assert_eq!(blink_all(inp), vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test1() {
        let mut inp = vec![125, 17];
        let exp = vec![
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3,
            2,
        ];
        for _ in 0..6 {
            inp = blink_all(inp);
        }
        assert_eq!(inp, exp);
    }
}
