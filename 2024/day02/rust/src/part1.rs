use crate::common::parse_input;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Increasing,
    Decreasing,
    Invalid,
}

fn get_state((a, b): (&u8, &u8)) -> State {
    let diff = a.abs_diff(*b);
    if diff > 0 && diff <= 3 {
        if a > b {
            State::Decreasing
        } else {
            State::Increasing
        }
    } else {
        State::Invalid
    }
}

fn is_valid_report(rep: &[u8]) -> bool {
    let rep: Vec<_> = rep.iter().zip(&rep[1..]).map(get_state).collect();
    let all_increasing = rep.iter().all(|x| *x == State::Increasing);
    let all_decreasing = rep.iter().all(|x| *x == State::Decreasing);
    all_increasing || all_decreasing
}

pub fn parse_and_solve() -> usize {
    let data = parse_input();
    solve(data)
}

fn solve(data: Vec<Vec<u8>>) -> usize {
    data.into_iter()
        .filter(|x| is_valid_report(x))
        .collect::<Vec<_>>()
        .len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_valid_report(&report));

        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_valid_report(&report));

        let report = vec![1, 3, 2, 4, 5];
        assert!(!is_valid_report(&report));

        let report = vec![8, 6, 4, 4, 1];
        assert!(!is_valid_report(&report));

        let report = vec![1, 3, 6, 7, 9];
        assert!(is_valid_report(&report));
    }
}
