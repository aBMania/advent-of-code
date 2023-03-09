use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 4;
const YEAR: u16 = 2021;


fn parse_input(input: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let (numbers, boards) = input.split_once("\n\n").unwrap();

    let numbers: Vec<u8> = numbers.split(',').map(|n| n.parse().unwrap()).collect();

    let boards = boards.split("\n\n").map(
        |board| board.replace('\n', " ").split_whitespace().map(|n| n.parse().unwrap()).collect()
    ).collect();

    (numbers, boards)
}

const COL_COMPLETION: u32 = 0b0000100001000010000100001u32;
const ROW_COMPLETION: u32 = 0b0000000000000000000011111u32;

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, boards) = parse_input(input);

    let mut board_completion = vec![0u32; boards.len()];

    for number in numbers {
        for i in 0..boards.len() {
            for j in 0..boards[i].len() {
                if boards[i][j] == number {
                    board_completion[i] += 1 << j;

                    for k in 0..4 {
                        if (board_completion[i] & (COL_COMPLETION << k) == (COL_COMPLETION << k))
                            || (board_completion[i] & (ROW_COMPLETION << (k * 5)) == (ROW_COMPLETION << (k * 5))) {
                            let board_sum: u32 = boards[i]
                                .iter()
                                .enumerate()
                                .filter(|(l, _)| {
                                    (board_completion[i] & (1 << l)) >> l == 0
                                }
                                )
                                .map(|(_, x)| *x as u32)
                                .sum();

                            return Some(number as u32 * board_sum)
                        }
                    }
                }
            };
        }
    }

    None
}


pub fn part_two(input: &str) -> Option<u32> {

    let (numbers, boards) = parse_input(input);

    let mut board_completion = vec![0u32; boards.len()];
    let mut board_won = vec![false; boards.len()];

    for number in numbers {
        for i in 0..boards.len() {
            for j in 0..boards[i].len() {
                if boards[i][j] == number {
                    board_completion[i] += 1 << j;

                    for k in 0..4 {
                        if (board_completion[i] & (COL_COMPLETION << k) == (COL_COMPLETION << k))
                            || (board_completion[i] & (ROW_COMPLETION << (k * 5)) == (ROW_COMPLETION << (k * 5))) {
                            let board_sum: u32 = boards[i]
                                .iter()
                                .enumerate()
                                .filter(|(l, _)| {
                                    (board_completion[i] & (1 << l)) >> l == 0
                                }
                                )
                                .map(|(_, x)| *x as u32)
                                .sum();

                            board_won[i] = true;

                            if board_won.iter().filter(|won| **won).count() == boards.len() {
                                return Some(number as u32 * board_sum)
                            }
                        }
                    }
                }
            };
        }
    }

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
    use super::{DAY, YEAR, part_one, part_two};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(4512));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(1924));
    }
}
