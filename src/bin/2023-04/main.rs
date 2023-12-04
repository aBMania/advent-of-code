use std::collections::HashMap;
use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 4;
const YEAR: u16 = 2023;

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<_> = input.lines()
        .map(|line| {
            let parts: Vec<_> = line.splitn(2, ": ").collect();
            let id_parts: Vec<_> = parts.get(0).unwrap().splitn(2, ' ').collect();
            let id: u32 = id_parts.get(1).unwrap().trim().parse().unwrap();

            let numbers_parts: Vec<_> = parts.get(1).unwrap().splitn(2, '|').collect();
            let winning_numbers: Vec<u32> = numbers_parts.get(0).unwrap().split(' ').collect::<Vec<_>>().iter()
                .filter_map(|&number| number.trim().parse().ok())
                .collect();
            let my_numbers: Vec<u32> = numbers_parts.get(1).unwrap().split(' ').collect::<Vec<_>>().iter()
                .filter_map(|&number| number.trim().parse().ok())
                .collect();
            (id, winning_numbers, my_numbers)
        })
        .collect();

    Some(
        input.iter()
            .map(|(_, winning_numbers, my_numbers)| {
                let n = winning_numbers.iter().filter(|n| my_numbers.contains(n)).count();
                match n {
                    0 => 0,
                    _ => 2u32.pow(n as u32 - 1)
                }
            }).sum()
    )
}


pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<_> = input.lines()
        .map(|line| {
            let parts: Vec<_> = line.splitn(2, ": ").collect();
            let id_parts: Vec<_> = parts.get(0).unwrap().splitn(2, ' ').collect();
            let id: u32 = id_parts.get(1).unwrap().trim().parse().unwrap();

            let numbers_parts: Vec<_> = parts.get(1).unwrap().splitn(2, '|').collect();
            let winning_numbers: Vec<u32> = numbers_parts.get(0).unwrap().split(' ').collect::<Vec<_>>().iter()
                .filter_map(|&number| number.trim().parse().ok())
                .collect();
            let my_numbers: Vec<u32> = numbers_parts.get(1).unwrap().split(' ').collect::<Vec<_>>().iter()
                .filter_map(|&number| number.trim().parse().ok())
                .collect();
            (id, winning_numbers, my_numbers)
        })
        .collect();

    let mut cards: HashMap<u32, u32> = HashMap::new();

    Some(
        input.iter()
            .map(|(id, winning_numbers, my_numbers)| {
                let n_cards = 1 + *cards.get(id).unwrap_or(&0);


                let n = winning_numbers.iter().filter(|n| my_numbers.contains(n)).count();
                for i in (*id + 1)..=(*id + n as u32) {
                    *cards.entry(i).or_insert(0) += n_cards;
                };

                n_cards
            }).sum()
    )
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
        assert_eq!(part_one(&example), Some(13));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(30));
    }
}
