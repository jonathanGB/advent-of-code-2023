use smallvec::SmallVec;
use std::str::Lines;

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

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Self::Up => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Up,
            Self::Left => Self::Right,
        }
    }
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
    energized: SmallVec<[Direction; std::mem::variant_count::<Direction>()]>,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        let cell_type = value.into();
        Self {
            cell_type,
            energized: SmallVec::new(),
        }
    }
}

impl Cell {
    fn interact(&self, in_light_beam: LightBeam) -> SmallVec<[LightBeam; 2]> {
        let in_direction = in_light_beam.direction;
        let (first_out_direction, second_out_direction) = match self.cell_type {
            CellType::EmptySpace => (in_direction, None),
            CellType::LeftMirror => (Self::interact_left_mirror(in_direction), None),
            CellType::RightMirror => (Self::interact_left_mirror(in_direction).opposite(), None),
            CellType::HorizontalSplitter => Self::interact_horizontal_splitter(in_direction),
            CellType::VerticalSplitter => Self::interact_vertical_splitter(in_direction),
        };

        let mut out_light_beams = SmallVec::new();
        if let Some(out_light_beam) =
            LightBeam::try_out(in_light_beam.position, first_out_direction)
        {
            out_light_beams.push(out_light_beam);
        }

        if second_out_direction.is_none() {
            return out_light_beams;
        }

        if let Some(out_light_beam) =
            LightBeam::try_out(in_light_beam.position, second_out_direction.unwrap())
        {
            out_light_beams.push(out_light_beam);
        }

        out_light_beams
    }

    fn interact_left_mirror(in_direction: Direction) -> Direction {
        match in_direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Right,
            Direction::Left => Direction::Up,
        }
    }

    fn interact_horizontal_splitter(in_direction: Direction) -> (Direction, Option<Direction>) {
        match in_direction {
            Direction::Left | Direction::Right => (in_direction, None),
            Direction::Up | Direction::Bottom => (Direction::Left, Some(Direction::Right)),
        }
    }

    fn interact_vertical_splitter(in_direction: Direction) -> (Direction, Option<Direction>) {
        match in_direction {
            Direction::Up | Direction::Bottom => (in_direction, None),
            Direction::Left | Direction::Right => (Direction::Up, Some(Direction::Bottom)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct LightBeam {
    position: Position,
    direction: Direction,
}

impl LightBeam {
    fn try_out(in_position: Position, out_direction: Direction) -> Option<Self> {
        let Position {
            row: in_row,
            col: in_col,
        } = in_position;

        match out_direction {
            Direction::Up => {
                if in_row == 0 {
                    None
                } else {
                    Some(Self {
                        position: pos!(in_row - 1, in_col),
                        direction: out_direction,
                    })
                }
            }
            Direction::Right => {
                if in_col == GRID_SIZE - 1 {
                    None
                } else {
                    Some(Self {
                        position: pos!(in_row, in_col + 1),
                        direction: out_direction,
                    })
                }
            }
            Direction::Bottom => {
                if in_row == GRID_SIZE - 1 {
                    None
                } else {
                    Some(Self {
                        position: pos!(in_row + 1, in_col),
                        direction: out_direction,
                    })
                }
            }
            Direction::Left => {
                if in_col == 0 {
                    None
                } else {
                    Some(Self {
                        position: pos!(in_row, in_col - 1),
                        direction: out_direction,
                    })
                }
            }
        }
    }
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

            cell.energized.push(in_light_beam.direction);
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
