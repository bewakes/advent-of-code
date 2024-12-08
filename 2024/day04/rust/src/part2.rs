use crate::common::parse_input;

pub fn solve() -> u32 {
    let data = parse_input();
    solve_inner(&data)
}

fn solve_inner(data: &[Vec<char>]) -> u32 {
    let mut count = 0;
    // Look for center A and check diagonal
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            // no need to check at edges
            if x == 0 || y == 0 || x >= data[0].len() - 1 || y >= data.len() - 1 {
                continue;
            }
            if data[y][x] != 'A' {
                continue;
            }
            let d1_items = (data[y - 1][x - 1], data[y + 1][x + 1]);
            let d2_items = (data[y + 1][x - 1], data[y - 1][x + 1]);

            let d1_valid = (d1_items.0 == 'M' && d1_items.1 == 'S')
                || (d1_items.0 == 'S' && d1_items.1 == 'M');

            let d2_valid = (d2_items.0 == 'M' && d2_items.1 == 'S')
                || (d2_items.0 == 'S' && d2_items.1 == 'M');

            if d1_valid && d2_valid {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_at() {
        let arr = &[
            vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.', '.'],
            vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.', 'S'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.', '.'],
            vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.', 'M'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let res = solve_inner(arr);
        assert_eq!(res, 9);
    }
}
