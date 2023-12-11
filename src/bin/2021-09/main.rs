use advent_of_code::{CustomGrid, input_to_grid, read_input, should_submit, submit};

const DAY: u8 = 9;
const YEAR: u16 = 2021;

const NEXT: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn lowest_points(grid: &CustomGrid<u8>) -> Vec<(usize, usize)> {
    let mut lowest_points: Vec<(usize, usize)> = vec![];

    for ((y, x), value) in grid.indexed_iter() {
        let left = if y > 0 { grid.get(y - 1, x) } else { None };
        let right = if y <= grid.rows() { grid.get(y + 1, x) } else { None };
        let up = if x > 0 { grid.get(y, x - 1) } else { None };
        let down = if x <= grid.cols() { grid.get(y, x + 1) } else { None };

        if value < up.unwrap_or(&u8::MAX)
            && value < down.unwrap_or(&u8::MAX)
            && value < left.unwrap_or(&u8::MAX)
            && value < right.unwrap_or(&u8::MAX)
        {
            lowest_points.push((x, y));
        }
    }

    lowest_points
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: CustomGrid<u8> = input_to_grid(input).unwrap();

    let lowest_points = lowest_points(&grid);

    Some(lowest_points.iter().fold(0, |acc, (x, y)| {
        acc + *grid.get(*y, *x).unwrap() as u32 + 1
    }))
}

fn basin(grid: &mut CustomGrid<u8>, x: usize, y: usize) -> usize {
    grid[y][x] = 9;

    NEXT
        .iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            if *grid.get(y, x).unwrap_or(&u8::MAX) >= 9 {
                acc
            } else {
                acc + basin(grid, x, y)
            }
        })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: CustomGrid<u8> = input_to_grid(input).unwrap();

    let lowest_points = lowest_points(&grid);

    let mut basin_sizes: Vec<usize> = lowest_points.into_iter().map(|(x, y)| {
        basin(&mut grid, x, y)
    }).collect();

    basin_sizes.sort();

    Some(basin_sizes.into_iter().rev().take(3).product::<usize>() as u32)
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
        assert_eq!(part_one(&example), Some(15));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        // TODO: set example expected response
        assert_eq!(part_two(&example), Some(1134));
    }
}
