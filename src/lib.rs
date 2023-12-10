#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use std::{env, fs, io, process};
use std::fmt::{Debug, Display};
use std::process::{Command, Stdio};
use std::str::FromStr;
use grid::*;

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

pub fn read_example_2(day: u8, year: u16) -> String {
    let cwd = env::current_dir().unwrap();
    let input_filepath = cwd.join("src").join("bin").join(format!("{year}-{day:02}/example2.txt"));
    fs::read_to_string(input_filepath).expect("Could not open example")
}

pub fn get_folder_path(day: u8, year: u16) -> String {
    let day_padded = format!("{day:02}");
    format!("src/bin/{year}-{day_padded}")
}

pub fn get_input_path(day: u8, year: u16) -> String {
    let day_padded = format!("{day:02}");
    format!("src/bin/{year}-{day_padded}/input.txt")
}

pub fn get_example_path(day: u8, year: u16) -> String {
    let day_padded = format!("{day:02}");
    format!("src/bin/{year}-{day_padded}/example.txt")
}

pub fn get_puzzle_path(day: u8, year: u16) -> String {
    let day_padded = format!("{day:02}");
    format!("src/bin/{year}-{day_padded}/puzzle.md")
}

pub fn get_module_path(day: u8, year: u16) -> String {
    let day_padded = format!("{day:02}");
    format!("src/bin/{year}-{day_padded}/main.rs")
}


pub fn fetch_puzzle_and_input(day: u8, year: u16, input_path: &str, puzzle_path: &str) {
    let args: Vec<String> = vec![
        "download".into(),
        "--session-file".into(),
        ".session".into(),
        "--overwrite".into(),
        "--input-file".into(),
        input_path.to_string(),
        "--puzzle-file".into(),
        puzzle_path.to_string(),
        "--day".into(),
        day.to_string(),
        "--year".into(),
        year.to_string(),
    ];


    println!("aoc {:?}", args);

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
    GaveAnswerTooRecently,
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
        "--session-file".into(),
        ".session".into(),
        part.to_string(),
        response.to_string(),
        "--day".into(),
        day.to_string(),
        "--year".into(),
        year.to_string(),
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

pub type CustomGrid<T> = Grid<T>;

pub trait NeighborsIterator<'a, T: 'a> {
    type NeighborIter: Iterator<Item = ((usize, usize), &'a T)>;
    fn iter_neighbors(&'a self, row: usize, col: usize) -> Self::NeighborIter;
}

pub trait NeighborsDiagonalIterator<'a, T: 'a> {
    type NeighborIter: Iterator<Item = ((usize, usize), &'a T)>;
    fn iter_diagonal_neighbors(&'a self, row: usize, col: usize) -> Self::NeighborIter;
}

impl<'a, T: 'a> NeighborsIterator<'a, T> for CustomGrid<T> {
    type NeighborIter = impl Iterator<Item = ((usize, usize), &'a T)>;

    fn iter_neighbors(&'a self, row: usize, col: usize) -> Self::NeighborIter {
        [(0, -1), (0, 1), (1, 0), (-1, 0)]
            .iter()
            .map(move |(col_offset, row_offset)| ((col as isize + col_offset), (row as isize + row_offset)))
            .filter_map(|(col, row)| {
                if col.is_negative() || row.is_negative() {
                    None
                } else {
                    self.get(row as usize, col as usize)
                        .map(|val| ((row as usize, col as usize), val))
                }
            })
    }
}

impl<'a, T: 'a> NeighborsDiagonalIterator<'a, T> for CustomGrid<T> {
    type NeighborIter = impl Iterator<Item = ((usize, usize), &'a T)>;

    fn iter_diagonal_neighbors(&'a self, row: usize, col: usize) -> Self::NeighborIter {
        [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
            .iter()
            .map(move |(col_offset, row_offset)| ((col as isize + col_offset), (row as isize + row_offset)))
            .filter_map(|(col, row)| {
                if col.is_negative() || row.is_negative() {
                    None
                } else {
                    self.get(row as usize, col as usize)
                        .map(|val| ((row as usize, col as usize), val))
                }
            })
    }
}

pub trait Neighbors<T> {
    fn right(&self, row: usize, col: usize) -> Option<((usize, usize), &T)>;
    fn left(&self, row: usize, col: usize) -> Option<((usize, usize), &T)>;
    fn up(&self, row: usize, col: usize) -> Option<((usize, usize), &T)>;
    fn down(&self, row: usize, col: usize) -> Option<((usize, usize), &T)>;
    fn right_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)>;
    fn left_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)>;
    fn up_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)>;
    fn down_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)>;
}

impl<T> Neighbors<T> for CustomGrid<T> {
    fn right(&self, row: usize, col: usize) -> Option<((usize, usize), &T)> {
        self.get(row, col + 1).map(|val| ((row, col + 1), val))
    }

    fn left(&self, row: usize, col: usize) -> Option<((usize, usize), &T)> {
        if col == 0 {
            None
        } else {
            self.get(row, col - 1).map(|val| ((row, col - 1), val))
        }
    }

    fn up(&self, row: usize, col: usize) -> Option<((usize, usize), &T)> {
        if row == 0 {
            None
        } else {
            self.get(row - 1, col).map(|val| ((row - 1, col), val))
        }
    }

    fn down(&self, row: usize, col: usize) -> Option<((usize, usize), &T)> {
        self.get(row + 1, col).map(|val| ((row + 1, col), val))
    }

    fn right_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        self.get_mut(row, col + 1).map(|val| ((row, col + 1), val))
    }

    fn left_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        if col == 0 {
            None
        } else {
            self.get_mut(row, col - 1).map(|val| ((row, col - 1), val))
        }
    }

    fn up_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        if row == 0 {
            None
        } else {
            self.get_mut(row - 1, col).map(|val| ((row - 1, col), val))
        }
    }

    fn down_mut(&mut self, row: usize, col: usize) -> Option<((usize, usize), &mut T)> {
        self.get_mut(row + 1, col).map(|val| ((row + 1, col), val))
    }
}

pub fn input_to_grid<T: FromStr>(input: &str) -> Result<CustomGrid<T>, <T as FromStr>::Err> {
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    let cols = lines[0].len();

    let grid_data: Result<Vec<T>, <T as FromStr>::Err> = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(|c| c.to_string().parse::<T>())
        .collect();

    Ok(CustomGrid::from_vec(grid_data?, cols))
}

pub fn expand_grid<T: Default + Clone>(grid: &CustomGrid<T>, empty: T) -> CustomGrid<T> {
    let mut expanded_grid: Grid<T> = CustomGrid::init(grid.rows() * 2+1, grid.cols() * 2+1, empty);

    for ((row, col), value) in grid.indexed_iter() {
        *expanded_grid.get_mut(row * 2+1, col * 2+1).unwrap() = value.clone();
    }

    expanded_grid
}

pub fn print_grid<T: Display>(grid: &CustomGrid<T>) {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            print!("{}", grid.get(row, col).unwrap())
        }
        println!()
    }
}