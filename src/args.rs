use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    #[command(subcommand)]
    pub day: Day,
}

#[derive(Subcommand, Debug)]
pub enum Day {
    /// Solve day 1's problem.
    Day1 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 2's problem.
    Day2 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 3's problem.
    Day3 {
        #[command(subcommand)]
        part: Part,
    },
    /// Solve day 4's problem.
    Day4 {
        #[command(subcommand)]
        part: Part,
    },
}

#[derive(Subcommand, Debug)]
pub enum Part {
    /// Solve the 1st part of the problem.
    Part1,
    /// Solve the 2nd part of the problem.
    Part2,
}
