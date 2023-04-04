use nom::{IResult};
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::char;
use nom::combinator::{map_res, opt};
use nom::multi::{many_m_n};
use nom::sequence::{separated_pair, terminated};

use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 8;
const YEAR: u16 = 2021;

pub fn parse_line(input: &str) -> ([String; 10], [String; 4]) {
    let left_parser = many_m_n(0, 10, terminated(map_res(take_while(|c: char| c.is_alphabetic()), |s: &str| s.parse()), opt(char(' '))));
    let right_parser = many_m_n(0, 4, terminated(map_res(take_while(|c: char| c.is_alphabetic()), |s: &str| s.parse()), opt(char(' '))));
    let mut parser = separated_pair(left_parser, tag("| "), right_parser);

    let result: IResult<&str, (Vec<String>, Vec<String>)> = parser(input);
    let (_, parsed_line) = result.unwrap();

    (
        parsed_line.0.try_into().unwrap(),
        parsed_line.1.try_into().unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed: Vec<([String; 10], [String; 4])> = input.lines().map(parse_line).collect();

    let count = parsed.into_iter().fold(0, |acc, (_, outputs)| {
        acc + outputs.iter().filter(|digits| [
            2, // 1 has 2 segment on
            3, // 7 has 3 segment on
            4, // 4 has 4 segment on
            7  // 8 has 7 segment on
        ].contains(&digits.len())).count()
    });

    Some(count as u32)
}


pub fn part_two(input: &str) -> Option<u32> {
    // TODO: implem
    None
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
        assert_eq!(part_one(&example), Some(26));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        // TODO: set example expected response
        assert_eq!(part_two(&example), None);
    }
}
