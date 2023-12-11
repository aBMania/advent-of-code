use std::collections::BTreeMap;
use std::iter::once;
use itertools::Itertools;

use advent_of_code::{CustomGrid, input_to_grid, read_input, should_submit, submit};

const DAY: u8 = 3;
const YEAR: u16 = 2023;

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid::<char>(input).unwrap();

    let mut iterator = grid.indexed_iter().peekable();
    let mut sum = 0;

    while let Some(((row, col), value)) = iterator.next() {
        match *value {
            _ if value.is_ascii_digit() => {}
            _ => continue
        }

        let group: Vec<_> =
            once(((row, col), value)).chain(
                iterator
                    .peeking_take_while(|(_, value)| value.is_ascii_digit())
            )
                .collect();

        let symbol = group
            .iter()
            .filter_map(|&((row, col), _)| {
                grid
                    .iter_diagonal_neighbors(row, col)
                    .filter(|(_, &neighbor_value)| neighbor_value != '.' && !neighbor_value.is_ascii_digit())
                    .map(|(pos, _)| pos)
                    .next()
            })
            .next();

        if let Some(_) = symbol {
            let group_sum: u32 = group
                .into_iter()
                .map(|(_, value)| value)
                .collect::<String>()
                .parse()
                .unwrap();

            sum += group_sum;
        }
    }

    Some(
        sum)
}


pub fn part_two(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid::<char>(input).unwrap();

    let mut iterator = grid.indexed_iter().peekable();
    let mut gears: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();

    while let Some(((row, col), value)) = iterator.next() {
        match *value {
            _ if value.is_ascii_digit() => {}
            _ => continue
        }

        let group: Vec<_> =
            once(((row, col), value)).chain(
                iterator
                    .peeking_take_while(|(_, value)| value.is_ascii_digit())
            )
                .collect();

        let gear = group
            .iter()
            .filter_map(|&((row, col), _)| {
                grid
                    .iter_diagonal_neighbors(row, col)
                    .filter(|(_, &neighbor_value)| neighbor_value == '*')
                    .map(|(pos, _)| pos)
                    .next()
            })
            .next();

        if let Some(gear) = gear {
            let group_sum: u32 = group
                .into_iter()
                .map(|(_, value)| value)
                .collect::<String>()
                .parse()
                .unwrap();

            gears.entry(gear).or_insert(vec![]).push(group_sum);
        }
    }

    Some(
        gears
            .iter()
            .filter_map(|(_, sums)| {
                match sums.len() {
                    2 => Some(sums.get(0).unwrap() * sums.get(1).unwrap()),
                    _ => None
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
        assert_eq!(part_one(&example), Some(4361));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(467835));
    }
}
