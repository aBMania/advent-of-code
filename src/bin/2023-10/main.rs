use std::collections::HashMap;
use std::time::Instant;
use itertools::Itertools;

use tailcall::tailcall;

use advent_of_code::{CustomGrid, expand_grid, input_to_grid, Neighbors, NeighborsIterator, print_grid, read_input, should_submit, submit};

const DAY: u8 = 10;
const YEAR: u16 = 2023;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}


fn next_direction(c: char, from: Direction) -> Direction {
    match (c, from) {
        ('|', d @ _) => d,
        ('-', d @ _) => d,
        ('L', Direction::Down) => Direction::Right,
        ('L', Direction::Left) => Direction::Up,
        ('J', Direction::Down) => Direction::Left,
        ('J', Direction::Right) => Direction::Up,
        ('7', Direction::Right) => Direction::Down,
        ('7', Direction::Up) => Direction::Left,
        ('F', Direction::Left) => Direction::Down,
        ('F', Direction::Up) => Direction::Right,
        (_, _) => unreachable!("should not come from this direction/char")
    }
}

fn first_step(grid: &CustomGrid<char>, start_row: usize, start_col: usize) -> (usize, usize, char, Direction) {
    let (((next_row, next_col), &next_char), direction) = grid.up(start_row, start_col)
        .filter(|(_, &c)| c == '|' || c == '7' || c == 'F')
        .map(|v| (v, Direction::Up))
        .unwrap_or_else(|| grid.right(start_row, start_col)
            .filter(|(_, &c)| c == '-' || c == '7' || c == 'J')
            .map(|v| (v, Direction::Right))
            .unwrap_or_else(|| grid.down(start_row, start_col)
                .filter(|(_, &c)| c == '|' || c == 'L' || c == 'J')
                .map(|v| (v, Direction::Down))
                .expect("Could not find a pipe near start"))
        );

    (next_row, next_col, next_char, direction)
}

fn step(grid: &CustomGrid<char>, next_row: usize, next_col: usize, next_char: char, direction: Direction) -> (usize, usize, char, Direction) {
    let direction = next_direction(next_char, direction);
    let ((next_row, next_col), &next_char) = match direction {
        Direction::Up => grid.up(next_row, next_col),
        Direction::Down => grid.down(next_row, next_col),
        Direction::Right => grid.right(next_row, next_col),
        Direction::Left => grid.left(next_row, next_col)
    }.expect("broken path");

    (next_row, next_col, next_char, direction)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<char> = input_to_grid(input).unwrap();
    let ((start_row, start_col), _) = grid.indexed_iter().find(|(_, &c)| c == 'S').expect("no S in grid");

    let (mut next_row, mut next_col, mut next_char, mut direction) = first_step(&grid, start_row, start_col);

    let mut path_len = 1;

    while next_row != start_row || next_col != start_col {
        (next_row, next_col, next_char, direction) = step(&grid, next_row, next_col, next_char, direction);
        path_len += 1;
    }

    Some(path_len / 2)
}

fn expand_grid_with(grid: &mut CustomGrid<char>, c: char) {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if let Some(cell) = grid.get(row, col) {
                match cell {
                    '|' => {
                        grid.up_mut(row, col).map(|(_, up)| *up = c);
                        grid.down_mut(row, col).map(|(_, down)| *down = c);
                    }
                    '-' => {
                        grid.right_mut(row, col).map(|(_, right)| *right = c);
                        grid.left_mut(row, col).map(|(_, left)| *left = c);
                    }
                    'L' => {
                        grid.right_mut(row, col).map(|(_, right)| *right = c);
                        grid.up_mut(row, col).map(|(_, up)| *up = c);
                    }
                    'J' => {
                        grid.up_mut(row, col).map(|(_, up)| *up = c);
                        grid.left_mut(row, col).map(|(_, left)| *left = c);
                    }
                    '7' => {
                        grid.left_mut(row, col).map(|(_, left)| *left = c);
                        grid.down_mut(row, col).map(|(_, down)| *down = c);
                    }
                    'F' => {
                        grid.right_mut(row, col).map(|(_, right)| *right = c);
                        grid.down_mut(row, col).map(|(_, down)| *down = c);
                    }
                    _ => {}
                };
            }
        }
    }
}

fn fill(grid: &mut CustomGrid<char>, row: usize, col: usize) {
    #[tailcall]
    fn fill_inner(grid: &mut CustomGrid<char>, neighbors: Vec<(usize, usize)>) {
        if neighbors.len() == 0 {
            return;
        }
        for (row, col) in neighbors.iter() {
            *grid.get_mut(*row, *col).unwrap() = 'O';
        }

        let next_neighbors: Vec<(usize, usize)> = neighbors
            .into_iter()
            .flat_map(
                |(row, col)| {
                    grid
                        .iter_neighbors(row, col)
                        .filter(|((row, col), _)| *grid.get(*row, *col).unwrap() == '.')
                        .map(|(pos, _)| pos)
                        .collect::<Vec<_>>()
                }
            )
            .dedup()
            .collect();

        fill_inner(grid, next_neighbors)
    }

    fill_inner(grid, vec![(row, col)])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();
    let ((start_row, start_col), _) = grid.indexed_iter().find(|(_, &c)| c == 'S').expect("no S in grid");

    let (mut next_row, mut next_col, mut next_char, mut direction) = first_step(&grid, start_row, start_col);


    let mut path = HashMap::from([
        ((start_row, start_col), true)
    ]);

    while next_row != start_row || next_col != start_col {
        path.insert((next_row, next_col), true);
        (next_row, next_col, next_char, direction) = step(&grid, next_row, next_col, next_char, direction);
    }

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if !path.contains_key(&(row, col)) {
                *grid.get_mut(row, col).unwrap() = '.'
            }
        }
    }

    let mut expanded_grid = expand_grid(&grid, '.');
    expand_grid_with(&mut expanded_grid, 'x');

    fill(&mut expanded_grid, start_row * 2 + 1 + 1, start_col * 2 + 1 - 1);


    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if *expanded_grid.get(row * 2 + 1, col * 2 + 1).unwrap() == 'O' {
                *grid.get_mut(row, col).unwrap() = '0'
            }
        }
    }

    // print_grid(&grid);
    // println!();

    Some(
        grid
            .indexed_iter()
            .filter(|((row, col), _)| *expanded_grid.get(*row * 2 + 1, *col * 2 + 1).unwrap() == 'O')
            .count() as u32
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

    use super::{DAY, part_one, part_two, YEAR};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(80));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(10));
    }
}
