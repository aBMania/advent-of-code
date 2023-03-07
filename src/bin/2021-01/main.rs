use advent_of_code::{input_to_list, read_input, should_submit, submit};

const DAY: u8 = 1;
const YEAR: u16 = 2021;

pub fn part_one(input: &str) -> Option<u32> {
    let list: Vec<u32> = input_to_list(input).unwrap();

    let mut number_of_increases: u32 = 0;
    let mut previous: Option<u32> = None;

    for current in list {
        if let Some(previous_value) = previous {
            if current > previous_value {
                number_of_increases += 1;
            }
        }

        previous = Some(current);
    };

    Some(number_of_increases)
}


pub fn part_two(input: &str) -> Option<u32> {
    let list: Vec<u32> = input_to_list(input).unwrap();

    let mut number_of_increases: u32 = 0;
    let mut previous: Option<u32> = None;

    for current in list.windows(3) {
        let current_sum = current.iter().sum();

        if let Some(previous_value) = previous {
            if current_sum > previous_value {
                number_of_increases += 1;
            }
        }

        previous = Some(current_sum);
    };

    Some(number_of_increases)
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
        assert_eq!(part_one(&example), Some(7));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(5));
    }
}
