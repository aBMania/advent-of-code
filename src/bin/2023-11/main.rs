use std::collections::HashMap;
use advent_of_code::{CustomGrid, input_to_grid, read_input, should_submit, submit};
use std::time::Instant;
use itertools::Itertools;

const DAY: u8 = 11;
const YEAR: u16 = 2023;

fn parse_input(input: &str, gap: usize) -> Vec<(usize, usize)> {
    let grid: CustomGrid<char> = input_to_grid(input).unwrap();

    // For each row and cols, how many empty row/col is there ahead of it.
    // It is to know how much it should shift any galaxy at this row/col
    let mut empty_rows: HashMap<usize, usize> = HashMap::new();
    let mut empty_cols: HashMap<usize, usize> = HashMap::new();

    let mut current_empty_rows = 0;

    for (i, mut row) in grid.iter_rows().enumerate() {
        if row.all(|&n| n == '.') {
            current_empty_rows += 1;
        }

        empty_rows.insert(i, current_empty_rows);
    }

    let mut current_empty_cols = 0;

    for (i, mut col) in grid.iter_cols().enumerate() {
        if col.all(|&n| n == '.') {
            current_empty_cols += 1;
        }

        empty_cols.insert(i, current_empty_cols);
    }

    grid
        .indexed_iter()
        .filter_map(|((row, col), c)| {
            match *c {
                '#' => Some(
                    (
                        row + (gap - 1) * empty_rows.get(&row).expect("no empty row entry in map"),
                        col + (gap - 1) * empty_cols.get(&col).expect("no empty col entry in map"),
                    )
                ),
                _ => None
            }
        })
        .collect()
}

fn distance((row, col): (usize, usize), (other_row, other_col): (usize, usize)) -> usize {
    row.abs_diff(other_row) + col.abs_diff(other_col)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input, 2)
            .iter()
            .combinations(2)
            .map(|chunk| {
                let (row, col) = chunk.get(0).unwrap();
                let (other_row, other_col) = chunk.get(1).unwrap();

                distance((*row, *col), (*other_row, *other_col)) as u32
            })
            .sum()
    )
}


pub fn part_two(input: &str) -> Option<u64> {
    let positions = parse_input(input, 1000000);
    Some(
        positions
            .iter()
            .combinations(2)
            .map(|chunk| {
                let (row, col) = chunk.get(0).unwrap();
                let (other_row, other_col) = chunk.get(1).unwrap();

                distance((*row, *col), (*other_row, *other_col)) as u64
            })
            .sum()
    )
}

fn main() {
    let input = read_input(DAY, YEAR);
    let now = Instant::now();
    let part_one_response = part_one(&input);
    let ellapsed = now.elapsed().as_micros();

    if let Some(part_one_response) = part_one_response {
        if should_submit() {
            println!("Submitting part 1 response ({ellapsed})μs: {part_one_response}");
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
            println!("Part 1 response ({ellapsed}μs): {part_one_response}")
        }
    } else {
        println!("No part 1 response");
    }

    let now = Instant::now();
    let part_two_response = part_two(&input);
    let ellapsed = now.elapsed().as_micros();
    if let Some(part_two_response) = part_two_response {
        if should_submit() {
            println!("Submitting part 2 response ({ellapsed}μs): {part_two_response}");
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
            println!("Part 2 response ({ellapsed}μs): {part_two_response}")
        }
    } else {
        println!("No part 2 response");
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::read_example;
    use super::{DAY, YEAR, part_one, part_two, distance};

    #[test]
    fn test_distance() {
        assert_eq!(distance((6, 1), (11, 5)), 9);
        assert_eq!(distance((0, 0), (0, 0)), 0);
        assert_eq!(distance((0, 0), (0, 1)), 1);
        assert_eq!(distance((0, 0), (1, 1)), 2);
    }

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(374));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(82000210));
    }
}
