use itertools::Itertools;
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
        ].contains(&digits.len())
        ).count()
    });

    Some(count as u32)
}

pub fn compute_line_value(line: ([String; 10], [String; 4])) -> u32 {
    let one = line.0.iter().find(|word| word.len() == 2).unwrap();
    let seven = line.0.iter().find(|word| word.len() == 3).unwrap();
    let eight = line.0.iter().find(|word| word.len() == 7).unwrap();

    println!("One:   {one}");
    println!("Seven: {seven}");
    println!("Eight: {eight}");

    let top = seven.chars().find(|c| !one.contains(*c)).unwrap();

    println!("top: {top}");

    let digits_with_6_bars: Vec<&String> = line.0.iter().filter(|word| word.len() == 6).collect();

    let six = *digits_with_6_bars.iter().find(|x| !one.chars().all(|c| x.contains(c))).unwrap();

    println!("Six: {six}");

    let bottom_right = one.chars().find(|c| six.contains(*c)).unwrap();
    let top_right = one.chars().find(|c| !six.contains(*c)).unwrap();

    println!("bottom_right: {bottom_right}");
    println!("top_right: {top_right}");

    let three = line.0.iter().filter(|word| {
        word.chars().filter(|c| [top, top_right, bottom_right].contains(c)).count() == 3
    }).find(|word| word.len() == 5).unwrap();

    println!("Three: {three}");

    let nine = *digits_with_6_bars.iter().find(|x| three.chars().all(|c| x.contains(c))).unwrap();

    println!("Nine: {nine}");

    let zero = *digits_with_6_bars.iter().find(|word| ![six, nine].contains(word)).unwrap();

    println!("Zero: {zero}");

    let two_and_five: Vec<&String> = line.0.iter().filter(|word| word.len() == 5 && !word.eq(&three)).collect();

    let two = *two_and_five.iter().find(|word| word.chars().contains(&top_right)).unwrap();
    let five = *two_and_five.iter().find(|word| !word.chars().contains(&top_right)).unwrap();

    println!("Two: {two}");
    println!("Five: {five}");

    let four = line.0.iter().find(|word| word.len() == 4).unwrap();

    println!("Four: {four}");

    let line_value = line.1.iter().enumerate().fold(0, |acc, (i, word)| {
        let exponent: u32 = 3u32 - (i as u32);
        let multiplicator = 10u32.pow(exponent);

        let value = word.chars().sorted().collect::<String>();
        acc + multiplicator * match value {
            _ if value.eq(&zero.chars().sorted().collect::<String>()) => 0,
            _ if value.eq(&one.chars().sorted().collect::<String>()) => 1,
            _ if value.eq(&two.chars().sorted().collect::<String>()) => 2,
            _ if value.eq(&three.chars().sorted().collect::<String>()) => 3,
            _ if value.eq(&four.chars().sorted().collect::<String>()) => 4,
            _ if value.eq(&five.chars().sorted().collect::<String>()) => 5,
            _ if value.eq(&six.chars().sorted().collect::<String>()) => 6,
            _ if value.eq(&seven.chars().sorted().collect::<String>()) => 7,
            _ if value.eq(&eight.chars().sorted().collect::<String>()) => 8,
            _ if value.eq(&nine.chars().sorted().collect::<String>()) => 9,
            _ => unreachable!()
        }
    });

    println!("Line value: {line_value}");

    line_value
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed: Vec<([String; 10], [String; 4])> = input.lines().map(parse_line).collect();

    let total_sum: u32 = parsed.into_iter().map(|line| compute_line_value(line)).sum();

    println!("Total sum: {}", total_sum);

    Some(total_sum)
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
        assert_eq!(part_one(&example), Some(26));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        // TODO: set example expected response
        assert_eq!(part_two(&example), Some(61229));
    }
}
