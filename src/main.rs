#![feature(ascii_char)]
#[macro_use]
extern crate lazy_static;

use clap::Parser;

mod args;
mod day1;
mod day2;
mod day3;
mod solver;

use args::{Args, Day};
use day1::Day1Solver;
use day2::Day2Solver;
use day3::Day3Solver;
use solver::Solver;

fn main() {
    let cli = Args::parse();

    match cli.day {
        Day::Day1 { part } => Day1Solver::solve(part),
        Day::Day2 { part } => Day2Solver::solve(part),
        Day::Day3 { part } => Day3Solver::solve(part),
    }
}
