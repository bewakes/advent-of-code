// Find the difference between two columns in the input

use crate::common::parse_input;

pub fn solve() -> u32 {
    let (mut a, mut b): (Vec<_>, Vec<_>) = parse_input().into_iter().unzip();
    a.sort();
    b.sort();
    let res: u32 = a.iter().zip(b).map(|(x, y)| x.abs_diff(y)).sum();
    res
}
