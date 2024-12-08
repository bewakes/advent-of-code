use crate::common::parse_input;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Increasing,
    Decreasing,
    Invalid,
}

fn get_state((a, b): (&u8, &u8)) -> State {
    match a.abs_diff(*b) {
        1..=3 if a > b => State::Decreasing,
        1..=3 => State::Increasing,
        _ => State::Invalid,
    }
}

fn is_valid_sub_report(sub_rep: &[u8]) -> bool {
    let states: Vec<_> = sub_rep.iter().zip(&sub_rep[1..]).map(get_state).collect();
    let all_increasing = states.iter().all(|x| *x == State::Increasing);
    let all_decreasing = states.iter().all(|x| *x == State::Decreasing);
    all_increasing || all_decreasing
}

fn is_valid_report(rep: &[u8]) -> bool {
    if is_valid_sub_report(rep) {
        return true;
    }
    for i in 0..rep.len() {
        let (left, right) = rep.split_at(i);
        if is_valid_sub_report(&[left, &right[1..]].concat()) {
            return true;
        }
    }
    false
}

pub fn parse_and_solve() -> usize {
    let data = parse_input();
    solve(data)
}

fn solve(data: Vec<Vec<u8>>) -> usize {
    data.into_iter().filter(|x| is_valid_report(x)).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        println!("Testing 1");
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_valid_report(&report));

        println!("Testing 2");
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_valid_report(&report));

        println!("Testing 3");
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_valid_report(&report));

        println!("Testing 4");
        let report = vec![1, 3, 2, 4, 5];
        assert!(is_valid_report(&report));

        println!("Testing 5");
        let report = vec![8, 6, 4, 4, 1];
        assert!(is_valid_report(&report));

        println!("Testing 6");
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_valid_report(&report));
    }
}
