
use crate::common::{get_invalid_pos, parse_input, Input};

fn solve_input(inp: Input) -> u32 {
    let mut sum = 0;
    for order in inp.orders {
        if get_invalid_pos(&order, &inp.relations).is_none() {
            let mid = order.len() / 2;
            let item = order[mid];
            sum += item;
        }
    }
    sum
}

pub fn solve() -> u32 {
    let input = parse_input();
    solve_input(input)
}

#[cfg(test)]
mod test {
    use crate::common::Input;

    use super::*;
    #[test]
    fn test_whole() {
        let rel = [
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let input: Input = Input {
            relations: rel.to_vec(),
            orders: vec![
                vec![75, 47, 61, 53, 29],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
                vec![75, 29, 13],
                vec![97, 61, 53, 29, 13],
            ],
        };
        assert_eq!(solve_input(input), 143);
    }
}
