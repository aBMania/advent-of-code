use clap::Parser;
use advent_of_code::{fetch_puzzle_and_input, get_input_path, get_puzzle_path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 2022)]
    year: u16,
}

fn main() {
    let Args { day, year } = Args::parse();

    let puzzle_path = get_puzzle_path(day, year);
    let input_path = get_input_path(day, year);


    fetch_puzzle_and_input(day, year, &input_path, &puzzle_path);

    println!("---");
    println!(
        "🎄 Puzzle in {puzzle_path} updated"
    );
}
