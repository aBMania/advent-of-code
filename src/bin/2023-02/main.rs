use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 2;
const YEAR: u16 = 2023;

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

fn parse_draw(draw_str: &str) -> Draw {
    let mut draw = Draw {
        green: 0,
        blue: 0,
        red: 0,
    };
    for color_str in draw_str.split(',') {
        let mut parts = color_str.trim().split(' ');
        let n: u32 = parts.next().unwrap().parse().unwrap();
        let color: &str = parts.next().unwrap();

        match color {
            "red" => {
                draw.red = n
            }
            "blue" => {
                draw.blue = n
            }
            "green" => {
                draw.green = n
            }
            _ => panic!()
        }
    }

    draw
}

fn parse_draws(draws_str: &str) -> Vec<Draw> {
    draws_str.split(';')
        .into_iter()
        .map(|draw_str| parse_draw(draw_str))
        .collect()
}

fn parse_game(game_str: &str) -> Game {
    let mut parts = game_str.splitn(2, ':');
    let id: u32 = parts.next().unwrap().split(' ').last().unwrap().parse().unwrap();
    let draws = parse_draws(parts.next().unwrap());

    Game {
        id,
        draws,
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .map(|line| parse_game(line))
            .filter(|game: &Game| {
                game.draws.iter().all(|draw: &Draw| {
                    draw.blue <= 14
                        && draw.green <= 13
                        && draw.red <= 12
                })
            })
            .map(|game: Game| game.id)
            .sum()
    )
}


pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .map(|line| parse_game(line))
            .map(|game: Game| {
                let max_red = game.draws.iter().map(|draw: &Draw| draw.red).max().unwrap_or(0);
                let max_blue = game.draws.iter().map(|draw: &Draw| draw.blue).max().unwrap_or(0);
                let max_green = game.draws.iter().map(|draw: &Draw| draw.green).max().unwrap_or(0);

                max_blue * max_red * max_green
            })
            .sum()
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

    use super::{DAY, part_one, part_two, YEAR};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(8));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(2286));
    }
}
