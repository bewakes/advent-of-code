use std::collections::HashSet;

use crate::common::{parse_input, Dir, DirectedPos, Input};

fn solve_inner(input: Input) -> u32 {
    let mut curr = input.start_position();
    let mut curr_direction = Dir::Up;
    let mut all_visited = HashSet::new();
    let mut directed_positions = HashSet::new();
    directed_positions.insert(DirectedPos {
        dir: curr_direction.clone(),
        pos: curr.clone(),
    });
    loop {
        println!("curr: {:?}", curr);
        let (stop_at, visited) = input.move_until_obstacle(&curr, &curr_direction);
        println!("visited {:?}", visited);

        for p in visited {
            all_visited.insert(p);
        }

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
            println!("Exiting because of loop");
            break;
        } else {
            directed_positions.insert(dirpos);
        }
    }
    all_visited.len() as u32
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

        assert_eq!(solve_inner(inp), 41);
    }
}
