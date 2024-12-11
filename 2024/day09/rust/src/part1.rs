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
    let mut curr_blank_idx = 0;
    let mut curr_blank_capacity = input.blanks[curr_blank_idx] as i64;

    // Populate first write
    let mut final_writes: Vec<u64> = vec![0; input.writes[0] as usize];

    let mut write_front_idx = 1; // first is written start from second
    let mut write_back_idx = input.writes.len() - 1;
    let mut curr_back_length = input.writes[write_back_idx] as i64;
    let mut turn = 0; // 0 for blank, 1 for writes

    loop {
        if write_back_idx < write_front_idx || curr_blank_idx >= input.blanks.len() {
            break;
        }
        if turn == 0 {
            // write to blank from the back
            match curr_blank_capacity - curr_back_length {
                0 => {
                    // write all
                    write_n_items_to(
                        &mut final_writes,
                        curr_blank_capacity as u64,
                        write_back_idx,
                    );
                    write_back_idx -= 1;
                    curr_blank_idx += 1;
                    curr_back_length = input.writes[write_back_idx] as i64;
                    curr_blank_capacity = input.blanks[curr_blank_idx] as i64;
                    // now turn to write items from front in order
                    turn = 1;
                }
                n if n > 0 => {
                    // if more blanks than items
                    write_n_items_to(&mut final_writes, curr_back_length as u64, write_back_idx);
                    write_back_idx -= 1;
                    curr_back_length = input.writes[write_back_idx] as i64;
                    curr_blank_capacity = n;
                }
                n if n < 0 => {
                    // if more items than blanks
                    write_n_items_to(
                        &mut final_writes,
                        curr_blank_capacity as u64,
                        write_back_idx,
                    );
                    curr_back_length = -n;
                    curr_blank_idx += 1;
                    curr_blank_capacity = input.blanks[curr_blank_idx] as i64;
                    turn = 1;
                }
                _ => {
                    println!("unexpected");
                }
            }
        } else {
            // write in order from front
            let write_count = if write_back_idx == write_front_idx {
                curr_back_length as u64
            } else {
                input.writes[write_front_idx]
            };
            write_n_items_to(&mut final_writes, write_count, write_front_idx);
            write_front_idx += 1;
            turn = 0;
        }
    }
    final_writes
}

fn write_n_items_to(final_writes: &mut Vec<u64>, n: u64, item: usize) {
    for _ in 0..n {
        final_writes.push(item as u64);
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
        assert_eq!(solve_inner(input), 1928);
    }
}
