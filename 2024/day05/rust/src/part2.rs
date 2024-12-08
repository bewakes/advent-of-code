use crate::common::{get_invalid_pos, parse_input, Input};

fn solve_input(inp: Input) -> u32 {
    let mut sum = 0;
    for mut order in inp.orders {
        println!("Original order: {:?}", order);
        if get_corrected_order(&mut order, &inp.relations).is_some() {
            println!("Corrected order: {:?}", order);
            let mid = order.len() / 2;
            let item = order[mid];
            sum += item;
        } else {
            println!("Already ordered");
        }
    }
    sum
}

/// Returns some if the order was invalid, else returns None if order was valid in the beginning
fn get_corrected_order(order: &mut [u32], rel: &Vec<(u32, u32)>) -> Option<bool> {
    if let Some((i, j)) = get_invalid_pos(order, rel) {
        let mut inv_i = i;
        let mut inv_j = j;
        loop {
            order.swap(inv_i, inv_j);
            if let Some((i, j)) = get_invalid_pos(order, rel) {
                inv_i = i;
                inv_j = j;
            } else {
                break;
            }
        }
        Some(true)
    } else {
        None
    }
}

pub fn solve() -> u32 {
    let input = parse_input();
    solve_input(input)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
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
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        };
        assert_eq!(solve_input(input), 123);
        assert!(false);
    }
}
