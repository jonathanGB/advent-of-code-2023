#![feature(ascii_char)]
#[macro_use]
extern crate lazy_static;

use clap::Parser;

mod args;
mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod solver;

use args::{Args, Day};
use day1::Day1Solver;
use day10::Day10Solver;
use day2::Day2Solver;
use day3::Day3Solver;
use day4::Day4Solver;
use day5::Day5Solver;
use day6::Day6Solver;
use day7::Day7Solver;
use day8::Day8Solver;
use day9::Day9Solver;
use solver::Solver;

fn main() {
    let cli = Args::parse();

    match cli.day {
        Day::Day1 { part } => Day1Solver::solve(part),
        Day::Day2 { part } => Day2Solver::solve(part),
        Day::Day3 { part } => Day3Solver::solve(part),
        Day::Day4 { part } => Day4Solver::solve(part),
        Day::Day5 { part } => Day5Solver::solve(part),
        Day::Day6 { part } => Day6Solver::solve(part),
        Day::Day7 { part } => Day7Solver::solve(part),
        Day::Day8 { part } => Day8Solver::solve(part),
        Day::Day9 { part } => Day9Solver::solve(part),
        Day::Day10 { part } => Day10Solver::solve(part),
    }
}
