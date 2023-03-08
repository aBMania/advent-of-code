use advent_of_code::{input_to_list, read_input, should_submit, submit};

const DAY: u8 = 3;
const YEAR: u16 = 2021;

fn get_gamma_epsilon(list: &Vec<u32>, width: usize) -> (u32, u32) {
    let bit_count: Vec<u32> = list
        .iter()
        .fold(vec![0; width], |count, value| {
            count.into_iter().enumerate().map(
                |(bit, bit_count)| bit_count + ((value & 1 << bit) >> bit)
            ).collect()
        });

    let gamma: u32 = bit_count
        .into_iter()
        .enumerate()
        .map(|(bit, count)| {
            ((count * 2 >= list.len() as u32) as u32) << bit
        })
        .sum();

    let epsion_mask = (1 << width) - 1;
    let epsilon = gamma ^ epsion_mask;

    (gamma, epsilon)
}

pub fn part_one(input: &str) -> Option<u32> {
    let list: Vec<String> = input_to_list(input).unwrap();

    let width = list[0].len();

    let list = list
        .iter()
        .map(|value| u32::from_str_radix(value, 2).unwrap())
        .collect();

    let (gamma, epsilon) = get_gamma_epsilon(&list, width);

    Some(epsilon * gamma)
}


pub fn part_two(input: &str) -> Option<u32> {
    let list: Vec<String> = input_to_list(input).unwrap();
    let width = list[0].len();

    let list: Vec<u32> = list
        .iter()
        .map(|value| u32::from_str_radix(value, 2).unwrap())
        .collect();

    let mut oxygen_list: Vec<u32> = list.clone();

    for i in (0..width).rev() {
        // It's a bit overkill to compute gamma for all bits, we only need the informations for the current (i th) bit.
        let (gamma, _) = get_gamma_epsilon(&oxygen_list, width);
        let gamma_bit = (gamma & (1 << i)) >> i;

        oxygen_list = oxygen_list.clone().into_iter().filter(|value| (*value & (1 << i)) >> i == gamma_bit).collect();

        if oxygen_list.len() == 1 {
            break;
        }
    }

    let oxygen = oxygen_list.last().copied().unwrap();

    let mut co2_list: Vec<u32> = list;

    for i in (0..width).rev() {
        let (_, epsilon) = get_gamma_epsilon(&co2_list, width);
        let epsilon_bit = (epsilon & (1 << i)) >> i;

        co2_list = co2_list.clone().into_iter().filter(|value| (*value & (1 << i)) >> i == epsilon_bit).collect();

        if co2_list.len() == 1 {
            break;
        }
    }

    let co2 = co2_list.last().copied().unwrap();

    Some(oxygen * co2)
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
                        println!("{part_two_response}");
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
    use advent_of_code::{read_example, read_input};
    use super::{DAY, YEAR, part_one, part_two};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(198));
    }


    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(230));
    }

    #[test]
    fn input_part_two() {
        let input = read_input(DAY, YEAR);
        assert_eq!(part_two(&input), Some(1353024));
    }
}
