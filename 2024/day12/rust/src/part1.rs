use crate::common::{parse_input, Input, Point};

pub fn solve() -> u64 {
    let input = parse_input();
    solve_inner(input)
}

fn solve_inner(input: Input) -> u64 {
    let mut visited_map: Vec<Vec<bool>> = vec![vec![false; input.width]; input.height];
    let mut regions: Vec<(char, Vec<Point>)> = Vec::new();
    for j in 0..input.height {
        for i in 0..input.width {
            let char = input.array[j][i];
            let point = Point::new(i, j);
            let mut region_points = vec![point.clone()];
            if !visited_map[j][i] {
                let res = dfs(char, &point, &mut visited_map, &input);
                region_points.extend(res);
                regions.push((char, region_points));
            }
        }
    }
    // Now we have the regions
    let mut total_price = 0u64;
    for (k, v) in regions.iter() {
        let area = v.len() as u64;
        let perimeter: u64 = v.iter().map(|p| p.get_perimeter(*k, &input)).sum();
        total_price += area * perimeter;
    }
    total_price
}

fn dfs(char: char, p: &Point, visited_map: &mut [Vec<bool>], input: &Input) -> Vec<Point> {
    let mut result = Vec::new();
    // mark it as visited
    visited_map[p.y as usize][p.x as usize] = true;
    for n in p.valid_neighbors(input) {
        let y = n.y as usize;
        let x = n.x as usize;
        let items = match (visited_map[y][x], input.array[y][x]) {
            (true, _) => Default::default(),
            (_, c) if c != char => Default::default(),
            (false, _) => {
                result.push(n.clone());
                dfs(char, &n, visited_map, input)
            }
        };
        result.extend(items);
    }
    result
}

#[cfg(test)]
mod test {
    use crate::common::parse_str;

    use super::*;

    #[test]
    fn test() {
        let raw = r###"RRRRIICCFF
                       RRRRIICCCF
                       VVRRRCCFFF
                       VVRCCCJFFF
                       VVVVCJJCFE
                       VVIVCCJJEE
                       VVIIICJJEE
                       MIIIIIJJEE
                       MIIISIJEEE
                       MMMISSJEEE"###;
        let input = parse_str(raw);
        assert_eq!(solve_inner(input), 1930);
    }

    #[test]
    fn test_small() {
        let raw = r###"OOOOO
                       OXOXO
                       OOOOO
                       OXOXO
                       OOOOO"###;
        let input = parse_str(raw);
        assert_eq!(solve_inner(input), 772);
    }

    #[test]
    fn test_smaller() {
        let raw = r###"AAAA
                       BBCD
                       BBCC
                       EEEC"###;
        let input = parse_str(raw);
        assert_eq!(solve_inner(input), 140);
    }
}
