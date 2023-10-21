use std::collections::VecDeque;
use itertools::Itertools;
use advent_of_code::{input_to_list, read_input, should_submit, submit};

const DAY: u8 = 10;
const YEAR: u16 = 2021;

pub fn opposite(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!()
    }
}

pub fn score_part_one(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!()
    }
}

pub fn score_part_two(c: char) -> u32 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let list: Vec<String> = input_to_list(input).unwrap();

    let total = list.iter().fold(0, |acc, line| {
        let mut stack: Vec<char> = Vec::new();
        let mut chars = line.chars().collect::<VecDeque<char>>();
        while let Some(c) = chars.pop_front() {
            match c {
                ')' | ']' | '}' | '>' => {
                    let o = stack.pop().unwrap();
                    if o != opposite(c) {
                        return acc + score_part_one(c);
                    }
                }
                c => {
                    stack.push(c)
                }
            }
        }
        acc
    });

    Some(total)
}


pub fn part_two(input: &str) -> Option<u32> {
    let list: Vec<String> = input_to_list(input).unwrap();

    let total: Vec<u64> = list.iter().map(|line| {
        let mut stack: Vec<char> = Vec::new();
        let mut chars = line.chars().collect::<VecDeque<char>>();
        while let Some(c) = chars.pop_front() {
            match c {
                ')' | ']' | '}' | '>' => {
                    let o = stack.pop().unwrap();
                    if o != opposite(c) {
                        return 0;
                    }
                }
                c => {
                    stack.push(c)
                }
            }
        }
        stack.into_iter().rev().fold(0u64, |acc, c| {
            (5 * acc) + score_part_two(c) as u64
        })
    }).filter(|x| *x != 0).sorted().collect();

    Some(*total.get(total.len() / 2).unwrap() as u32)
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
        assert_eq!(part_one(&example), Some(26_397));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(288957));
    }
}
