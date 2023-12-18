use std::str::Lines;

use crate::solver::Solver;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ReflectionLine {
    orientation: Orientation,
    index: usize,
}

impl ReflectionLine {
    fn value(&self) -> u64 {
        match self.orientation {
            Orientation::Horizontal => 100 * self.index as u64,
            Orientation::Vertical => self.index as u64,
        }
    }
}

#[derive(Debug)]
struct Pattern {
    pattern: Vec<Vec<char>>,
    transposed_pattern: Vec<Vec<char>>,
}

impl Pattern {
    // This assumes a rectangular matrix.
    fn new(pattern: Vec<Vec<char>>) -> Self {
        let num_transposed_rows = pattern[0].len();
        let num_transposed_cols = pattern.len();
        let mut transposed_pattern =
            vec![vec![char::default(); num_transposed_cols]; num_transposed_rows];

        for i in 0..num_transposed_rows {
            for j in 0..num_transposed_cols {
                transposed_pattern[i][j] = pattern[j][i];
            }
        }

        Self {
            pattern,
            transposed_pattern,
        }
    }

    fn flip_smudge(&mut self, row: usize, col: usize) {
        let smudge = self.pattern[row][col];
        if smudge == '#' {
            self.pattern[row][col] = '.';
            self.transposed_pattern[col][row] = '.';
        } else {
            self.pattern[row][col] = '#';
            self.transposed_pattern[col][row] = '#';
        }
    }

    fn find_reflections(pattern: &Vec<Vec<char>>) -> Vec<usize> {
        let num_cols = pattern[0].len();

        let mut reflections = Vec::new();
        'reflection: for reflection in 1..num_cols {
            let left_reflection_size = reflection;
            let right_reflection_size = num_cols - left_reflection_size;
            let min_reflection_size = left_reflection_size.min(right_reflection_size);

            for row in pattern {
                let left_reflection = &row[..reflection];
                let right_reflection = &row[reflection..];
                let left_reflection =
                    &left_reflection[left_reflection_size - min_reflection_size..];
                let right_reflection = &right_reflection[..min_reflection_size];

                if !left_reflection
                    .iter()
                    .rev()
                    .zip(right_reflection.iter())
                    .all(|(left_item, right_item)| left_item == right_item)
                {
                    continue 'reflection;
                }
            }

            // Found a reflection.
            reflections.push(reflection);
        }

        reflections
    }

    fn summarize(&self) -> Option<ReflectionLine> {
        self.summarize_ignoring(None)
    }

    fn summarize_ignoring(
        &self,
        ignore_reflection_line: Option<ReflectionLine>,
    ) -> Option<ReflectionLine> {
        for vertical_reflection in Self::find_reflections(&self.pattern) {
            let found_reflection = ReflectionLine {
                orientation: Orientation::Vertical,
                index: vertical_reflection,
            };

            match ignore_reflection_line {
                Some(ignore_reflection_line) if ignore_reflection_line == found_reflection => {
                    continue;
                }
                _ => return Some(found_reflection),
            }
        }

        for horizontal_reflection in Self::find_reflections(&self.transposed_pattern) {
            let found_reflection = ReflectionLine {
                orientation: Orientation::Horizontal,
                index: horizontal_reflection,
            };

            match ignore_reflection_line {
                Some(ignore_reflection_line) if ignore_reflection_line == found_reflection => {
                    continue;
                }
                _ => return Some(found_reflection),
            }
        }

        None
    }

    fn summarize_after_smudge_fix(&mut self) -> ReflectionLine {
        let initial_reflection_line = self.summarize().unwrap();

        for i in 0..self.pattern.len() {
            for j in 0..self.pattern[0].len() {
                self.flip_smudge(i, j);
                let new_summary = self.summarize_ignoring(Some(initial_reflection_line));
                // Flip it back, so that the pattern is back to its initial state.
                self.flip_smudge(i, j);

                if let Some(new_reflection_line) = new_summary {
                    return new_reflection_line;
                }
            }
        }

        unreachable!()
    }
}

pub struct Day13Solver {}

impl Day13Solver {
    fn generate_patterns_list(lines: Lines) -> Vec<Pattern> {
        let mut patterns = Vec::new();
        let mut curr_pattern = Vec::new();
        for line in lines {
            if line.is_empty() {
                patterns.push(curr_pattern.clone());
                curr_pattern.clear();
                continue;
            }

            curr_pattern.push(line.chars().collect::<Vec<_>>());
        }
        patterns.push(curr_pattern);

        patterns.into_iter().map(Pattern::new).collect()
    }
}

impl Solver for Day13Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day13/input.txt").unwrap();

        let patterns = Self::generate_patterns_list(file.lines());
        let mut sum_of_summaries = 0;
        for pattern in patterns {
            sum_of_summaries += pattern.summarize().unwrap().value();
        }

        println!("The sum of all summaries is {sum_of_summaries}");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day13/input.txt").unwrap();

        let patterns = Self::generate_patterns_list(file.lines());
        let mut sum_of_summaries = 0;
        for mut pattern in patterns {
            sum_of_summaries += pattern.summarize_after_smudge_fix().value();
        }

        println!("The sum of all summaries is {sum_of_summaries}");
    }
}
