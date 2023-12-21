use smallvec::SmallVec;
use std::{collections::HashSet, str::Lines};

use crate::solver::Solver;

const GRID_SIZE: usize = 110;

#[derive(Clone, Copy, Debug)]
enum CellType {
    EmptySpace,
    LeftMirror,
    RightMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::EmptySpace,
            '\\' => Self::LeftMirror,
            '/' => Self::RightMirror,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Bottom,
    Left,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

macro_rules! pos {
    ($row:expr, $col:expr) => {
        Position {
            row: $row,
            col: $col,
        }
    };
}

#[derive(Debug)]
struct Cell {
    cell_type: CellType,
    energized: HashSet<Direction>,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        let cell_type = value.into();
        Self {
            cell_type,
            energized: HashSet::new(),
        }
    }
}

impl Cell {
    fn interact(&self, in_light_beam: LightBeam) -> SmallVec<[LightBeam; 2]> {
        match self.cell_type {
            CellType::EmptySpace => Self::interact_empty_space(in_light_beam),
            CellType::LeftMirror => Self::interact_left_mirror(in_light_beam),
            CellType::RightMirror => Self::interact_right_mirror(in_light_beam),
            CellType::HorizontalSplitter => Self::interact_horizontal_splitter(in_light_beam),
            CellType::VerticalSplitter => Self::interact_vertical_splitter(in_light_beam),
        }
    }

    fn interact_empty_space(in_light_beam: LightBeam) -> SmallVec<[LightBeam; 2]> {
        let Position { row, col } = in_light_beam.position;
        let in_direction = in_light_beam.direction;
        match in_direction {
            Direction::Up => {
                if let Some(new_row) = row.checked_sub(1) {
                    Some(pos!(new_row, col))
                } else {
                    None
                }
            }
            Direction::Right => {
                if col + 1 < GRID_SIZE {
                    Some(pos!(row, col + 1))
                } else {
                    None
                }
            }
            Direction::Bottom => {
                if row + 1 < GRID_SIZE {
                    Some(pos!(row + 1, col))
                } else {
                    None
                }
            }
            Direction::Left => {
                if let Some(new_col) = col.checked_sub(1) {
                    Some(pos!(row, new_col))
                } else {
                    None
                }
            }
        }
        .into_iter()
        .map(|new_position| LightBeam {
            position: new_position,
            direction: in_direction,
        })
        .collect()
    }

    fn interact_left_mirror(in_light_beam: LightBeam) -> SmallVec<[LightBeam; 2]> {
        let Position { row, col } = in_light_beam.position;
        let in_direction = in_light_beam.direction;
        let mut out_light_beam = SmallVec::new();

        match in_direction {
            Direction::Up => {
                if let Some(new_col) = col.checked_sub(1) {
                    let new_direction = Direction::Left;

                    out_light_beam.push(LightBeam {
                        position: pos!(row, new_col),
                        direction: new_direction,
                    });
                }
            }
            Direction::Right => {
                if row + 1 < GRID_SIZE {
                    let new_direction = Direction::Bottom;

                    out_light_beam.push(LightBeam {
                        position: pos!(row + 1, col),
                        direction: new_direction,
                    });
                }
            }
            Direction::Bottom => {
                if col + 1 < GRID_SIZE {
                    let new_direction = Direction::Right;

                    out_light_beam.push(LightBeam {
                        position: pos!(row, col + 1),
                        direction: new_direction,
                    });
                }
            }
            Direction::Left => {
                if let Some(new_row) = row.checked_sub(1) {
                    let new_direction = Direction::Up;

                    out_light_beam.push(LightBeam {
                        position: pos!(new_row, col),
                        direction: new_direction,
                    });
                }
            }
        }

        out_light_beam
    }

    fn interact_right_mirror(in_light_beam: LightBeam) -> SmallVec<[LightBeam; 2]> {
        let Position { row, col } = in_light_beam.position;
        let in_direction = in_light_beam.direction;
        let mut out_light_beam = SmallVec::new();

        match in_direction {
            Direction::Up => {
                if col + 1 < GRID_SIZE {
                    let new_direction = Direction::Right;

                    out_light_beam.push(LightBeam {
                        position: pos!(row, col + 1),
                        direction: new_direction,
                    });
                }
            }
            Direction::Right => {
                if let Some(new_row) = row.checked_sub(1) {
                    let new_direction = Direction::Up;

                    out_light_beam.push(LightBeam {
                        position: pos!(new_row, col),
                        direction: new_direction,
                    });
                }
            }
            Direction::Bottom => {
                if let Some(new_col) = col.checked_sub(1) {
                    let new_direction = Direction::Left;

                    out_light_beam.push(LightBeam {
                        position: pos!(row, new_col),
                        direction: new_direction,
                    });
                }
            }
            Direction::Left => {
                if row + 1 < GRID_SIZE {
                    let new_direction = Direction::Bottom;

                    out_light_beam.push(LightBeam {
                        position: pos!(row + 1, col),
                        direction: new_direction,
                    });
                }
            }
        }

        out_light_beam
    }

    fn interact_horizontal_splitter(in_light_beam: LightBeam) -> SmallVec<[LightBeam; 2]> {
        match in_light_beam.direction {
            Direction::Left | Direction::Right => Self::interact_empty_space(in_light_beam),
            Direction::Up | Direction::Bottom => {
                let mut split: SmallVec<[LightBeam; 2]> = Self::interact_left_mirror(in_light_beam);
                split.extend(Self::interact_right_mirror(in_light_beam));
                split
            }
        }
    }

    fn interact_vertical_splitter(in_light_beam: LightBeam) -> SmallVec<[LightBeam; 2]> {
        match in_light_beam.direction {
            Direction::Up | Direction::Bottom => Self::interact_empty_space(in_light_beam),
            Direction::Left | Direction::Right => {
                let mut split: SmallVec<[LightBeam; 2]> = Self::interact_left_mirror(in_light_beam);
                split.extend(Self::interact_right_mirror(in_light_beam));
                split
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct LightBeam {
    position: Position,
    direction: Direction,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(lines: Lines) -> Self {
        let cells = lines
            .map(|line| line.chars().map(Cell::from).collect())
            .collect();
        Self { cells }
    }

    fn get_cell_mut(&mut self, position: Position) -> &mut Cell {
        &mut self.cells[position.row][position.col]
    }

    fn light_beam(&mut self, light_beam: LightBeam) -> usize {
        let mut light_beams = vec![light_beam.clone()];
        let mut num_cells_energized = 0;
        while let Some(in_light_beam) = light_beams.pop() {
            let cell = self.get_cell_mut(in_light_beam.position);
            if cell.energized.contains(&in_light_beam.direction) {
                continue;
            }

            if cell.energized.is_empty() {
                num_cells_energized += 1;
            }

            cell.energized.insert(in_light_beam.direction);
            light_beams.extend(cell.interact(in_light_beam));
        }

        num_cells_energized
    }

    fn de_energize(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                cell.energized.clear();
            }
        }
    }

    // fn light_beam_rec(&mut self, in_light_beam: LightBeam) -> usize {
    //     let cell = self.get_cell_mut(in_light_beam.position);

    //     if cell.energized.contains(&in_light_beam.direction) {
    //         return 0;
    //     }

    //     let mut num_cells_energized = 0;
    //     if cell.energized.is_empty() {
    //         num_cells_energized += 1;
    //     }

    //     cell.energized.push(in_light_beam.direction);

    //     for out_light_beam in cell.interact(in_light_beam) {
    //         num_cells_energized += self.light_beam_rec(out_light_beam);
    //     }

    //     num_cells_energized
    // }
}

pub struct Day16Solver {}

impl Day16Solver {
    fn solve_part1_with_lines(lines: Lines) -> usize {
        let mut grid = Grid::new(lines);
        grid.light_beam(LightBeam {
            position: pos!(0, 0),
            direction: Direction::Right,
        })
    }

    fn solve_part2_with_lines(lines: Lines) -> usize {
        let mut max_energized_cells = 0;
        let mut grid = Grid::new(lines);

        for i in 0..GRID_SIZE {
            let top_light_beam = LightBeam {
                position: pos!(GRID_SIZE - 1, i),
                direction: Direction::Up,
            };
            max_energized_cells = max_energized_cells.max(grid.light_beam(top_light_beam));
            grid.de_energize();

            let right_light_beam = LightBeam {
                position: pos!(i, 0),
                direction: Direction::Right,
            };
            max_energized_cells = max_energized_cells.max(grid.light_beam(right_light_beam));
            grid.de_energize();

            let bottom_light_beam = LightBeam {
                position: pos!(0, i),
                direction: Direction::Bottom,
            };
            max_energized_cells = max_energized_cells.max(grid.light_beam(bottom_light_beam));
            grid.de_energize();

            let left_light_beam = LightBeam {
                position: pos!(i, GRID_SIZE - 1),
                direction: Direction::Left,
            };
            max_energized_cells = max_energized_cells.max(grid.light_beam(left_light_beam));
            grid.de_energize();
        }

        max_energized_cells
    }
}

impl Solver for Day16Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day16/input.txt").unwrap();

        let num_energized_cells = Self::solve_part1_with_lines(file.lines());
        println!("There are {num_energized_cells} energized cells.");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day16/input.txt").unwrap();

        let max_energized_cells = Self::solve_part2_with_lines(file.lines());
        println!("The maximum number energized in a given configuration is {max_energized_cells}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let file = std::fs::read_to_string("src/day16/input.txt").unwrap();

        b.iter(|| Day16Solver::solve_part1_with_lines(file.lines()));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let file = std::fs::read_to_string("src/day16/input.txt").unwrap();

        b.iter(|| Day16Solver::solve_part2_with_lines(file.lines()));
    }
}
