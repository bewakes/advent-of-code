use crate::common::{parse_input, Input, Machine};

const TOKEN_A: i64 = 3;
const TOKEN_B: i64 = 1;

const ERROR: i64 = 10000000000000;

pub fn solve_machine(m: Machine) -> i64 {
    let [a, b] = [m.button_a.x, m.button_b.x];
    let [c, d] = [m.button_a.y, m.button_b.y];
    let [px, py] = [m.prize.x + ERROR, m.prize.y + ERROR];
    // Conditions for no solution
    if a * d == b * c {
        return 0; // correct?
    }

    let det = a * d - b * c;
    // Matrix Mul
    let sx = d * px - b * py;
    let sy = a * py - c * px;

    if sx % det != 0 || sy % det != 0 {
        return 0;
    }
    let a_tokens = sx / det;
    let b_tokens = sy / det;
    if a_tokens < 0 || b_tokens < 0 {
        return 0;
    }
    TOKEN_A * a_tokens + TOKEN_B * b_tokens
}

pub fn solve() -> i64 {
    let input = parse_input();
    solve_inner(input)
}

fn solve_inner(input: Input) -> i64 {
    input.into_iter().map(solve_machine).sum()
}

#[cfg(test)]
mod test {
    use crate::common::parse_machine;

    use super::*;

    // NOTE: THE FOLLOWING ARE wrong as the second part does not have answers for examples
    #[test]
    fn test1() {
        let str = r###"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
            "###;
        let m = parse_machine(str);
        println!("Machine {:?}", m);
        assert_eq!(solve_machine(m), 0);
    }

    #[test]
    fn test2() {
        let str = r###"
            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176
            "###;
        let m = parse_machine(str);
        println!("Machine {:?}", m);
        assert_eq!(solve_machine(m), 0);
    }

    #[test]
    fn test3() {
        let str = r###"
            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450
            "###;
        let m = parse_machine(str);
        println!("Machine {:?}", m);
        assert_eq!(solve_machine(m), 200);
    }

    #[test]
    fn test4() {
        let str = r###"
            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
            "###;
        let m = parse_machine(str);
        println!("Machine {:?}", m);
        assert_eq!(solve_machine(m), 0);
    }
}
