use std::{env, fs, io};
use std::fmt::{Debug, Display};
use std::process::{Command};
use std::str::FromStr;

use clap::Parser;

pub fn read_input(day: u8, year: u16) -> String {
    let cwd = env::current_dir().unwrap();
    let input_filepath = cwd.join("src").join("bin").join(format!("{year}-{day:02}/input.txt"));
    fs::read_to_string(input_filepath).expect("Could not open input")
}

pub fn read_example(day: u8, year: u16) -> String {
    let cwd = env::current_dir().unwrap();
    let input_filepath = cwd.join("src").join("bin").join(format!("{year}-{day:02}/example.txt"));
    fs::read_to_string(input_filepath).expect("Could not open example")
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    submit: bool,
}

pub fn should_submit() -> bool {
    let Args { submit } = Args::parse();
    submit
}

pub enum SubmitError {
    IoError(io::Error),
    GaveAnswerTooRecently
}

impl Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmitError::IoError(e) => write!(f, "I/O Error: {e}"),
            SubmitError::GaveAnswerTooRecently => write!(f, "You gave an answer too recently")
        }
    }
}

pub fn submit(day: u8, year: u16, part: u8, response: &str) -> Result<bool, SubmitError> {
    let args: Vec<String> = vec![
        "submit".into(),
        part.to_string(),
        response.to_string(),
        "--day".into(),
        day.to_string(),
        "--year".into(),
        year.to_string()
    ];


    let output = Command::new("aoc")
        .args(args)
        .output()
        .map_err(|e| SubmitError::IoError(e))?;

    let stdout = String::from_utf8(output.stdout).unwrap();

    if stdout.contains("You gave an answer too recently") {
        return Err(SubmitError::GaveAnswerTooRecently);
    }

    if stdout.contains("That's not the right answer.") {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn input_to_list<T: FromStr>(input: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    input.lines().map(|line| line.trim().parse()).collect()
}
