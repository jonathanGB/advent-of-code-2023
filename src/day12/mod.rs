use std::collections::HashMap;
use std::str::Lines;

use crate::solver::Solver;

pub struct Day12Solver {}

impl Day12Solver {
    fn count_arrangements<'a>(
        unknown_record: &'a [char],
        goal_record: &'a [usize],
        cached_count_arrangements: &mut HashMap<(&'a [char], &'a [usize]), u64>,
    ) -> u64 {
        if unknown_record.is_empty() {
            if goal_record.is_empty() {
                return 1;
            }

            return 0;
        }

        if goal_record.is_empty() {
            if !unknown_record.contains(&'#') {
                return 1;
            }

            return 0;
        }

        let cached_count_arrangements_key = (unknown_record, goal_record);
        if let Some(count_arrangement) =
            cached_count_arrangements.get(&cached_count_arrangements_key)
        {
            return *count_arrangement;
        }

        let mut count_arrangements = 0;
        let first_unknown_record = unknown_record[0];
        let first_goal_record = goal_record[0];

        if first_unknown_record == '.' || first_unknown_record == '?' {
            count_arrangements += Self::count_arrangements(
                &unknown_record[1..],
                goal_record,
                cached_count_arrangements,
            );
        }
        if first_unknown_record == '#' || first_unknown_record == '?' {
            if unknown_record.len() > first_goal_record
                && !unknown_record[..first_goal_record].contains(&'.')
                && unknown_record[first_goal_record] != '#'
            {
                count_arrangements += Self::count_arrangements(
                    &unknown_record[first_goal_record + 1..],
                    &goal_record[1..],
                    cached_count_arrangements,
                );
            } else if unknown_record.len() == first_goal_record
                && !unknown_record[..first_goal_record].contains(&'.')
            {
                count_arrangements += Self::count_arrangements(
                    &unknown_record[first_goal_record..],
                    &goal_record[1..],
                    cached_count_arrangements,
                );
            }
        }

        cached_count_arrangements.insert(cached_count_arrangements_key, count_arrangements);
        count_arrangements
    }

    fn solve_problem(lines: Lines, folds: usize) -> u64 {
        let mut sum_of_arrangements = 0;
        let mut all_unknown_and_goal_records = Vec::new();
        for line in lines {
            let (unknown_record, goal_record) = line.split_once(' ').unwrap();
            let mut unknown_record: Vec<_> = unknown_record.chars().collect();
            let mut goal_record: Vec<_> = goal_record
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            let unknown_record_clone = unknown_record.clone();
            let goal_record_clone = goal_record.clone();
            for _ in 0..folds {
                unknown_record.push('?');
                unknown_record.extend(unknown_record_clone.clone());
                goal_record.extend(goal_record_clone.clone());
            }

            all_unknown_and_goal_records.push((unknown_record, goal_record));
        }

        for (unknown_record, goal_record) in &all_unknown_and_goal_records {
            let mut cached_count_arrangements = HashMap::new();
            cached_count_arrangements.reserve(2_000);
            sum_of_arrangements += Self::count_arrangements(
                &unknown_record,
                &goal_record,
                &mut cached_count_arrangements,
            );
        }

        sum_of_arrangements
    }
}

impl Solver for Day12Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day12/input.txt").unwrap();

        let sum_of_arrangements = Self::solve_problem(file.lines(), 0);
        println!("The sum of all arrangements is {sum_of_arrangements}");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day12/input.txt").unwrap();

        let sum_of_arrangements = Self::solve_problem(file.lines(), 4);
        println!("The sum of all arrangements is {sum_of_arrangements}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let file = std::fs::read_to_string("src/day12/input.txt").unwrap();

        b.iter(|| Day12Solver::solve_problem(file.lines(), 4));
    }
}
