use crate::common::{parse_input, Entry, Input, Op};

fn can_be_valid(entry: &Entry) -> bool {
    let num_ops = entry.items.len() as u64 - 1;
    for i in 0..exp(3, num_ops) {
        let ops = Op::ops_from_u64_base(3, i, num_ops);
        let eval = ops
            .iter()
            .zip(entry.items[1..].iter())
            .fold(entry.items[0], |acc, (op, el)| op.apply(acc, *el));
        if eval == entry.value {
            return true;
        }
    }
    false
}

fn exp(arg: u64, num_ops: u64) -> u64 {
    let mut r = 1;
    for _ in 0..num_ops {
        r *= arg;
    }
    r
}

fn solve_inner(input: Input) -> u64 {
    input
        .iter()
        .filter_map(|x| if can_be_valid(x) { Some(x.value) } else { None })
        .sum()
}

pub fn solve() -> u64 {
    let input = parse_input();
    solve_inner(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let entries = vec![
            Entry {
                value: 190,
                items: vec![10, 19],
            },
            Entry {
                value: 3267,
                items: vec![81, 40, 27],
            },
            Entry {
                value: 83,
                items: vec![17, 5],
            },
            Entry {
                value: 156,
                items: vec![15, 6],
            },
            Entry {
                value: 7290,
                items: vec![6, 8, 6, 15],
            },
            Entry {
                value: 161011,
                items: vec![16, 10, 13],
            },
            Entry {
                value: 192,
                items: vec![17, 8, 14],
            },
            Entry {
                value: 21037,
                items: vec![9, 7, 18, 13],
            },
            Entry {
                value: 292,
                items: vec![11, 6, 16, 20],
            },
        ];
        let expected = vec![true, true, false, true, true, false, true, false, true];
        for (ent, exp) in entries.iter().zip(expected) {
            assert_eq!(can_be_valid(ent), exp);
        }

        assert_eq!(solve_inner(entries), 11387);
    }
}
