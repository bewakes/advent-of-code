use crate::common::parse_input;

const XMAS: &[char] = &['X', 'M', 'A', 'S'];
const REV: &[char] = &['S', 'A', 'M', 'X'];

pub fn solve() -> u32 {
    let data = parse_input();
    solve_inner(&data)
}

fn check(a: [char; 4]) -> u32 {
    if a == XMAS || a == REV {
        1
    } else {
        0
    }
}

fn solve_inner(d: &[Vec<char>]) -> u32 {
    let mut count = 0;
    let xlen = d[0].len();
    let ylen = d.len();
    for y in 0..d.len() {
        for x in 0..d[0].len() {
            if x <= xlen - 4 && y <= ylen - 4 {
                count += check([d[y][x], d[y + 1][x + 1], d[y + 2][x + 2], d[y + 3][x + 3]]);
            }
            if x <= xlen - 4 && y >= 3 {
                count += check([d[y][x], d[y - 1][x + 1], d[y - 2][x + 2], d[y - 3][x + 3]]);
            }
            if x <= xlen - 4 {
                count += check([d[y][x], d[y][x + 1], d[y][x + 2], d[y][x + 3]]);
            }
            if y <= ylen - 4 {
                count += check([d[y][x], d[y + 1][x], d[y + 2][x], d[y + 3][x]]);
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let arr = &[
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        assert_eq!(solve_inner(arr), 18);
    }
}
