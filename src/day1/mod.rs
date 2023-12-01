use crate::solver::Solver;
use regex::{Captures, Regex};

lazy_static! {
    static ref PART_1_FIRST_DIGIT_CALIBRATION_RE: Regex = Regex::new(r"^\D*?(\d)").unwrap();
    static ref PART_1_SECOND_DIGIT_CALIBRATION_RE: Regex = Regex::new(r"^.*(\d).*?$").unwrap();
    static ref PART_2_FIRST_DIGIT_CALIBRATION_RE: Regex =
        Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*$").unwrap();
    static ref PART_2_SECOND_DIGIT_CALIBRATION_RE: Regex =
        Regex::new(r"^.*(\d|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();
}
pub struct Day1Solver {}
impl Day1Solver {
    fn parse_calibration(captures: &Captures) -> usize {
        match captures.get(1).unwrap().as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            capture => capture.parse().unwrap(),
        }
    }
}

impl Solver for Day1Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day1/input.txt").unwrap();

        let mut total_calibrations = 0;
        for line in file.lines() {
            let captures = PART_1_FIRST_DIGIT_CALIBRATION_RE.captures(line).unwrap();
            let first_calibration = Self::parse_calibration(&captures);
            let captures = PART_1_SECOND_DIGIT_CALIBRATION_RE.captures(line).unwrap();
            let second_calibration = Self::parse_calibration(&captures);

            // The first calibration is in the 10s position, hence the multiplication.
            let curr_calibration = first_calibration * 10 + second_calibration;
            total_calibrations += curr_calibration;
        }

        println!("Sum of all calibrations: {total_calibrations}.");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day1/input.txt").unwrap();

        let mut total_calibrations = 0;
        for line in file.lines() {
            let captures = PART_2_FIRST_DIGIT_CALIBRATION_RE.captures(line).unwrap();
            let first_calibration = Self::parse_calibration(&captures);
            let captures = PART_2_SECOND_DIGIT_CALIBRATION_RE.captures(line).unwrap();
            let second_calibration = Self::parse_calibration(&captures);

            // The first calibration is in the 10s position, hence the multiplication.
            let curr_calibration = first_calibration * 10 + second_calibration;
            total_calibrations += curr_calibration;
        }

        println!("Sum of all calibrations: {total_calibrations}.");
    }
}
