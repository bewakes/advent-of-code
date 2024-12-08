use std::{collections::HashMap, fs::read_to_string};

const INPUT_PATH: &str = "data/input";

#[derive(Clone, Debug)]
pub struct Input {
    pub relations: Vec<(u32, u32)>,
    pub orders: Vec<Vec<u32>>,
}

pub fn parse_input() -> Input {
    let inp = read_to_string(INPUT_PATH).unwrap();
    let splitted = inp.split_terminator("\n");

    let mut relations_section = true;
    let mut relations: Vec<(u32, u32)> = Vec::new();
    let mut expected_orders: Vec<Vec<u32>> = Vec::new();

    for line in splitted {
        if line.trim() == "" {
            relations_section = false;
            continue;
        }
        if relations_section {
            let (a, b) = line.trim().split_once("|").unwrap();
            relations.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
        } else {
            expected_orders.push(line.split(",").map(|x| x.parse::<u32>().unwrap()).collect());
        }
    }
    Input {
        relations,
        orders: expected_orders,
    }
}

/// Returns the first encountered invalid ordering positions, if not returns None
pub fn get_invalid_pos(order: &[u32], relations: &Vec<(u32, u32)>) -> Option<(usize, usize)> {
    let map: HashMap<u32, usize> =
        HashMap::from_iter(order.iter().enumerate().map(|(i, &x)| (x, i)));
    for (k, v) in relations {
        let kidx = map.get(k);
        let vidx = map.get(v);
        match (kidx, vidx) {
            (Some(kidx), Some(vidx)) if kidx > vidx => {
                return Some((*kidx, *vidx));
            }
            _ => {}
        };
    }
    None
}
