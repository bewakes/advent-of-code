use crate::common::{parse_input, Dir, Input, Item, Map, Vector};

pub fn solve() -> u64 {
    let mut input = parse_input();
    solve_inner(&mut input)
}

pub fn move_one_step(curr_pos: &Vector, dir: &Dir, map: &mut Map) -> Vector {
    let new_pos = curr_pos.move_once(dir);
    // No need to check bounds as we'll be inside and won't move past bounds
    match map.at(&new_pos) {
        Item::Empty => new_pos,
        Item::Wall => {
            // do nothing
            curr_pos.clone()
        }
        Item::Box => push_box(dir, curr_pos.clone(), map),
    }
}

fn solve_inner(input: &mut Input) -> u64 {
    for dir in input.movements.iter() {
        let curr = input.map.curr_position.clone();
        input.map.curr_position = move_one_step(&curr, dir, &mut input.map);
    }
    sum_boxes(&input.map)
}

fn sum_boxes(map: &Map) -> u64 {
    let mut sum = 0;
    for (j, row) in map.arr.iter().enumerate() {
        for (i, item) in row.iter().enumerate() {
            if *item == Item::Box {
                sum += 100 * j + i;
            }
        }
    }
    sum as u64
}

fn push_box(dir: &Dir, curr_pos: Vector, map: &mut Map) -> Vector {
    let mut curr = curr_pos.move_once(dir);
    // this can be optimized by sending n items of same dir and see if there are n empties in the
    // end
    let mut count = 1;
    while map.at(&curr) == Item::Box {
        curr = curr.move_once(dir);
        count += 1;
    }
    if map.at(&curr) == Item::Wall {
        curr_pos
    } else {
        map.shift_n_items_by_one(count, dir, &curr_pos.move_once(dir));
        curr_pos.move_once(dir)
    }
}

fn print_map(map: &Map) {
    for (j, row) in map.arr.iter().enumerate() {
        for (i, item) in row.iter().enumerate() {
            if map.curr_position == Vector::new(i as i64, j as i64) {
                print!("@");
            } else {
                print!("{}", item.draw());
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use crate::common::parse_str;

    use super::*;

    #[test]
    fn test1() {
        let raw = r###"########
                       #..O.O.#
                       ##@.O..#
                       #...O..#
                       #.#.O..#
                       #...O..#
                       #......#
                       ########

                <^^>>>vv<v>>v<<"###;
        let mut input = parse_str(raw);
        let res = solve_inner(&mut input);
        print_map(&input.map);
        assert_eq!(res, 2028);
    }
    #[test]
    fn test2() {
        let raw = r###"##########
                       #..O..O.O#
                       #......O.#
                       #.OO..O.O#
                       #..O@..O.#
                       #O#..O...#
                       #O..O..O.#
                       #.OO.O.OO#
                       #....O...#
                       ##########

                       <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
                       vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
                       ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
                       <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
                       ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
                       ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
                       >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
                       <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
                       ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
                       v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"###;
        let mut input = parse_str(raw);
        let res = solve_inner(&mut input);
        print_map(&input.map);
        assert_eq!(res, 10092);
    }
}
