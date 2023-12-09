use advent_of_code::{read_input, should_submit, submit};
use std::time::Instant;

const DAY: u8 = 9;
const YEAR: u16 = 2023;

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line|
            line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        )
        .collect()
}

fn serie_gaps(serie: &Vec<i32>) -> Vec<i32> {
    serie
        .windows(2)
        .map(|slice| match slice {
            &[a, b] => b - a,
            _ => unreachable!(), // Handle other slice patterns if needed
        })
        .collect()
}

fn find_next(serie: &Vec<i32>) -> i32 {
    let gaps = serie_gaps(serie);
    if gaps.iter().all(|&n| n == 0) {
        *serie.last().expect("No more element in serie")
    } else {
        serie.last().expect("No more element in serie") + find_next(&gaps)
    }
}

fn find_previous(serie: &Vec<i32>) -> i32 {
    let gaps = serie_gaps(serie);
    if gaps.iter().all(|&n| n == 0) {
        *serie.first().expect("No more element in serie")
    } else {
        serie.first().expect("No more element in serie") - find_previous(&gaps)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let series = parse_input(input);

    Some(
        series
            .into_iter()
            .map(|serie| {
                find_next(&serie)
            })
            .sum()
    )
}


pub fn part_two(input: &str) -> Option<i32> {
    let series = parse_input(input);

    Some(
        series
            .into_iter()
            .map(|serie| {
                find_previous(&serie)
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
            println!("Submitting part 1 response {ellapsed}μs: {part_one_response}");
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
    use super::{DAY, YEAR, part_one, part_two};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(114));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(2));
    }
}
