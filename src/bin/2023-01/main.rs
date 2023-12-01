use itertools::Itertools;
use nom::FindSubstring;

use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 1;
const YEAR: u16 = 2023;

pub fn part_one(input: &str) -> Option<u32> {
    let response = input.lines().into_iter().map(|line| {
        let numeric_chars: Vec<_> = line.chars().filter(|x| x.is_numeric()).collect();
        format!("{}{}",
                numeric_chars.first().unwrap(),
                numeric_chars.last().unwrap()
        )
            .parse::<u32>()
            .unwrap()
    }).sum();
    Some(response)
}

const INT_STRINGS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part_two(input: &str) -> Option<u32> {
    let response = input.lines().into_iter().map(|mut line| {
        let mut replaced = true;

        // Absolute overkill solution
        // At first, i thought nineight should be understood as 9ight
        // But actually, it is expected to be 98
        // Simple adjustment is to replace 9 -> 9e (keeping the last char to be reused later)
        while replaced {
            let mut int_substrings_index: Vec<_> = INT_STRINGS
                .iter()
                .enumerate()
                .filter_map(|(index, &int_string)|
                    line
                        .find_substring(int_string)
                        .map(|i| (i, index)))
                .sorted_by(|(a, _), (b, _)| Ord::cmp(b, a))
                .collect();

            if let Some((_, i)) = int_substrings_index.pop() {
                // The trick
                let last_char = INT_STRINGS[i].chars().last().unwrap();

                line = &*line.replacen(INT_STRINGS[i], &*format!("{}{}", i + 1, last_char), 1).leak();
                replaced = true
            } else {
                replaced = false
            }
        }

        let numeric_chars: Vec<_> = line.chars().filter(|x| x.is_numeric()).collect();
        format!("{}{}",
                numeric_chars.first().unwrap(),
                numeric_chars.last().unwrap()
        )
            .parse::<u32>()
            .unwrap()
    }).sum();
    Some(response)
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
    use advent_of_code::{read_example, read_example_2};

    use super::{DAY, part_one, part_two, YEAR};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(142));
    }

    #[test]
    fn test_part_two() {
        let example = read_example_2(DAY, YEAR);
        assert_eq!(part_two(&example), Some(281));
    }
}
