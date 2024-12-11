use std::collections::HashSet;

use crate::common::{parse_input, Input, Point};

pub fn solve() -> u64 {
    let input = parse_input();
    solve_inner(input)
}

fn solve_inner(input: Input) -> u64 {
    let mut memoized_map: Vec<Vec<Option<HashSet<Point>>>> =
        vec![vec![None; input.width]; input.height];
    let mut total_score = 0;
    for j in 0..input.height {
        for i in 0..input.width {
            if input.array[j][i] == 0 {
                // Recursively calls and memoizes the path
                total_score += get_path_ends(0, i, j, &input, &mut memoized_map).len();
            }
        }
    }
    total_score as u64
}

fn get_path_ends(
    start: u8,
    i: usize,
    j: usize,
    input: &Input,
    memoized_map: &mut [Vec<Option<HashSet<Point>>>],
) -> HashSet<Point> {
    match &memoized_map[j][i] {
        Some(v) => v.clone(),
        None => {
            if start == 9 {
                let entries = HashSet::from_iter([Point {
                    x: i as i32,
                    y: j as i32,
                }]);
                memoized_map[j][i] = Some(entries.clone());
                entries
            } else {
                let p = Point {
                    x: i as i32,
                    y: j as i32,
                };
                let mut trail_ends = HashSet::new();
                for neighbor in p.valid_neighbors(input) {
                    let x = neighbor.x as usize;
                    let y = neighbor.y as usize;
                    if input.array[y][x] == start + 1 {
                        for e in get_path_ends(start + 1, x, y, input, memoized_map) {
                            trail_ends.insert(e);
                        }
                    }
                }
                memoized_map[j][i] = Some(trail_ends.clone());
                trail_ends
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::parse_str;

    use super::*;

    #[test]
    fn test() {
        let raw = r###"89010123
                       78121874
                       87430965
                       96549874
                       45678903
                       32019012
                       01329801
                       10456732"###;
        let input = parse_str(raw);
        assert_eq!(solve_inner(input), 36);
    }
}
