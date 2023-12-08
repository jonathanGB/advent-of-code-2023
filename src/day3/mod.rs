use crate::solver::Solver;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

const GRID_SIZE: usize = 140;

#[derive(Debug)]
struct PartNumberCandidate {
    number: u32,
    id: usize,
    valid: bool,
}

impl PartNumberCandidate {
    fn new(number: u32, id: usize) -> Self {
        Self {
            number,
            id,
            valid: false,
        }
    }
}

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

type PartNumberCandidateRow = Vec<Option<Rc<RefCell<PartNumberCandidate>>>>;
type PartNumberCandidateMatrix = Vec<PartNumberCandidateRow>;
type PartNumberCandidatePositions = Vec<Vec<Position>>;

#[derive(Default)]
pub struct Day3Solver {
    part_number_candidate_id: usize,
}

impl Day3Solver {
    fn maybe_add_part_number_candidate(
        &mut self,
        part_number_candidate_start: &mut Option<usize>,
        col: usize,
        line: &str,
        part_number_candidate_row: &mut PartNumberCandidateRow,
    ) {
        if let Some(start_col) = part_number_candidate_start {
            let candidate_num: u32 = line[*start_col..col].parse().unwrap();
            let part_number_candidate = Rc::new(RefCell::new(PartNumberCandidate::new(
                candidate_num,
                self.part_number_candidate_id,
            )));
            self.part_number_candidate_id += 1;

            for _ in *start_col..col {
                part_number_candidate_row.push(Some(part_number_candidate.clone()));
            }

            *part_number_candidate_start = None;
        }
    }

    fn add_adjacent_positions(
        all_adjacent_positions: &mut PartNumberCandidatePositions,
        position: Position,
    ) {
        let Position { row, col } = position;
        let first_row = row == 0;
        let first_col = col == 0;
        let last_row = row == (GRID_SIZE - 1);
        let last_col = col == (GRID_SIZE - 1);

        let mut adjacent_positions = Vec::new();
        if !first_row {
            if !first_col {
                adjacent_positions.push(Position {
                    row: row - 1,
                    col: col - 1,
                });
            }

            adjacent_positions.push(Position { row: row - 1, col });

            if !last_col {
                adjacent_positions.push(Position {
                    row: row - 1,
                    col: col + 1,
                });
            }
        }

        if !first_col {
            adjacent_positions.push(Position { row, col: col - 1 });
        }

        if !last_col {
            adjacent_positions.push(Position { row, col: col + 1 });
        }

        if !last_row {
            if !first_col {
                adjacent_positions.push(Position {
                    row: row + 1,
                    col: col - 1,
                });
            }

            adjacent_positions.push(Position { row: row + 1, col });

            if !last_col {
                adjacent_positions.push(Position {
                    row: row + 1,
                    col: col + 1,
                });
            }
        }

        all_adjacent_positions.push(adjacent_positions);
    }

    fn build_part_number_candidates_map_and_positions(
        &mut self,
        file: String,
        symbol_filter: Option<char>,
    ) -> (PartNumberCandidateMatrix, PartNumberCandidatePositions) {
        let mut part_number_candidate_matrix = PartNumberCandidateMatrix::new();
        let mut part_number_candidate_positions = PartNumberCandidatePositions::new();

        for (row, line) in file.lines().enumerate() {
            let mut part_number_candidate_row = Vec::new();
            let mut part_number_candidate_start = None;

            for (col, character) in line.chars().enumerate() {
                if character.is_ascii_digit() {
                    if part_number_candidate_start.is_some() {
                        continue;
                    }
                    part_number_candidate_start = Some(col);
                } else {
                    // If we were still processing a candidate and we hit
                    // a character that is not a digt, then we should create
                    // a new candidate.
                    self.maybe_add_part_number_candidate(
                        &mut part_number_candidate_start,
                        col,
                        line,
                        &mut part_number_candidate_row,
                    );

                    // The current character is definitely not a part number,
                    // so we can just insert None.
                    part_number_candidate_row.push(None);

                    let add_adjacent_positions = match symbol_filter {
                        Some(symbol_filter) if symbol_filter == character => true,
                        None if character != '.' => true,
                        _ => false,
                    };

                    if add_adjacent_positions {
                        Self::add_adjacent_positions(
                            &mut part_number_candidate_positions,
                            Position { row, col },
                        );
                    }
                }
            }

            // If a candidate ends the line, we should add it.
            self.maybe_add_part_number_candidate(
                &mut part_number_candidate_start,
                line.len(),
                line,
                &mut part_number_candidate_row,
            );
            part_number_candidate_matrix.push(part_number_candidate_row);
        }

        (
            part_number_candidate_matrix,
            part_number_candidate_positions,
        )
    }
}

impl Solver for Day3Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day3/input.txt").unwrap();

        let mut solver = Day3Solver::default();
        let (part_number_candidate_matrix, part_number_candidate_positions) =
            solver.build_part_number_candidates_map_and_positions(file, None);

        let mut sum_part_numbers = 0;
        for Position { row, col } in part_number_candidate_positions.into_iter().flatten() {
            if let Some(part_number_candidate) = &part_number_candidate_matrix[row][col] {
                let mut part_number_candidate = part_number_candidate.borrow_mut();
                if part_number_candidate.valid {
                    continue;
                }

                sum_part_numbers += part_number_candidate.number;
                part_number_candidate.valid = true;
            }
        }

        println!("The sum of all part numbers is {sum_part_numbers}.");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day3/input.txt").unwrap();

        let mut solver = Day3Solver::default();
        let (part_number_candidate_matrix, part_number_candidate_positions) =
            solver.build_part_number_candidates_map_and_positions(file, Some('*'));

        let mut sum_gear_ratios = 0;
        for gear_adjacent_positions in part_number_candidate_positions.into_iter() {
            let mut num_adjacent_candidates = 0;
            let mut gear_ratio = 1;
            let mut unique_gear_parts = HashSet::new();
            for Position { row, col } in gear_adjacent_positions {
                if let Some(gear_part_number) = &part_number_candidate_matrix[row][col] {
                    let gear_part_number = gear_part_number.borrow();

                    if !unique_gear_parts.insert(gear_part_number.id) {
                        continue;
                    }
                    num_adjacent_candidates += 1;
                    gear_ratio *= gear_part_number.number;
                }
            }

            if num_adjacent_candidates == 2 {
                sum_gear_ratios += gear_ratio;
            }
        }

        println!("The sum of all part numbers is {sum_gear_ratios}.");
    }
}
