use std::str::Lines;

use crate::solver::Solver;

const GRID_SIZE: usize = 140;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthAndEast,
            'J' => Pipe::NorthAndWest,
            '7' => Pipe::SouthAndWest,
            'F' => Pipe::SouthAndEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => unreachable!(),
        }
    }
}

// The top-left point of the grid is set as the origin (0,0). Going down increments the row
// (e.g. (1,0), (2,0), etc.), while going to the right increments the column
// (e.g. (0,1), (0,2), etc.).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        if row >= GRID_SIZE || col >= GRID_SIZE {
            unreachable!()
        } else {
            Self { row, col }
        }
    }
}

macro_rules! pos {
    ($row:expr, $col:expr) => {
        Position::new($row, $col)
    };
}

struct Tile {
    pipe: Pipe,
    pos: Position,
}

impl Tile {
    fn new(pipe: Pipe, pos: Position) -> Self {
        Self { pipe, pos }
    }

    fn is_start_tile(&self) -> bool {
        self.pipe == Pipe::Start
    }

    // Given a "from" position (which should be adjacential to the current tile),
    // returns the next tile we would connect to if we went from "from" to the current tile.
    // There can be at most one valid next position, but sometimes there is none if the
    // "from" position cannot legally connect to the current tile.
    //
    // Important: do not call if the tile is a start tile, as the start tile doesn't know
    // where it actually connects. If you do so, this will panic. First call `is_start_tile`
    // before this to verify.
    fn try_connect_to_next_tile_from(&self, from: Position) -> Option<Position> {
        match self.pipe {
            Pipe::Vertical => {
                if self.pos.row == 0
                    || self.pos.row == GRID_SIZE - 1
                    || self.pos.col != from.col
                    || (self.pos.row - 1 != from.row && self.pos.row + 1 != from.row)
                {
                    None
                } else if self.pos.row + 1 == from.row {
                    Some(pos! {
                        self.pos.row - 1,
                        self.pos.col
                    })
                } else {
                    Some(pos! {
                        self.pos.row + 1,
                        self.pos.col
                    })
                }
            }
            Pipe::Horizontal => {
                if self.pos.col == 0
                    || self.pos.col == GRID_SIZE - 1
                    || self.pos.row != from.row
                    || (self.pos.col - 1 != from.col && self.pos.col + 1 != from.col)
                {
                    None
                } else if self.pos.col + 1 == from.col {
                    Some(pos! {
                        self.pos.row,
                        self.pos.col - 1
                    })
                } else {
                    Some(pos! {
                        self.pos.row,
                        self.pos.col + 1
                    })
                }
            }
            Pipe::NorthAndEast => {
                todo!()
            }
            Pipe::NorthAndWest => {
                todo!()
            }
            Pipe::SouthAndWest => {
                todo!()
            }
            Pipe::SouthAndEast => {
                todo!()
            }
            // Ground connects to nothing.
            Pipe::Ground => None,
            Pipe::Start => panic!(
                "A Start tile doesn't know where it connects to, so calling this function panics."
            ),
        }
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    start_pos: Position,
}

impl Grid {
    fn new(lines: Lines) -> Self {
        let mut start_pos = None;
        let tiles = lines
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, character)| {
                        let pipe = character.into();
                        let pos = pos! {row, col};
                        if pipe == Pipe::Start {
                            start_pos = Some(pos);
                        }

                        Tile::new(pipe, pos)
                    })
                    .collect()
            })
            .collect();

        let start_pos = start_pos.unwrap();
        Grid { tiles, start_pos }
    }

    fn get_cell(&self, pos: Position) -> &Tile {
        &self.tiles[pos.row][pos.col]
    }
}

pub struct Day10Solver {}

impl Solver for Day10Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day10/input.txt").unwrap();
        let grid = Grid::new(file.lines());
    }

    fn solve_part2() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn position_beyond_row_bound_panics() {
        Position::new(GRID_SIZE, 0);
    }

    #[test]
    #[should_panic]
    fn position_beyond_column_bound_panics() {
        Position::new(0, GRID_SIZE);
    }

    #[test]
    fn vertical_tile_at_origin_never_connects() {
        let tile = Tile::new('|'.into(), pos! { 0, 0 });
        assert!(tile.try_connect_to_next_tile_from(pos! {0, 0}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {0, 1}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {0, 2}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {1, 0}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {2, 0}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {1, 1}).is_none());
    }

    #[test]
    fn vertical_tile_at_last_row_never_connects() {
        let tile = Tile::new('|'.into(), pos! { GRID_SIZE-1, 1 });
        assert!(tile
            .try_connect_to_next_tile_from(pos! {GRID_SIZE-1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(pos! {GRID_SIZE-2, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(pos! {GRID_SIZE-3, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(pos! {GRID_SIZE-1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(pos! {GRID_SIZE-1, 2})
            .is_none());
    }

    #[test]
    fn vertical_tile_in_valid_position_connects_sometimes() {
        let tile = Tile::new('|'.into(), pos! { 2, 2 });
        // Two valid connection positions.
        assert_eq!(
            tile.try_connect_to_next_tile_from(pos! {1, 2}),
            Some(pos! {3, 2})
        );
        assert_eq!(
            tile.try_connect_to_next_tile_from(pos! {3, 2}),
            Some(pos! {1, 2})
        );

        // Rest are invalid positions to connect from.
        assert!(tile.try_connect_to_next_tile_from(pos! {2, 2}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {2, 1}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {2, 3}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {1, 1}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {0, 2}).is_none());
    }

    #[test]
    fn ground_tile_never_connects() {
        let tile = Tile::new('.'.into(), pos! { 2, 2 });
        assert!(tile.try_connect_to_next_tile_from(pos! {2, 2}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {2, 1}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {2, 3}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {1, 1}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {3, 2}).is_none());
        assert!(tile.try_connect_to_next_tile_from(pos! {0, 0}).is_none());
    }

    #[test]
    #[should_panic]
    fn start_tile_connection_panics() {
        let tile = Tile::new('S'.into(), pos! { 2, 2 });
        tile.try_connect_to_next_tile_from(pos! {2, 2});
    }
}
