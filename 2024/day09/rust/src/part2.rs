use crate::common::{parse_input, Input};

fn solve_inner(input: Input) -> u64 {
    let final_writes = get_final_writes(input);
    final_writes
        .iter()
        .enumerate()
        .map(|(i, &x)| i as u64 * x)
        .sum()
}

fn get_final_writes(input: Input) -> Vec<u64> {
    // map holds which blank index the write moves to
    let mut blanks = input.blanks.clone();
    let mut final_writes =
        vec![0; input.writes.iter().sum::<u64>() as usize + blanks.iter().sum::<u64>() as usize];
    let mut blank_indices: Vec<_> = input
        .blanks
        .iter()
        .enumerate()
        .map(|(i, _)| {
            input.writes[0..i + 1].iter().sum::<u64>() + input.blanks[0..i].iter().sum::<u64>()
        })
        .collect();
    // println!("blank indices {:?}", blank_indices);
    // println!("final_writes {:?}", final_writes);

    for j in (0..input.writes.len()).rev() {
        // println!("checking for {}", j);
        let mut found = false;
        for b in 0..j {
            if blanks[b] >= input.writes[j] && blanks[b] > 0 {
                blanks[b] -= input.writes[j];

                write_n_items_at(
                    &mut final_writes,
                    input.writes[j] as u8,
                    blank_indices[b] as usize,
                    j,
                );
                blank_indices[b] += input.writes[j];

                found = true;
                break;
            }
        }
        if !found && j > 0 {
            // println!("Not found {}", j);
            let from: u64 =
                input.writes[0..j].iter().sum::<u64>() + input.blanks[0..j].iter().sum::<u64>();
            write_n_items_at(&mut final_writes, input.writes[j] as u8, from as usize, j);
        }
        // println!("blank indices {:?}", blank_indices);
        // println!("blanks {:?}", blanks);
        // println!("final_writes {:?}", final_writes);
    }
    final_writes
}

fn write_n_items_at(final_writes: &mut [u64], n: u8, at: usize, item: usize) {
    for i in 0..n {
        final_writes[i as usize + at] = item as u64
    }
}

pub fn solve() -> u64 {
    let input = parse_input();
    solve_inner(input)
}

#[cfg(test)]
mod test {
    use crate::common::parse_string;

    use super::*;

    #[test]
    fn test() {
        let raw = "2333133121414131402";
        let input = parse_string(raw);
        assert_eq!(solve_inner(input), 2858);
    }
}
