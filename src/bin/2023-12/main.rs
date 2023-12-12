use advent_of_code::{read_input, should_submit, submit};
use std::time::Instant;
use memoize::memoize;
use rayon::prelude::*;

const DAY: u8 = 12;
const YEAR: u16 = 2023;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Row<'a> {
    pattern: &'a str,
    consecutives: Vec<u8>,
}

fn parse_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let [pattern, consecutives] = line.split_whitespace().collect::<Vec<_>>()[0..2] else { panic!() };
            let consecutives: Vec<_> = consecutives.split(',').map(|i| i.parse().unwrap()).collect();
            Row {
                pattern,
                consecutives,
            }
        })
        .collect()
}

#[memoize]
fn solve_row(row: Row<'static>) -> u64 {
    // let row_clone = row.clone();
    // println!("solving {:?}", &row);
    if row.consecutives.is_empty() {
        return match row.pattern.contains('#') {
            true => {
                // println!("{row_clone:?}");
                // println!("return 0 (consecutive empty, # remaining)");
                0
            }
            false => {
                // println!("{row_clone:?}");
                // println!("return 1 (consecutive empty, no #)");
                1
            }
        };
    }

    if row.pattern.is_empty() {
        // println!("{row_clone:?}");
        // println!("return 0 (pattern empty)");
        return 0;
    }

    let mut row = row.clone();
    row.pattern = row.pattern.trim_end_matches('.'); // Remove trailing dots
    let consecutive = row.consecutives.pop().unwrap() as usize;

    let last_dot_pos = row.pattern.rfind('.').map(|x| x);

    let left_space = match last_dot_pos {
        None => row.pattern.len(),
        Some(last_dot_pos) => row.pattern.len() - last_dot_pos - 1
    };

    if left_space < consecutive {
        return match last_dot_pos {
            None => {
                // println!("{row_clone:?}");
                // println!("return 0 (no space left for a {consecutive} consecutive)");
                0
            }
            Some(last_dot_pos) => {
                if row.pattern[last_dot_pos..].contains('#') {
                    // println!("{row_clone:?}");
                    // println!("return 0 (cannot include # in a consecutive)");
                    0
                } else {
                    let mut consecutive_with_popped = row.consecutives.clone();
                    consecutive_with_popped.push(consecutive as u8);
                    let sub_solve = solve_row(Row {
                        consecutives: consecutive_with_popped,
                        pattern: &row.pattern[..last_dot_pos],
                    });

                    // println!("{row_clone:?}");
                    // println!("return {sub_solve} (no solution, went to subsolve for {:?})", &row.pattern[..last_dot_pos]);

                    sub_solve
                }
            }
        };
    }

    let current = &row.pattern[row.pattern.len() - left_space..];
    let rest = &row.pattern[..row.pattern.len() - left_space];

    if left_space == consecutive {
        let sub_case_replace_by_hashtag = Row {
            consecutives: row.consecutives.clone(),
            pattern: rest,
        };

        return if current.chars().all(|c| c.eq(&'?')) {
            let mut consecutive_with_popped = row.consecutives.clone();
            consecutive_with_popped.push(consecutive as u8);
            let sub_case_replace_by_dot = Row {
                consecutives: consecutive_with_popped,
                pattern: rest,
            };

            let sub_case_replace_by_dot_solve = solve_row(sub_case_replace_by_dot);
            let sub_case_replace_by_hashtag_solve = solve_row(sub_case_replace_by_hashtag);

            // println!("{row_clone:?}");
            // println!("return {sub_case_replace_by_dot_solve} + {sub_case_replace_by_hashtag_solve} (dot + hashtag)");
            sub_case_replace_by_dot_solve + sub_case_replace_by_hashtag_solve
        } else {
            let sub_case_replace_by_hashtag_solve = solve_row(sub_case_replace_by_hashtag);

            // println!("{row_clone:?}");
            // println!("return sub_case_replace_by_hashtag_solve (hashtag)");

            sub_case_replace_by_hashtag_solve
        };
    }

    let possibilities = left_space - consecutive as usize + 1;
    let start_looking = row.pattern.len() - left_space;
    let end_looking = start_looking + possibilities;

    let mut total = 0;

    // Consider replacing everything by dots
    if let Some(last_dot_pos) = last_dot_pos {
        if !row.pattern[last_dot_pos..].contains('#') {
            let mut consecutive_with_popped = row.consecutives.clone();
            consecutive_with_popped.push(consecutive as u8);
            let sub_solve = solve_row(Row {
                consecutives: consecutive_with_popped,
                pattern: &row.pattern[..last_dot_pos],
            });

            total += sub_solve;
        }
    }


    for i in start_looking..end_looking {
        let before = match i {
            0 => '.',
            i @ _ => row.pattern.chars().nth(i - 1).unwrap()
        };
        let after = &row.pattern[i + consecutive..];
        // println!("{i} before: {before} after: {after}");
        if (before == '?' || before == '.') &&
            after.chars().all(|c| c.eq(&'?')) {
            let sub_row = match i {
                0 => Row {
                    consecutives: row.consecutives.clone(),
                    pattern: "",
                },
                i @ _ => Row {
                    consecutives: row.consecutives.clone(),
                    pattern: &row.pattern[..i - 1],
                }
            };

            total += solve_row(sub_row)
        }
    }

    // println!("{row_clone:?}");
    // println!("return sum of {:?}: {}", &total, total.iter().sum::<u64>());
    total
}

fn solve(rows: Vec<Row<'static>>) -> u64 {
    rows
        .par_iter()
        .map(|row| {
            let solve = solve_row(row.clone());
            // println!("{:?} {:?} {}", row.pattern, row.consecutives, solve);
            solve
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.to_string().leak();
    let input = parse_input(input);

    Some(solve(input))
}

fn complicate_things(input: &mut Vec<Row>) {
    for row in input.iter_mut() {
        let mut duplicated = format!("{}?", row.pattern).repeat(5);
        duplicated.pop(); // remove trailing ?
        row.pattern = duplicated.leak();
        row.consecutives = row.consecutives.repeat(5);

        // println!("{:?}", row);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.to_string().leak();
    let mut input = parse_input(input);

    complicate_things(&mut input);

    Some(solve(input))
}

fn main() {
    let input = read_input(DAY, YEAR);
    let now = Instant::now();
    let part_one_response = part_one(&input);
    let ellapsed = now.elapsed().as_micros();

    if let Some(part_one_response) = part_one_response {
        if should_submit() {
            println!("Submitting part 1 response ({ellapsed})μs: {part_one_response}");
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
    use super::{DAY, YEAR, part_one, part_two, solve_row, Row};

    #[test]
    fn test_solve_row_trivial() {
        assert_eq!(solve_row(Row {
            pattern: "#",
            consecutives: vec![1],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: "?",
            consecutives: vec![1],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: ".",
            consecutives: vec![1],
        }), 0);
        assert_eq!(solve_row(Row {
            pattern: ".#",
            consecutives: vec![1],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: "..##",
            consecutives: vec![2],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: "###.",
            consecutives: vec![3],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: "?.?.?",
            consecutives: vec![1],
        }), 3);
    }

    #[test]
    fn test_solve_example_individual() {
        assert_eq!(solve_row(Row {
            pattern: "..???",
            consecutives: vec![1, 1],
        }), 1);

        assert_eq!(solve_row(Row {
            pattern: ".??..??...?##.",
            consecutives: vec![1, 1, 3],
        }), 4);

        assert_eq!(solve_row(Row {
            pattern: "?#?#?#?#?#?#?#?",
            consecutives: vec![1, 3, 1, 6],
        }), 1);

        assert_eq!(solve_row(Row {
            pattern: "????.#...#...",
            consecutives: vec![4, 1, 1],
        }), 1);

        assert_eq!(solve_row(Row {
            pattern: "????.######..#####.",
            consecutives: vec![1, 6, 5],
        }), 4);

        assert_eq!(solve_row(Row {
            pattern: "?###????????",
            consecutives: vec![3, 2, 1],
        }), 10);
    }

    #[test]
    fn test_solve_part_1_individual() {
        assert_eq!(solve_row(Row { pattern: "##????????#?#??????", consecutives: vec![4, 1, 8, 2] }), 4);
        assert_eq!(solve_row(Row { pattern: "?.#??????.#????#??", consecutives: vec![1, 1, 1, 1, 1, 7] }), 1);
        assert_eq!(solve_row(Row { pattern: ".#??.??.????###?????", consecutives: vec![1, 1, 2, 8, 3] }), 1);
        assert_eq!(solve_row(Row { pattern: "??.???#????", consecutives: vec![1, 4, 1] }), 13);
        assert_eq!(solve_row(Row { pattern: "?????.??????##.", consecutives: vec![2, 3, 3] }), 8);
        assert_eq!(solve_row(Row { pattern: ".??#??.??#", consecutives: vec![3, 2] }), 3);
        assert_eq!(solve_row(Row { pattern: "?.#?##??#.?#?????", consecutives: vec![1, 5, 1, 2, 3] }), 3);
        assert_eq!(solve_row(Row { pattern: "?.###??.??#??????", consecutives: vec![4, 8] }), 2);
        assert_eq!(solve_row(Row { pattern: "?#?????#???", consecutives: vec![2, 1, 1] }), 9);
        assert_eq!(solve_row(Row { pattern: "???????..??#?.", consecutives: vec![3, 1] }), 5);
        assert_eq!(solve_row(Row { pattern: "??#??###.????#??.???", consecutives: vec![1, 6, 2, 3, 3] }), 3);
        assert_eq!(solve_row(Row { pattern: "???????#?????#..??", consecutives: vec![5, 2] }), 4);
        assert_eq!(solve_row(Row { pattern: "....#?##????.??#??", consecutives: vec![4, 1] }), 1);
        assert_eq!(solve_row(Row { pattern: "?#??.?.?#?????", consecutives: vec![2, 1, 4] }), 8);
        assert_eq!(solve_row(Row { pattern: "?#?##????#??.#?#", consecutives: vec![5, 4, 1, 1] }), 5);
        assert_eq!(solve_row(Row { pattern: "..?.????#??????????", consecutives: vec![1, 1, 1, 1, 1, 4] }), 26);
        assert_eq!(solve_row(Row { pattern: "?.????#????", consecutives: vec![4, 2] }), 3);
        assert_eq!(solve_row(Row { pattern: "??.#???.?????", consecutives: vec![1, 3, 1, 1] }), 12);
        assert_eq!(solve_row(Row { pattern: ".?###???????.?##", consecutives: vec![4, 2, 3] }), 9);
        assert_eq!(solve_row(Row { pattern: "#??.?##????#?????", consecutives: vec![3, 8, 2] }), 5);
        assert_eq!(solve_row(Row { pattern: "?#.???.?#?", consecutives: vec![2, 1, 2] }), 6);
        assert_eq!(solve_row(Row { pattern: "?#???.#????.??", consecutives: vec![4, 2, 1, 2] }), 4);
        assert_eq!(solve_row(Row { pattern: ".???.??.#?????#?#", consecutives: vec![1, 1, 4, 1, 1] }), 7);
    }

    #[test]
    fn test_solve_part_1_cases() {
        assert_eq!(solve_row(Row {
            pattern: "???.",
            consecutives: vec![1, 1],
        }), 1);
    }

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(21));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(525152));
    }
}
