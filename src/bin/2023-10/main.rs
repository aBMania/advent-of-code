use std::collections::HashMap;
use std::time::Instant;

use advent_of_code::{
    CustomGrid, input_to_grid, read_input, should_submit, submit,
};

const DAY: u8 = 10;
const YEAR: u16 = 2023;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn step(
    grid: &CustomGrid<char>,
    next_row: usize,
    next_col: usize,
    direction: Direction,
) -> (usize, usize, Direction) {
    let direction = match (grid.get(next_row, next_col), direction) {
        (Some('│'), Direction::Down) => Direction::Down,
        (Some('│'), Direction::Up) => Direction::Up,
        (Some('─'), Direction::Right) => Direction::Right,
        (Some('─'), Direction::Left) => Direction::Left,
        (Some('└'), Direction::Down) => Direction::Right,
        (Some('└'), Direction::Left) => Direction::Up,
        (Some('┘'), Direction::Down) => Direction::Left,
        (Some('┘'), Direction::Right) => Direction::Up,
        (Some('┐'), Direction::Right) => Direction::Down,
        (Some('┐'), Direction::Up) => Direction::Left,
        (Some('┌'), Direction::Left) => Direction::Down,
        (Some('┌'), Direction::Up) => Direction::Right,
        _ => unreachable!("should not come from this direction/char"),
    };

    let ((next_row, next_col), _) = match direction {
        Direction::Up => grid.up_indexed(next_row, next_col).unwrap(),
        Direction::Down => grid.down_indexed(next_row, next_col).unwrap(),
        Direction::Right => grid.right_indexed(next_row, next_col).unwrap(),
        Direction::Left => grid.left_indexed(next_row, next_col).unwrap()
    };

    (next_row, next_col, direction)
}

fn start_point(grid: &CustomGrid<char>) -> (usize, usize) {
    grid
        .indexed_iter()
        .find(|(_, &c)| c == 'S')
        .map(|((start_row, start_col), _)| (start_row, start_col))
        .expect("no S in grid")
}

fn replace_with_box_char(grid: &mut CustomGrid<char>) {
    for c in grid.iter_mut() {
        match *c {
            'F' => *c = '┌',
            '7' => *c = '┐',
            '-' => *c = '─',
            '|' => *c = '│',
            'J' => *c = '┘',
            'L' => *c = '└',
            '.' => *c = '•',
            'S' => {}
            _ => panic!("invalid char")
        }
    }
}

fn replace_starting_point(grid: &mut CustomGrid<char>, start_row: usize, start_col: usize) -> Direction {
    // Replace starting point
    match (
        grid.left(start_row, start_col),
        grid.up(start_row, start_col),
        grid.right(start_row, start_col),
        grid.down(start_row, start_col),
    ) {
        (Some('─' | '┌' | '└'), Some('┌' | '│' | '┐'), _, _) => {
            *grid.get_mut(start_row, start_col).unwrap() = '┘';
            Direction::Down
        }
        (Some('─' | '┌' | '└'), _, Some('─' | '┘' | '┐'), _) => {
            *grid.get_mut(start_row, start_col).unwrap() = '─';
            Direction::Right
        }
        (Some('─' | '┌' | '└'), _, _, Some('│' | '┘' | '└')) => {
            *grid.get_mut(start_row, start_col).unwrap() = '┐';
            Direction::Up
        }
        (_, Some('┌' | '│' | '┐'), Some('─' | '┘' | '┐'), _) => {
            *grid.get_mut(start_row, start_col).unwrap() = '└';
            Direction::Down
        }
        (_, Some('┌' | '│' | '┐'), _, Some('│' | '┘' | '└')) => {
            *grid.get_mut(start_row, start_col).unwrap() = '│';
            Direction::Up
        }
        (_, _, Some('─' | '┘' | '┐'), Some('│' | '┘' | '└')) => {
            *grid.get_mut(start_row, start_col).unwrap() = '┌';
            Direction::Up
        }
        _ => panic!("invalid start")
    }
}

fn count_inside_space(grid: &CustomGrid<char>) -> u32 {
    #[derive(Debug, PartialEq)]
    enum State {
        Outside,
        Inside,
        BottomCorner(bool),
        TopCorner(bool),
    }
    let mut count = 0;

    // Knot theory magic
    for row in 0..grid.rows() {
        let mut state = State::Outside;
        for col in 0..grid.cols() {
            state = match (state, grid.get(row, col).unwrap()) {
                (State::Outside, '│') => State::Inside,
                (State::Inside, '│') => State::Outside,
                (State::Inside, ' ') => {
                    count += 1;
                    State::Inside
                }
                (State::Outside, '┌') => State::BottomCorner(false),
                (State::Outside, '└') => State::TopCorner(false),
                (State::Inside, '┌') => State::BottomCorner(true),
                (State::Inside, '└') => State::TopCorner(true),
                (State::BottomCorner(inside @ _), '─') => State::BottomCorner(inside),
                (State::BottomCorner(true), '┐') => State::Inside,
                (State::BottomCorner(true), '┘') => State::Outside,
                (State::BottomCorner(false), '┐') => State::Outside,
                (State::BottomCorner(false), '┘') => State::Inside,
                (State::TopCorner(inside @ _), '─') => State::TopCorner(inside),
                (State::TopCorner(true), '┐') => State::Outside,
                (State::TopCorner(true), '┘') => State::Inside,
                (State::TopCorner(false), '┐') => State::Inside,
                (State::TopCorner(false), '┘') => State::Outside,
                (state @ _, _) => state
            };
        }
    };
    count
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();

    let (start_row, start_col) = start_point(&grid);

    replace_with_box_char(&mut grid);
    let start_direction = replace_starting_point(&mut grid, start_row, start_col);

    let (mut next_row, mut next_col, mut direction) = (start_row, start_col, start_direction);

    let mut path_length = 0;
    loop {
        path_length += 1;
        (next_row, next_col, direction) = step(&grid, next_row, next_col, direction);

        if (next_row, next_col) == (start_row, start_col) {
            break;
        }
    }

    Some(path_length / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<char> = input_to_grid(input).unwrap();

    let (start_row, start_col) = start_point(&grid);

    replace_with_box_char(&mut grid);
    let start_direction = replace_starting_point(&mut grid, start_row, start_col);

    let mut path = HashMap::from([
        ((start_row, start_col), true)
    ]);
    let (mut next_row, mut next_col, mut direction) = (start_row, start_col, start_direction);

    loop {
        path.insert((next_row, next_col), true);
        (next_row, next_col, direction) = step(&grid, next_row, next_col, direction);

        if (next_row, next_col) == (start_row, start_col) {
            break;
        }
    }

    // Remove everything not in path
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if !path.contains_key(&(row, col)) {
                *grid.get_mut(row, col).unwrap() = ' ';
            }
        }
    }

    Some(
        count_inside_space(&grid)
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
