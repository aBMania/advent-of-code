use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;
use advent_of_code::{read_input, should_submit, submit};
use nom::{
    IResult,
    bytes::complete::{tag, take_while},
    character::complete::char,
    combinator::{map, map_res},
    sequence::{tuple, separated_pair},
};
use itertools::Itertools;

const DAY: u8 = 5;
const YEAR: u16 = 2021;

fn uint<T>(input: &str) -> IResult<&str, T> where T: FromStr {
    map_res(take_while(|c: char| c.is_ascii_digit()), |s: &str| s.parse::<T>())(input)
}


fn parse_line<T>(input: &str) -> (T, T, T, T) where T: FromStr {
    let tuple_parser = tuple((separated_pair(uint, char(','), uint), tag(" -> "), separated_pair(uint, char(','), uint)));
    let mut result_parser = map(tuple_parser, |((a, b), _, (c, d))| (a, b, c, d));

    let (_, test) = result_parser(input).unwrap();
    test
}


pub fn part_one(input: &str) -> Option<usize> {
    let count = input.lines()
        .map(parse_line::<u16>)
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2) // Filter only horizontal & vertical lines
        .fold(HashMap::new(), |mut counts, (x1, y1, x2, y2)| {
            for point in (min(x1, x2)..=max(x1, x2)).cartesian_product(min(y1, y2)..=max(y1, y2)) {
                *counts.entry(point).or_insert(0) += 1;
            }
            counts
        })
        .into_iter()
        .filter(|((_, _), count)| {
            *count >= 2
        })
        .count();

    Some(count)
}


pub fn part_two(input: &str) -> Option<usize> {
    let count = input.lines()
        .map(parse_line::<i32>)
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2 || x1.abs_diff(*x2) == y1.abs_diff(*y2)) // Filter only horizontal, vertical and 45° lines
        .fold(HashMap::new(), |mut counts, (x1, y1, x2, y2)| {
            let dx = (x2 - x1).signum();
            let dy = (y2 - y1).signum();

            let (mut x, mut y) = (x1, y1);

            while (x, y) != (x2 + dx, y2 + dy) {
                *counts.entry((x, y)).or_insert(0) += 1;
                x += dx;
                y += dy;
            }

            counts
        })
        .into_iter()
        .filter(|((_, _), count)| {
            *count >= 2
        })
        .count();

    Some(count)
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
        assert_eq!(part_one(&example), Some(5));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(12));
    }
}
