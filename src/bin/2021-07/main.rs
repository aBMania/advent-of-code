use itertools::Itertools;
use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 7;
const YEAR: u16 = 2021;

pub fn part_one(input: &str) -> Option<u32> {
    let crabs: Vec<u32> = input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .sorted()
        .collect();

    let n = crabs.len();
    let median = match crabs.len() % 2 {
        0 => (crabs[n / 2 - 1] + crabs[n / 2]) / 2,
        1 => crabs[n / 2],
        _ => unreachable!()
    };

    Some(crabs.iter().map(|x| x.abs_diff(median)).sum())
}


pub fn part_two(input: &str) -> Option<u32> {
    let crabs: Vec<u32> = input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .sorted()
        .collect();

    let min: u32 = *crabs.iter().min().unwrap();
    let max: u32 = *crabs.iter().max().unwrap();

    (min..max).into_iter()
        .map(|value|
            crabs
                .iter()
                .map(|x| x.abs_diff(value) * (x.abs_diff(value) + 1) / 2)
                .sum()
        )
        .min()
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
    use super::{DAY, YEAR, part_one, part_two};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(37));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(168));
    }
}
