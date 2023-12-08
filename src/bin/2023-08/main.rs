use std::collections::HashMap;
use std::time::Instant;
use num::integer::lcm;

use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 8;
const YEAR: u16 = 2023;

#[derive(Eq, PartialOrd, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<(&str, Direction), &str>) {
    let (directions, nodes) = input.split_once("\n\n").expect("split directions and nodes");
    let directions: Vec<_> = directions.chars().map(|c| match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Unhandled direction char")
    }).collect();

    let nodes = nodes.lines().flat_map(|line| {
        let node = &line[..3];
        let left_node = &line[7..10];
        let right_node = &line[12..15];

        [
            ((node, Direction::Left), left_node),
            ((node, Direction::Right), right_node)
        ]
    }).collect();

    (directions, nodes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = parse_input(input);

    let mut n = 0;
    let mut current_node = "AAA";

    for direction in directions.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }
        current_node = nodes.get(&(current_node, *direction)).unwrap();
        n += 1;
    }

    Some(n)
}


pub fn part_two(input: &str) -> Option<u64> {
    let (directions, nodes) = parse_input(input);

    let mut n = 0;
    let mut current_nodes: Vec<_> = nodes.keys()
        .filter(|(node, direction)| node.ends_with('A') && direction.eq(&Direction::Left))
        .map(|(node, _)| *node)
        .collect();

    let mut node_cycle: Vec<Option<u64>> = (0..current_nodes.len()).map(|_| None).collect();

    for direction in directions.iter().cycle() {
        for (i, node) in current_nodes.iter().enumerate() {
            if node.ends_with('Z') {
                node_cycle[i] = Some(n);
            }
        }

        if node_cycle.iter().all(|cycle| cycle.is_some()) {
            break;
        }

        for current_node in current_nodes.iter_mut() {
            *current_node = nodes.get(&(current_node, *direction)).unwrap();
        }
        n += 1;
    }

    Some(node_cycle.into_iter().fold(1u64, |acc, cycle| lcm(acc, cycle.unwrap())))
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
    use advent_of_code::{read_example, read_example_2};

    use super::{DAY, part_one, part_two, YEAR};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(2));
    }

    #[test]
    fn test_part_two() {
        let example = read_example_2(DAY, YEAR);
        assert_eq!(part_two(&example), Some(6));
    }
}
