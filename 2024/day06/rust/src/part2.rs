use std::collections::HashSet;

use crate::common::{parse_input, Dir, DirectedPos, Input, Pos};

fn has_loop(input: &Input, start: &Pos) -> bool {
    let mut curr = start.clone();
    let mut curr_direction = Dir::Up;
    let mut directed_positions = HashSet::new();

    directed_positions.insert(DirectedPos {
        dir: curr_direction.clone(),
        pos: curr.clone(),
    });
    loop {
        let (stop_at, _) = input.move_until_obstacle(&curr, &curr_direction);

        // Check if moving forward is out of bound
        if !input.in_bound(&stop_at.clone().step(&curr_direction)) {
            break;
        }

        curr_direction = curr_direction.right();

        curr = stop_at;

        let dirpos = DirectedPos {
            pos: curr.clone(),
            dir: curr_direction.clone(),
        };

        if directed_positions.contains(&dirpos) {
            return true;
        } else {
            directed_positions.insert(dirpos);
        }
    }
    false
}

fn solve_inner(input: Input) -> u32 {
    let start = input.start_position();
    let mut possible_positions = 0;
    for j in 0..input.height {
        for i in 0..input.width {
            if input.arr[j as usize][i as usize] != '#' && input.arr[j as usize][i as usize] != '^'
            {
                let mut new = input.clone();
                // insert obstacle
                new.arr[j as usize][i as usize] = '#';
                if has_loop(&new, &start) {
                    possible_positions += 1;
                }
            }
        }
    }
    possible_positions
}

pub fn solve() -> u32 {
    let input = parse_input();
    solve_inner(input)
}

#[cfg(test)]
mod test {
    use crate::common::Input;

    use super::*;

    #[test]
    fn test() {
        let arr = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];
        let inp = Input {
            arr,
            width: 10,
            height: 10,
        };

        assert_eq!(solve_inner(inp), 6);
    }
}
