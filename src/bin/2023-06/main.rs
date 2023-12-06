use advent_of_code::{read_input, should_submit, submit};
use std::time::Instant;

const DAY: u8 = 6;
const YEAR: u16 = 2023;

struct Race {
    time: u64,
    distance: u64,
}

fn parse_input_1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times: Vec<_> = lines.next().expect("times").split_ascii_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();
    let distances: Vec<_> = lines.next().expect("distances").split_ascii_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();

    times.into_iter().zip(distances)
        .map(|(time, distance)| Race {
            time,
            distance,
        })
        .collect()
}

fn solve_race(race: &Race) -> u64 {
    let t = race.time as f64;
    let d = race.distance as f64;
    let x_min = (0.5f64 * (t - (t * t - 4f64 * d).sqrt())).floor() as u64 + 1;
    let x_max = (0.5f64 * (t + (t * t - 4f64 * d).sqrt())).ceil() as u64 - 1;

    x_max - x_min + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_input_1(input);

    let product = races.iter().map(|race| solve_race(race)).product();

    Some(product)
}

fn parse_input_2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines.next().expect("time").chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u64>().expect("time parsing");
    let distance = lines.next().expect("distance").chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u64>().expect("distance parsing");

    Race {
        time,
        distance,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_input_2(input);

    Some(
        solve_race(&race)
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
        assert_eq!(part_one(&example), Some(288));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(71503));
    }
}
