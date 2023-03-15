use std::collections::HashMap;
use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 6;
const YEAR: u16 = 2021;

pub fn solve(input: &str, n_days: u32) -> Option<u64> {
    let mut fishes: HashMap<u64, usize> = input
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .fold(HashMap::new(), |mut counts, timer| {
            *counts.entry(timer).or_insert(0) += 1;
            counts
        });

    for _ in 0..n_days {
        let mut n_new_fished = 0;

        for day in 0..=8 {
            let mut day_fishes = *fishes.get(&day).unwrap_or(&0);

            if day == 0 {
                // New born
                n_new_fished = day_fishes;
                continue;
            }
            if day == 7 {
                // Parents are reseted to 6 days
                day_fishes += n_new_fished
            }

            *fishes.entry(day - 1).or_insert(0) = day_fishes;
        }

        *fishes.entry(8).or_insert(0) = n_new_fished;
    }

    let sum = fishes.iter().map(|(_, n)| *n as u64).sum();

    Some(sum)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 80)
}


pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 256)
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
        assert_eq!(part_one(&example), Some(5934));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(26984457539));
    }
}
