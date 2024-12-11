use std::collections::HashMap;

pub type Input = Vec<u64>;

pub fn parse_input() -> Input {
    vec![773, 79858, 0, 71, 213357, 2937, 1, 3998391]
}

pub fn parse_input_as_hashmap() -> HashMap<u64, u64> {
    HashMap::from_iter([
        (773, 1),
        (79858, 1),
        (0, 1),
        (71, 1),
        (213357, 1),
        (2937, 1),
        (1, 1),
        (3998391, 1),
    ])
}
