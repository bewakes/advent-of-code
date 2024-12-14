use std::{
    thread::sleep,
    time::{Duration},
};

use crate::common::{parse_input, Input, Location, Vector};

const STEPS: i64 = 100;
const W: u64 = 101;
const H: u64 = 103;

pub fn solve() -> i64 {
    let input = parse_input(W, H);
    let mut pos: Vec<_> = input
        .locations
        .clone()
        .into_iter()
        .map(|x| x.position)
        .collect();
    let mut time: i64 = 0;

    loop {
        pos = input
            .locations
            .iter()
            .map(|x| position_after_steps(x, time, input.width, input.height))
            .collect();
        let densities = calculate_block_densities(&pos);
        if densities.iter().filter(|&&x| x >=40).count() >= 1 { // This is a heuristic only, works
                                                              // for densities upto 75
            render(&pos, time);
            println!("TIME {}", time);
            break;
        }
        time += 1;
    }
    time
}

fn calculate_block_densities(pos:&[Vector]) -> [u64; 100] {
    // divide up in to 10x10
    let mut buckets = [0; 100]; // 100 buckets of 10x10(approx) ignoring the edges
    for p in pos {
        let buc_idx = 10* (p.y / 10) + (p.x / 10);
        buckets[buc_idx.min(99) as usize] += 1;
    }
    buckets
}

fn position_after_steps(l: &Location, steps: i64, w: u64, h: u64) -> Vector {
    let r = l.position.add(&l.velocity.scale(steps));
    r.rem(w as i64, h as i64)
}

fn render(pos: &Vec<Vector>, ts: i64) {
    let mut map: [[bool; W as usize]; H as usize] = [[false; W as usize]; H as usize];
    for p in pos {
        map[p.y as usize][p.x as usize] = true;
    }
    for j in 0..H {
        for i in 0..W {
            if map[j as usize][i as usize] {
                print!("o");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("time: {}", ts);
}
