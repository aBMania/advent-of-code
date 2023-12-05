use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 5;
const YEAR: u16 = 2023;

#[derive(Debug)]
struct SeedMapper {
    from: u32,
    to: u32,
    width: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");

    let (_, seeds) = parts.next().unwrap().split_once(": ").expect("parse seed line");
    let seeds: Vec<u32> = seeds.split(' ').map(|s| s.parse().expect("parse seed")).collect();

    let mappers: Vec<Vec<SeedMapper>> = parts.map(|m| {
        m
            .lines()
            .skip(1)
            .map(|mapper_line| {
                let mut seed_mapper_parts = mapper_line.split(' ');
                let to = seed_mapper_parts.next().unwrap().parse().expect("parse to");
                let from = seed_mapper_parts.next().unwrap().parse().expect("parse from");
                let width = seed_mapper_parts.next().unwrap().parse().expect("parse width");
                SeedMapper {
                    from,
                    to,
                    width,
                }
            }).collect()
    })
        .collect::<Vec<Vec<SeedMapper>>>();

    mappers
        .into_iter()
        .fold(seeds, |mut seeds, seed_mappers| {
            let mut mapped_seeds = vec![];

            for seed_mapper in seed_mappers.into_iter() {
                let gap: i64 = seed_mapper.to as i64 - seed_mapper.from as i64;
                let (mapped, stayed_same): (Vec<_>, Vec<_>) = seeds
                    .into_iter()
                    .partition(|&seed| seed >= seed_mapper.from && seed <= seed_mapper.from + seed_mapper.width);
                seeds = stayed_same;
                mapped_seeds = [mapped.into_iter().map(|seed| (seed as i64 + gap) as u32).collect(), mapped_seeds].concat();
            }

            [seeds, mapped_seeds].concat()
        })
        .into_iter()
        .min()
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");

    let (_, seeds) = parts.next().unwrap().split_once(": ").expect("parse seed line");
    let seeds: Vec<u32> = seeds.split(' ').map(|s| s.parse().expect("parse seed"))
        .collect::<Vec<u32>>()
        .chunks(2)
        .flat_map(|chunk| {
            let start = *chunk.get(0).unwrap();
            let length = *chunk.get(1).unwrap();
            start..start + length
        })
        .collect()
        ;
    // 1 394 379 071
    println!("{}", seeds.len());

    let mappers: Vec<Vec<SeedMapper>> = parts.map(|m| {
        m
            .lines()
            .skip(1)
            .map(|mapper_line| {
                let mut seed_mapper_parts = mapper_line.split(' ');
                let to = seed_mapper_parts.next().unwrap().parse().expect("parse to");
                let from = seed_mapper_parts.next().unwrap().parse().expect("parse from");
                let width = seed_mapper_parts.next().unwrap().parse().expect("parse width");
                SeedMapper {
                    from,
                    to,
                    width,
                }
            }).collect()
    })
        .collect::<Vec<Vec<SeedMapper>>>();

    mappers
        .into_iter()
        .fold(seeds, |mut seeds, seed_mappers| {
            println!("New fold round");
            let mut mapped_seeds = vec![];

            for seed_mapper in seed_mappers.into_iter() {
                println!("{:?}", seed_mapper);
                let gap: i64 = seed_mapper.to as i64 - seed_mapper.from as i64;
                let (mapped, stayed_same): (Vec<_>, Vec<_>) = seeds
                    .into_iter()
                    .partition(|&seed| seed >= seed_mapper.from && seed < seed_mapper.from + seed_mapper.width);
                seeds = stayed_same;
                mapped_seeds = [mapped.into_iter().map(|seed| (seed as i64 + gap) as u32).collect(), mapped_seeds].concat();
            }

            [seeds, mapped_seeds].concat()
        })
        .into_iter()
        .min()
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
        assert_eq!(part_one(&example), Some(35));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(46));
    }
}
