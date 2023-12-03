use std::collections::BTreeMap;

use advent_of_code::{CustomGrid, input_to_grid, read_input, should_submit, submit, NeighborsDiagonalIterator};

const NEIGHBORS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

const DAY: u8 = 3;
const YEAR: u16 = 2023;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input_to_grid::<char>(input).unwrap();

    let mut is_block_counted = false;
    let mut block_sum = 0u32;
    let mut total_sum = 0u32;
    let mut last_seen_row = 0;

    for ((row, col), value) in grid.indexed_iter() {
        if last_seen_row != row || !value.is_ascii_digit() {
            if is_block_counted {
                total_sum += block_sum;
            }

            block_sum = 0;
            is_block_counted = false;
        }

        if !value.is_ascii_digit() {
            last_seen_row = row;
            continue;
        }

        if !is_block_counted {
            is_block_counted = grid.iter_diagonal_neighbors(row, col)
                .any(|(_, &neighbor_value)| {
                    neighbor_value != '.' && !neighbor_value.is_ascii_digit()
                });
        }

        if let Some(value_digit) = value.to_digit(10) {
            block_sum = block_sum * 10 + value_digit
        }
        last_seen_row = row
    }

    Some(total_sum)
}


pub fn part_two(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid::<char>(input).unwrap();

    let mut gear_map: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();
    let mut block_sum = 0u32;
    let mut last_seen_row = 0;
    let mut current_gear: Option<(usize, usize)> = None;

    for ((row, col), value) in grid.indexed_iter() {
        if last_seen_row != row || !value.is_ascii_digit() {
            if let Some(current_gear) = current_gear {
                gear_map.entry(current_gear).or_insert(vec![]).push(block_sum);
            }

            current_gear = None;
            block_sum = 0;
        }

        if !value.is_ascii_digit() {
            last_seen_row = row;
            continue;
        }


        if current_gear.is_none() {
            current_gear = grid
                .iter_diagonal_neighbors(row, col)
                .filter(|(_, &neighbor_value)| neighbor_value == '*')
                .map(|(pos, _)| pos)
                .next()
        }


        if let Some(value_digit) = value.to_digit(10) {
            block_sum = block_sum * 10 + value_digit
        }
        last_seen_row = row
    }

    Some(
        gear_map
            .into_iter()
            .map(|(_, blocks)| {
                if blocks.len() != 2 {
                    0
                } else {
                    blocks.get(0).unwrap() * blocks.get(1).unwrap()
                }
            })
            .sum()
    )
}

fn main() {
    let input = read_input(DAY, YEAR);
    let part_one_response = part_one(&input);

    if let Some(part_one_response) = part_one_response {
        if should_submit() {
            println!("Submitting part 1 response: {part_one_response}");
            match submit(DAY, YEAR, 1, &part_one_response.to_string()) {
                Ok(correct) => {
                    if correct {
                        println!("Part 1 valid");
                    } else {
                        eprintln!("Part 1 invalid");
                    }
                }
                Err(e) => {
                    eprintln!("Error while submitting part1: {e}");
                }
            }
        } else {
            println!("Part 1 response: {part_one_response}")
        }
    } else {
        println!("No part 1 response");
    }

    let part_two_response = part_two(&input);
    if let Some(part_two_response) = part_two_response {
        if should_submit() {
            println!("Submitting part 2 response: {part_two_response}");
            match submit(DAY, YEAR, 2, &part_two_response.to_string()) {
                Ok(correct) => {
                    if correct {
                        println!("Part 2 valid");
                    } else {
                        eprintln!("Part 2 invalid");
                    }
                }
                Err(e) => {
                    eprintln!("Error while submitting part 2: {e}");
                }
            }
        } else {
            println!("Part 2 response: {part_two_response}")
        }
    } else {
        println!("No part 2 response");
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::read_example;

    use super::{DAY, part_one, part_two, YEAR};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        // TODO: set example expected response
        assert_eq!(part_one(&example), Some(4361));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        // TODO: set example expected response
        assert_eq!(part_two(&example), Some(467835));
    }
}
