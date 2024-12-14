use crate::common::{parse_input, Input, Location, Vector};

/*
 * . . .
 * . . .
 * (1, 0)  v (1, -1)
 * after 5 steps (1+5, 0-5) = (6, -5) = 0, -1
 */
const STEPS: i64 = 100;
const W: u64 = 101;
const H: u64 = 103;

pub fn solve() -> i64 {
    let input = parse_input(W, H);
    solve_inner(input)
}

fn solve_inner(input: Input) -> i64 {
    let pos: Vec<_> = input
        .locations
        .iter()
        .map(|x| position_after_steps(x, STEPS, input.width, input.height))
        .collect();
    let half_w = (input.width / 2) as i64;
    let half_h = (input.height / 2) as i64;
    let q1 = pos.iter().filter(|p| p.x < half_w && p.y < half_h).count();
    let q2 = pos.iter().filter(|p| p.x > half_w && p.y < half_h).count();
    let q3 = pos.iter().filter(|p| p.x < half_w && p.y > half_h).count();
    let q4 = pos.iter().filter(|p| p.x > half_w && p.y > half_h).count();
    (q1 * q2 * q3 * q4) as i64
}

fn position_after_steps(l: &Location, steps: i64, w: u64, h: u64) -> Vector {
    let r = l.position.add(&l.velocity.scale(steps));
    r.rem(w as i64, h as i64)
}

#[cfg(test)]
mod test {
    use crate::common::parse_str;

    use super::*;

    #[test]
    fn test1() {
        let l = Location {
            position: Vector::new(2, 4),
            velocity: Vector::new(2, -3),
        };
        let steps = 5;
        let new = position_after_steps(&l, steps, 11, 7);
        assert_eq!(new.x, 1);
        assert_eq!(new.y, 3);
    }

    #[test]
    fn test() {
        let inp = r###"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
            "###;
        let inp = parse_str(inp, 11, 7);
        assert_eq!(solve_inner(inp), 12);
    }
}
