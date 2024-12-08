use std::collections::HashSet;

use crate::common::{parse_input, Input, Position};

pub fn solve() -> u32 {
    let input = parse_input();
    solve_inner(input)
}

fn solve_inner(input: Input) -> u32 {
    let mut antinodes = HashSet::new();
    for (_, nodes) in input.antennas.iter() {
        for an in get_antinodes(nodes, &input) {
            antinodes.insert(an);
        }
    }
    antinodes.len() as u32
}

fn get_antinodes(nodes: &[Position], input: &Input) -> Vec<Position> {
    let mut antinodes = Vec::new();
    for i in 0..(nodes.len() - 1) {
        for j in i + 1..nodes.len() {
            let anodes = get_antinodes_for_pair(&nodes[i], &nodes[j], input);
            for an in anodes {
                antinodes.push(an);
            }
        }
    }
    antinodes
}

fn get_antinodes_for_pair(nodea: &Position, nodeb: &Position, input: &Input) -> Vec<Position> {
    let mut anodes = vec![];
    let diff = nodea.diff(nodeb);
    for i in 0..input.width {
        let a1 = nodea.add(&diff.scale(-i));
        let a2 = nodeb.add(&diff.scale(i));
        if input.is_in_bound(&a1) {
            anodes.push(a1);
        }
        if input.is_in_bound(&a2) {
            anodes.push(a2);
        }
    }
    anodes
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test() {
        let antennas = HashMap::from([
            (
                '0',
                vec![
                    Position::new(8, 1),
                    Position::new(5, 2),
                    Position::new(7, 3),
                    Position::new(4, 4),
                ],
            ),
            (
                'A',
                vec![
                    Position::new(6, 5),
                    Position::new(8, 8),
                    Position::new(9, 9),
                ],
            ),
        ]);
        let input = Input {
            antennas,
            width: 12,
            height: 12,
        };

        assert_eq!(solve_inner(input), 34);
    }
}
