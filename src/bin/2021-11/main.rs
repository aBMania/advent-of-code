use advent_of_code::{input_to_grid, read_input, should_submit, submit};

const DAY: u8 = 11;
const YEAR: u16 = 2021;

const NEXT: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn increment(grid: &mut grid::Grid<u8>, x: usize, y: usize) {
    if grid.get(y, x).is_none() {
        return;
    }
    grid[y][x] += 1;

    if grid[y][x] == 10 {
        for (xx, yy) in NEXT {
            increment(grid, (x as isize + xx) as usize, (y as isize + yy) as usize)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input_to_grid::<u8>(input).unwrap();
    let n_steps = 100u8;
    let mut n_flashes = 0u32;

    for _ in 0..n_steps {
        for x in 0..grid.cols() {
            for y in 0..grid.rows() {
                increment(&mut grid, x, y)
            }
        }
        for x in 0..grid.cols() {
            for y in 0..grid.rows() {
                if grid[y][x] >= 10 {
                    grid[y][x] = 0;
                    n_flashes += 1;
                }
            }
        }
    }

    Some(n_flashes)
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input_to_grid::<u8>(input).unwrap();
    let mut i= 0;

    loop {
        i += 1;
        for x in 0..grid.cols() {
            for y in 0..grid.rows() {
                increment(&mut grid, x, y)
            }
        }
        let mut n_flashes = 0u32;
        for x in 0..grid.cols() {
            for y in 0..grid.rows() {
                if grid[y][x] >= 10 {
                    grid[y][x] = 0;
                    n_flashes += 1;
                }
            }
        }

        if n_flashes == (grid.cols() * grid.rows()) as u32 {
            return Some(i)
        }
    }
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
        assert_eq!(part_one(&example), Some(1656));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(195));
    }
}
