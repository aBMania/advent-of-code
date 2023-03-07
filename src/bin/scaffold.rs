use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Write};
use std::process;
use std::process::{Command, Stdio};
use clap::Parser;

const TEMPLATE: &str = r###"use advent_of_code::{read_example, read_input, should_submit, submit};

const DAY: u8 = {DAY};
const YEAR: u16 = {YEAR};

pub fn part_one(input: &str) -> Option<u32> {
    // TODO: implem
    None
}


pub fn part_two(input: &str) -> Option<u32> {
    // TODO: implem
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
    use super::*;

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        // TODO: set example expected response
        assert_eq!(part_one(&example), None);
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        // TODO: set example expected response
        assert_eq!(part_two(&example), None);
    }
}
"###;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 2022)]
    year: u16,
}


fn safe_create_folder(path: &str) -> std::io::Result<()> {
    create_dir_all(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn main() {
    let Args { day, year } = Args::parse();

    let day_padded = format!("{day:02}");

    let folder_path = format!("src/bin/{year}-{day_padded}");
    let input_path = format!("src/bin/{year}-{day_padded}/input.txt");
    let example_path = format!("src/bin/{year}-{day_padded}/example.txt");
    let puzzle_path = format!("src/bin/{year}-{day_padded}/puzzle.md");
    let module_path = format!("src/bin/{year}-{day_padded}/main.rs");

    match safe_create_folder(&folder_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to create folder: {e}");
            process::exit(1);
        }
    }

    let mut file = match create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };


    match file.write(TEMPLATE
        .replace("{DAY}", &day.to_string())
        .replace("{YEAR}", &year.to_string())
        .as_bytes()) {
        Ok(_) => {
            println!("Created file: \"{module_path}\"");
        }
        Err(e) => {
            eprintln!("Failed to write module file: {e}");
            process::exit(1);
        }
    };

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created input file: \"{module_path}\"");
        }
        Err(e) => {
            eprintln!("Failed to write input file: {e}");
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created example file: \"{example_path}\"");
        }
        Err(e) => {
            eprintln!("Failed to write input file: {e}");
            process::exit(1);
        }
    }

    let args: Vec<String> = vec![
        "download".into(),
        "--overwrite".into(),
        "--input-file".into(),
        input_path,
        "--puzzle-file".into(),
        puzzle_path.to_string(),
        "--day".into(),
        day.to_string(),
        "--year".into(),
        year.to_string()
    ];


    // Fetch input and puzzle description
    match Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output() {
        Ok(_) => {
            println!("Fetched puzzle and input from aoc website");
        }
        Err(e) => {
            eprintln!("Failed to fetch puzzle and input from aoc website: {e}");
            process::exit(1);
        }
    }

    println!("---");
    println!(
        "🎄 Type `cargo solve {year}-{day_padded}` to run your solution."
    );
}
