use crate::solver::Solver;

const IMAGE_SIZE: usize = 140;

#[derive(Debug)]
struct Galaxy {
    row: usize,
    col: usize,
}

pub struct Day11Solver {}

impl Day11Solver {
    fn find_sum_of_distances_between_expanded_galaxies(expansion_factor: usize) -> usize {
        let file = std::fs::read_to_string("src/day11/input.txt").unwrap();

        let mut empty_row = [true; IMAGE_SIZE];
        let mut empty_col = [true; IMAGE_SIZE];
        let mut galaxies = Vec::new();
        for (row, line) in file.lines().enumerate() {
            for (col, character) in line.chars().enumerate() {
                if character != '#' {
                    continue;
                }

                empty_row[row] = false;
                empty_col[col] = false;
                galaxies.push(Galaxy { row, col });
            }
        }

        let rows_to_expand: Vec<_> = empty_row
            .iter()
            .enumerate()
            .filter(|(_, row)| **row)
            .map(|(i, _)| i)
            .collect();
        let cols_to_expand: Vec<_> = empty_col
            .iter()
            .enumerate()
            .filter(|(_, col)| **col)
            .map(|(i, _)| i)
            .collect();

        for galaxy in &mut galaxies {
            let Galaxy { row, col } = galaxy;
            let mut row_expansion = 0;
            let mut col_expansion = 0;
            for row_to_expand in &rows_to_expand {
                if *row > *row_to_expand {
                    row_expansion += 1;
                } else {
                    break;
                }
            }
            for col_to_expand in &cols_to_expand {
                if *col > *col_to_expand {
                    col_expansion += 1;
                } else {
                    break;
                }
            }

            *row += row_expansion * (expansion_factor - 1);
            *col += col_expansion * (expansion_factor - 1);
        }

        let mut sum_of_lengths = 0;
        for i in 0..(galaxies.len() - 1) {
            for j in (i + 1)..galaxies.len() {
                let Galaxy {
                    row: first_galaxy_row,
                    col: first_galaxy_col,
                } = galaxies[i];
                let Galaxy {
                    row: second_galaxy_row,
                    col: second_galaxy_col,
                } = galaxies[j];

                let manhattan_distance = first_galaxy_row.abs_diff(second_galaxy_row)
                    + first_galaxy_col.abs_diff(second_galaxy_col);
                sum_of_lengths += manhattan_distance;
            }
        }

        sum_of_lengths
    }
}

impl Solver for Day11Solver {
    fn solve_part1() {
        println!(
            "The sum of the lengths between all pairs of galaxies including expansion is {}",
            Self::find_sum_of_distances_between_expanded_galaxies(2)
        );
    }

    fn solve_part2() {
        println!(
            "The sum of the lengths between all pairs of galaxies including expansion is {}",
            Self::find_sum_of_distances_between_expanded_galaxies(1_000_000)
        );
    }
}
