use std::cmp::{min, max};
use advent_of_code::{read_input, should_submit, submit};

const DAY: u8 = 5;
const YEAR: u16 = 2023;

#[derive(Debug)]
struct SeedMapper {
    range: Range,
    gap: i64,
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Range {
    from: u64,
    // included
    to: u64, // excluded
}

impl Range {
    fn new(from: u64, to: u64) -> Self {
        assert!(from < to);
        Self {
            from,
            to,
        }
    }
    fn intersect(&self, other: &Range) -> bool {
        self.from >= other.from && self.from < other.to
            || self.from <= other.from && self.to > other.from
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        if !self.intersect(other) {
            None
        } else {
            Some(Range {
                from: max(self.from, other.from),
                to: min(self.to, other.to),
            })
        }
    }


    // (intersection, rest)
    fn intersection_remainder(&self, other: &Range) -> (Option<Range>, Vec<Range>) {
        match self.intersection(other) {
            None => (None, vec![self.clone()]),
            Some(intersection) => {
                let mut remainder = vec![];
                if self.from < intersection.from {
                    remainder.push(Range::new(self.from, intersection.from))
                }
                if self.to > intersection.to {
                    remainder.push(Range::new(intersection.to, self.to))
                }
                (Some(intersection), remainder)
            }
        }
    }

    fn shift(&mut self, n: i64) {
        self.from = (self.from as i64 + n) as u64;
        self.to = (self.to as i64 + n) as u64;
    }
}

fn parse_mappers(input: &str) -> Vec<Vec<SeedMapper>> {
    let parts = input.split("\n\n");
    parts.map(|m| {
        m
            .lines()
            .skip(1)
            .map(|mapper_line| {
                let mut seed_mapper_parts = mapper_line.split(' ');
                let to: i64 = seed_mapper_parts.next().unwrap().parse().expect("parse to");
                let from: i64 = seed_mapper_parts.next().unwrap().parse().expect("parse from");
                let width: u64 = seed_mapper_parts.next().unwrap().parse().expect("parse width");
                SeedMapper {
                    range: Range::new(from as u64, from as u64 + width),
                    gap: to - from,
                }
            }).collect()
    }).collect()
}

fn parse_seeds_part1(input: &str) -> Vec<Range> {
    let (_, seeds) = input.split_once(": ").expect("parse seed line");
    seeds.split(' ')
        .map(|s| s.parse::<u64>().expect("parse seed"))
        .map(|seed| {
            Range {
                from: seed,
                to: seed + 1,
            }
        })
        .collect()
}

fn parse_seeds_part2(input: &str) -> Vec<Range> {
    let (_, seeds) = input.split_once(": ").expect("parse seed line");
    seeds.split(' ').map(|s| s.parse().expect("parse seed"))
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| {
            let from = *chunk.get(0).unwrap();
            let n = *chunk.get(1).unwrap();
            Range {
                from,
                to: from + n,
            }
        })
        .collect()
}

fn solve(seeds: Vec<Range>, mappers: Vec<Vec<SeedMapper>>) -> Option<u64> {
    let min_range = mappers
        .into_iter()
        .fold(seeds, |seeds, seed_mappers| {
            let (unchanged_seeds, changed_seeds) =
                seed_mappers
                    .into_iter()
                    .fold((seeds, vec![]), |(mut unchanged_seeds, mut changed_seeds), seed_mapper| {
                        unchanged_seeds = unchanged_seeds.into_iter().flat_map(|seed| {
                            let (intersection, remainders) = seed.intersection_remainder(&seed_mapper.range);
                            if let Some(mut intersection) = intersection {
                                intersection.shift(seed_mapper.gap);
                                changed_seeds.push(intersection)
                            }
                            remainders
                        })
                            .collect();

                        (unchanged_seeds, changed_seeds)
                    });
            [unchanged_seeds, changed_seeds].concat()
        })
        .into_iter()
        .min_by(|&r, &o| r.from.cmp(&o.from));

    Some(min_range.unwrap().from)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, mappers) = input.split_once("\n\n").expect("split seeds mappers");
    let seeds = parse_seeds_part1(seeds);
    let mappers = parse_mappers(mappers);

    solve(seeds, mappers)
}


pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, mappers) = input.split_once("\n\n").expect("split seeds mappers");
    let seeds = parse_seeds_part2(seeds);
    let mappers = parse_mappers(mappers);

    solve(seeds, mappers)
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
    use super::{DAY, YEAR, part_one, part_two, Range};

    #[test]
    fn range_intersect() {
        let range = Range::new(10, 15);

        assert_eq!(range.intersect(&range), true);
        assert_eq!(range.intersect(&Range::new(10, 11)), true);
        assert_eq!(range.intersect(&Range::new(11, 12)), true);
        assert_eq!(range.intersect(&Range::new(8, 12)), true);
        assert_eq!(range.intersect(&Range::new(12, 16)), true);
        assert_eq!(range.intersect(&Range::new(15, 16)), false);
        assert_eq!(range.intersect(&Range::new(9, 10)), false);
        assert_eq!(range.intersect(&Range::new(16, 17)), false);
        assert_eq!(range.intersect(&Range::new(8, 9)), false);
    }

    #[test]
    fn range_intersection() {
        let range = Range::new(10, 15);

        assert_eq!(range.intersection(&Range::new(10, 11)), Some(Range::new(10, 11)));
        assert_eq!(range.intersection(&Range::new(11, 12)), Some(Range::new(11, 12)));
        assert_eq!(range.intersection(&Range::new(8, 12)), Some(Range::new(10, 12)));
        assert_eq!(range.intersection(&Range::new(12, 16)), Some(Range::new(12, 15)));
        assert_eq!(range.intersection(&Range::new(15, 16)), None);
        assert_eq!(range.intersection(&Range::new(9, 10)), None);
        assert_eq!(range.intersection(&Range::new(16, 17)), None);
        assert_eq!(range.intersection(&Range::new(8, 9)), None);
    }

    #[test]
    fn range_intersection_remainder() {
        let range = Range::new(10, 15);

        assert_eq!(range.intersection_remainder(&Range::new(10, 11)), (Some(Range::new(10, 11)), vec![Range::new(11, 15)]));
        assert_eq!(range.intersection_remainder(&Range::new(11, 12)), (Some(Range::new(11, 12)), vec![Range::new(10, 11), Range::new(12, 15)]));
        assert_eq!(range.intersection_remainder(&Range::new(8, 12)), (Some(Range::new(10, 12)), vec![Range::new(12, 15)]));
        assert_eq!(range.intersection_remainder(&Range::new(12, 16)), (Some(Range::new(12, 15)), vec![Range::new(10, 12)]));
        assert_eq!(range.intersection_remainder(&Range::new(15, 16)), (None, vec![Range::new(10, 15)]));
        assert_eq!(range.intersection_remainder(&Range::new(12, 16)), (Some(Range::new(12, 15)), vec![Range::new(10, 12)]));
        assert_eq!(range.intersection_remainder(&Range::new(9, 10)), (None, vec![Range::new(10, 15)]));
        assert_eq!(range.intersection_remainder(&Range::new(9, 10)), (None, vec![Range::new(10, 15)]));
        assert_eq!(range.intersection_remainder(&Range::new(0, 10)), (None, vec![Range::new(10, 15)]));
    }

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
