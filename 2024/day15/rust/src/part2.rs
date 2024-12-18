use std::{
    io::{stdin, Empty},
    thread::sleep,
    time::Duration,
};

use crate::common::{parse_input, parse_str, Dir, Input, Item, Item2, Map, Vector};

const DEBUG: bool = false;

pub fn solve() -> u64 {
    let inp = parse_input();
    let mut input = resize_input(inp);
    if DEBUG {
        return run_test();
    }
    solve_inner(&mut input)
}

fn resize_input(input: Input) -> Input<Item2> {
    let mut arr = Vec::new();
    for row in input.map.arr {
        let mut newrow = Vec::new();
        for item in row {
            match item {
                Item::Wall => {
                    newrow.push(Item2::Wall);
                    newrow.push(Item2::Wall);
                }
                Item::Box => {
                    newrow.push(Item2::BoxL);
                    newrow.push(Item2::BoxR);
                }
                Item::Empty => {
                    newrow.push(Item2::Empty);
                    newrow.push(Item2::Empty);
                }
            }
        }
        arr.push(newrow);
    }
    let map = Map {
        arr,
        width: input.map.width * 2,
        height: input.map.height,
        curr_position: Vector::new(input.map.curr_position.x * 2, input.map.curr_position.y),
    };
    Input {
        map,
        movements: input.movements,
    }
}

pub fn move_one_step(curr_pos: &Vector, dir: &Dir, map: &mut Map<Item2>) -> Vector {
    let new_pos = curr_pos.move_once(dir);
    // No need to check bounds as we'll be inside and won't move past bounds
    match map.at(&new_pos) {
        Item2::Empty => new_pos,
        Item2::Wall => {
            // Do nothing
            curr_pos.clone()
        }
        Item2::BoxL | Item2::BoxR => push_box(dir, curr_pos.clone(), map),
    }
}

fn solve_inner(input: &mut Input<Item2>) -> u64 {
    for (i, dir) in input.movements.iter().enumerate() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let curr = input.map.curr_position.clone();
        input.map.curr_position = move_one_step(&curr, dir, &mut input.map);
        if i == 16263 {
            input.map.show();
            println!("16263 next {:?}", input.movements[i + 1]);
        }
        if is_invalid(&input.map) {
            println!("Invalid at {}", i);
            input.map.show();
            break;
        }
        if DEBUG && i >= 310 {
            let mut s = String::new();
            input.map.show();
            println!("{i} Move {:?}", input.movements[i + 1]);
            stdin().read_line(&mut s);
        }
    }
    input.map.show();
    sum_boxes(&input.map)
}

fn sum_boxes(map: &Map<Item2>) -> u64 {
    let mut sum = 0;
    for (j, row) in map.arr.iter().enumerate() {
        for (i, item) in row.iter().enumerate() {
            if *item == Item2::BoxL {
                sum += 100 * j + i;
            }
        }
    }
    sum as u64
}

fn push_box(dir: &Dir, curr_pos: Vector, map: &mut Map<Item2>) -> Vector {
    if dir.is_vertical() {
        push_vertical(dir, curr_pos, map)
    } else {
        push_horizontal(dir, curr_pos, map)
    }
}

fn push_vertical(dir: &Dir, curr_pos: Vector, map: &mut Map<Item2>) -> Vector {
    let curr = curr_pos.move_once(dir);
    let mut curr_level = get_block_range(&curr, map);
    println!("curr_level {:?}", curr_level);
    map.show_level(&curr_level);
    let mut levels = vec![curr_level.clone()];
    loop {
        let new_level = get_next_level(dir, &curr_level, map);
        println!(
            "new level {:?}\n {:?}",
            new_level,
            new_level
                .clone()
                .map(|x| x.iter().map(|y| map.at(y)).collect::<Vec<_>>())
        );
        match new_level {
            None => return curr_pos,
            Some(v) if v.is_empty() => {
                // empty level found, break;
                break;
            }
            Some(v) => {
                levels.push(v.clone());
                curr_level = v;
            }
        }
    }
    // Now start pushing every_level in the direction, replacing original by empty
    for level in levels.iter().rev() {
        // println!("shifting");
        // map.show_level(level);
        map.shift_once_in_dir(dir, level);
    }
    curr_pos.move_once(dir)
}

fn get_block_range(curr: &Vector, map: &Map<Item2>) -> Vec<Vector> {
    let item = map.arr[curr.y as usize][curr.x as usize];
    match item {
        Item2::BoxL => vec![curr.clone(), curr.move_once(&Dir::Right)],
        Item2::BoxR => vec![curr.move_once(&Dir::Left), curr.clone()],
        _ => panic!("unpossible"),
    }
}

fn get_next_level(dir: &Dir, curr_level: &[Vector], map: &Map<Item2>) -> Option<Vec<Vector>> {
    let mut next_level: Vec<_> = curr_level.iter().map(|x| x.move_once(dir)).collect();
    println!("tmp next: {:?}", next_level);
    println!(
        "val next: {:?}",
        next_level.iter().map(|x| map.at(x)).collect::<Vec<_>>()
    );
    // If any is wall just return
    if next_level.iter().any(|x| map.at(x) == Item2::Wall) {
        return None;
    }
    drop_while(&mut next_level, |x| map.at(x) == Item2::Empty);
    // next_level.retain(|x| map.at(x) != Item2::Empty);
    // println!("nextlevel {:?} ", next_level);
    // map.show_level(&next_level);
    // if all empty, return empty vec
    if next_level.iter().all(|x| map.at(x) == Item2::Empty) {
        return Some(Vec::new());
    }
    // Now just need to check the edges: if leftmost is BoxL and rightmost is BoxR else add one
    // more each
    let last_idx = next_level.len() - 1;
    match (map.at(&next_level[0]), map.at(&next_level[last_idx])) {
        (Item2::BoxL, Item2::BoxR) => Some(next_level),
        (Item2::BoxR, Item2::BoxR) | (Item2::BoxR, Item2::Empty) => {
            next_level.insert(0, next_level[0].move_once(&Dir::Left));
            Some(next_level)
        }
        (Item2::BoxL, Item2::BoxL) | (Item2::Empty, Item2::BoxL) => {
            next_level.push(next_level[last_idx].move_once(&Dir::Right));
            Some(next_level)
        }
        (Item2::BoxR, Item2::BoxL) => {
            next_level.insert(0, next_level[0].move_once(&Dir::Left));
            next_level.push(next_level[last_idx + 1].move_once(&Dir::Right));
            Some(next_level)
        }
        _ => Some(next_level),
    }
}

fn drop_while(next_level: &mut Vec<Vector>, f: impl Fn(&Vector) -> bool) {
    // remove until condition from beginning and end
    while next_level.last().map(&f) == Some(true) {
        next_level.pop();
    }
    next_level.reverse();
    while next_level.last().map(&f) == Some(true) {
        next_level.pop();
    }
    next_level.reverse();
}

fn is_invalid(map: &Map<Item2>) -> bool {
    for j in 0..map.height {
        for i in 0..map.width - 1 {
            let p = Vector::new(i as i64, j as i64);
            if map.at(&p) == Item2::BoxL && map.at(&p.move_once(&Dir::Right)) != Item2::BoxR {
                println!("INvalid boxl {:?}", p);
                return true;
            }
            if map.at(&p) == Item2::BoxR && map.at(&p.move_once(&Dir::Left)) != Item2::BoxL {
                println!("INvalid boxr {:?}", p);
                return true;
            }
        }
    }
    false
}

fn push_horizontal(dir: &Dir, curr_pos: Vector, map: &mut Map<Item2>) -> Vector {
    let mut curr = curr_pos.move_once(dir);
    let mut count = 1;
    while map.at(&curr) == Item2::BoxL || map.at(&curr) == Item2::BoxR {
        curr = curr.move_once(dir);
        count += 1;
    }
    if map.at(&curr) == Item2::Wall {
        curr_pos
    } else {
        map.shift_n_items_by_one(count, dir, &curr_pos.move_once(dir));
        curr_pos.move_once(dir)
    }
}

fn run_test() -> u64 {
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
    let mut input = resize_input(parse_str(raw));
    &input.map.show();
    solve_inner(&mut input)
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
        let mut input = resize_input(parse_str(raw));
        input.map.show();
        let res = solve_inner(&mut input);
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
        let mut input = resize_input(parse_str(raw));
        input.map.show();
        let res = solve_inner(&mut input);
        // print_map(&input.map);
        assert_eq!(res, 9021);
    }
}
