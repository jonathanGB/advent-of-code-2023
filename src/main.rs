use clap::Parser;

mod args;
mod day1;
mod solver;

use args::{Args, Day};
use day1::Day1Solver;
use solver::Solver;

fn main() {
    let cli = Args::parse();

    match cli.day {
        Day::Day1 { part } => Day1Solver::solve(part),
    }
}
