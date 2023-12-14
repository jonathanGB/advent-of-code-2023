use crate::solver::Solver;

pub struct Day9Solver {}

impl Solver for Day9Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day9/input.txt").unwrap();

        let histories: Vec<Vec<i64>> = file
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        // Finding the extrapolated value to the right, if you look closely, is in fact the sum
        // of the last entry in each processed step.
        let mut sum_of_extrapolated_values = 0;
        for mut history in histories {
            let mut extrapolated_value = 0;
            while !history.iter().all(|difference| *difference == 0) {
                for i in 0..(history.len() - 1) {
                    history[i] = history[i + 1] - history[i];
                }

                extrapolated_value += history.pop().unwrap();
            }

            sum_of_extrapolated_values += extrapolated_value;
        }

        println!("The sum of all extrapolated values is {sum_of_extrapolated_values}");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day9/input.txt").unwrap();

        let histories: Vec<Vec<i64>> = file
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        let mut sum_of_extrapolated_values = 0;
        for mut history in histories {
            // Finding the extrapolated value to the left is analogous to part 1, except that
            // it is the difference of the first entry in each processed step. Note that the
            // substraction must be distributed, so the even steps have their sign flipped to
            // an addition.
            let mut extrapolated_value = 0;
            let mut step = 0;
            while !history.iter().all(|difference| *difference == 0) {
                extrapolated_value += if step % 2 == 0 {
                    history[0]
                } else {
                    -history[0]
                };

                for i in 0..(history.len() - 1) {
                    history[i] = history[i + 1] - history[i];
                }

                history.pop();
                step += 1;
            }

            sum_of_extrapolated_values += extrapolated_value;
        }

        println!("The sum of all extrapolated values is {sum_of_extrapolated_values}");
    }
}
