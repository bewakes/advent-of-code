// Find similarity score by counting the occurences

use std::collections::HashMap;

use crate::common::parse_input;

fn get_counts(items: &[u32]) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for item in items {
        *map.entry(*item).or_insert(0) += 1;
    }
    map
}

pub fn solve() -> u32 {
    let (a, b): (Vec<_>, Vec<_>) = parse_input().into_iter().unzip();
    let bcounts = get_counts(&b);
    let res = a
        .iter()
        .filter_map(|e| bcounts.get(e).map(|cnt| e * cnt))
        .sum();
    res
}
