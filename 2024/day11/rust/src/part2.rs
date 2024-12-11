use std::collections::HashMap;

use crate::common::parse_input_as_hashmap;

fn blink_one(stone: u64) -> HashMap<u64, u64> {
    match stone {
        0 => HashMap::from_iter([(1, 1)]),
        n if even_digits(n) => split(n),
        _ => HashMap::from_iter([(stone * 2024, 1)]),
    }
}

fn split(n: u64) -> HashMap<u64, u64> {
    let str = format!("{}", n);
    let len = str.len();
    let a = str[0..len / 2].parse::<u64>().unwrap();
    let b = str[len / 2..].parse::<u64>().unwrap();
    if a == b {
        HashMap::from_iter([(a, 2)])
    } else {
        HashMap::from_iter([(a, 1), (b, 1)])
    }
}

fn even_digits(n: u64) -> bool {
    format!("{}", n).len() % 2 == 0
}

pub fn solve() -> u64 {
    let mut inp = parse_input_as_hashmap();
    for _ in 0..75 {
        let mut hm = HashMap::new();
        for (s, c) in inp {
            let res = blink_one(s);
            for (k, v) in res {
                hm.entry(k).and_modify(|x| *x += v * c).or_insert(v * c);
            }
        }
        inp = hm;
    }
    inp.values().sum()
}
