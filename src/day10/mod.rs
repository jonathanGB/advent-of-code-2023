use std::str::Lines;

use crate::solver::Solver;

const TILE_GRID_SIZE: usize = 140;
// Conversion from tile grid to space grid is 2x + 1.
const SPACE_GRID_SIZE: usize = 281;

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

// The top-left point of the tile grid is set as the origin (0,0). Going down increments the row
// (e.g. (1,0), (2,0), etc.), while going to the right increments the column
// (e.g. (0,1), (0,2), etc.).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TilePosition {
    row: usize,
    col: usize,
}

impl TilePosition {
    fn new(row: usize, col: usize) -> Self {
        if row >= TILE_GRID_SIZE || col >= TILE_GRID_SIZE {
            unreachable!()
        } else {
            Self { row, col }
        }
    }
}

macro_rules! tile_pos {
    ($row:expr, $col:expr) => {
        TilePosition::new($row, $col)
    };
}

#[derive(Debug)]
struct Tile {
    pipe: Pipe,
    pos: TilePosition,
}

impl Tile {
    fn new(pipe: Pipe, pos: TilePosition) -> Self {
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
    fn try_connect_to_next_tile_from(&self, from: TilePosition) -> Option<TilePosition> {
        match self.pipe {
            Pipe::Vertical => {
                if self.pos.row == 0
                    || self.pos.row == TILE_GRID_SIZE - 1
                    || self.pos.col != from.col
                    || (self.pos.row - 1 != from.row && self.pos.row + 1 != from.row)
                {
                    None
                } else if self.pos.row + 1 == from.row {
                    Some(tile_pos! { self.pos.row - 1, self.pos.col })
                } else {
                    Some(tile_pos! { self.pos.row + 1, self.pos.col })
                }
            }
            Pipe::Horizontal => {
                if self.pos.col == 0
                    || self.pos.col == TILE_GRID_SIZE - 1
                    || self.pos.row != from.row
                    || (self.pos.col - 1 != from.col && self.pos.col + 1 != from.col)
                {
                    None
                } else if self.pos.col + 1 == from.col {
                    Some(tile_pos! { self.pos.row, self.pos.col - 1 })
                } else {
                    Some(tile_pos! { self.pos.row, self.pos.col + 1 })
                }
            }
            Pipe::NorthAndEast => {
                if self.pos.row == 0 || self.pos.col == TILE_GRID_SIZE - 1 {
                    None
                } else if self.pos.row - 1 == from.row && self.pos.col == from.col {
                    Some(tile_pos! { self.pos.row, self.pos.col + 1 })
                } else if self.pos.row == from.row && self.pos.col + 1 == from.col {
                    Some(tile_pos! { self.pos.row - 1, self.pos.col})
                } else {
                    None
                }
            }
            Pipe::NorthAndWest => {
                if self.pos.row == 0 || self.pos.col == 0 {
                    None
                } else if self.pos.row - 1 == from.row && self.pos.col == from.col {
                    Some(tile_pos! { self.pos.row, self.pos.col - 1 })
                } else if self.pos.row == from.row && self.pos.col - 1 == from.col {
                    Some(tile_pos! { self.pos.row - 1, self.pos.col})
                } else {
                    None
                }
            }
            Pipe::SouthAndWest => {
                if self.pos.row == TILE_GRID_SIZE - 1 || self.pos.col == 0 {
                    None
                } else if self.pos.row + 1 == from.row && self.pos.col == from.col {
                    Some(tile_pos! { self.pos.row, self.pos.col - 1 })
                } else if self.pos.row == from.row && self.pos.col - 1 == from.col {
                    Some(tile_pos! { self.pos.row + 1, self.pos.col})
                } else {
                    None
                }
            }
            Pipe::SouthAndEast => {
                if self.pos.row == TILE_GRID_SIZE - 1 || self.pos.col == TILE_GRID_SIZE - 1 {
                    None
                } else if self.pos.row + 1 == from.row && self.pos.col == from.col {
                    Some(tile_pos! { self.pos.row, self.pos.col + 1 })
                } else if self.pos.row == from.row && self.pos.col + 1 == from.col {
                    Some(tile_pos! { self.pos.row + 1, self.pos.col})
                } else {
                    None
                }
            }
            // Ground connects to nothing.
            Pipe::Ground => None,
            Pipe::Start => panic!(
                "A Start tile doesn't know where it connects to, so calling this function panics."
            ),
        }
    }
}

#[derive(Debug)]
struct TileGrid {
    tiles: Vec<Vec<Tile>>,
    start_pos: TilePosition,
}

impl TileGrid {
    fn new(lines: Lines) -> Self {
        let mut start_pos = None;
        let tiles = lines
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, character)| {
                        let pipe = character.into();
                        let pos = tile_pos! {row, col};
                        if pipe == Pipe::Start {
                            start_pos = Some(pos);
                        }

                        Tile::new(pipe, pos)
                    })
                    .collect()
            })
            .collect();

        let start_pos = start_pos.unwrap();
        TileGrid { tiles, start_pos }
    }

    fn get_tile(&self, pos: TilePosition) -> &Tile {
        &self.tiles[pos.row][pos.col]
    }

    fn explore_loop(&self) -> Vec<TilePosition> {
        let TilePosition {
            row: start_row,
            col: start_col,
        } = self.start_pos;
        // This is safe as long as the start of the loop is not on the
        // first nor last row/col.
        let explore_paths = [
            tile_pos! { start_row - 1, start_col},
            tile_pos! { start_row, start_col + 1 },
            tile_pos! { start_row + 1, start_col },
            tile_pos! { start_row, start_col - 1},
        ];

        for mut next_pos in explore_paths {
            let mut curr_pos = self.start_pos;
            let mut visited_loop = vec![curr_pos];

            loop {
                let next_tile = self.get_tile(next_pos);
                if next_tile.is_start_tile() {
                    visited_loop.push(next_pos);
                    return visited_loop;
                }

                match next_tile.try_connect_to_next_tile_from(curr_pos) {
                    Some(connected_pos) => {
                        visited_loop.push(next_pos);
                        curr_pos = next_pos;
                        next_pos = connected_pos;
                    }
                    None => break,
                }
            }
        }

        unreachable!()
    }
}

// The top-left point of the tile grid is set as the origin (0,0). Going down increments the row
// (e.g. (1,0), (2,0), etc.), while going to the right increments the column
// (e.g. (0,1), (0,2), etc.). Note that not every space maps to the position of a tile: each space
// can either be a tile location, or between two tiles. All even rows and columns are between-tile
// spaces, whereas all positions which have both odd rows and columns are tile locations. Hence,
// why converting from a `TilePosition` to a `SpacePosition` follows a 2x+1 function.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SpacePosition {
    row: usize,
    col: usize,
}

impl From<TilePosition> for SpacePosition {
    fn from(value: TilePosition) -> Self {
        let TilePosition { row, col } = value;
        Self {
            row: 2 * row + 1,
            col: 2 * col + 1,
        }
    }
}

impl SpacePosition {
    fn new(row: usize, col: usize) -> Self {
        if row >= SPACE_GRID_SIZE || col >= SPACE_GRID_SIZE {
            unreachable!()
        } else {
            Self { row, col }
        }
    }

    // Returns the average position in terms of rows and columns.
    // In practice, this is used to find the non-tile position between two
    // contiguous tile positions.
    fn space_position_between(&self, other: SpacePosition) -> SpacePosition {
        let row = (self.row + other.row) / 2;
        let col = (self.col + other.col) / 2;

        Self { row, col }
    }
}

macro_rules! space_pos {
    ($row:expr, $col:expr) => {
        SpacePosition::new($row, $col)
    };
}

#[derive(Copy, Clone, PartialEq)]
enum SpaceState {
    LoopTile,
    NonLoopTile { visited: bool },
    InBetween { visited: bool },
}

impl Default for SpaceState {
    fn default() -> Self {
        SpaceState::InBetween { visited: false }
    }
}

impl SpaceState {
    fn is_part_of_loop(&self) -> bool {
        *self == Self::LoopTile
    }

    fn is_unvisited_non_loop_tile(&self) -> bool {
        match self {
            Self::NonLoopTile { visited } if !*visited => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct Space {
    state: SpaceState,
}

struct SpaceGrid {
    spaces: Vec<Vec<Space>>,
}

impl Default for SpaceGrid {
    fn default() -> Self {
        Self {
            spaces: vec![vec![Space::default(); SPACE_GRID_SIZE]; SPACE_GRID_SIZE],
        }
    }
}

impl SpaceGrid {
    fn new(explored_loop: &Vec<TilePosition>) -> Self {
        let mut grid = SpaceGrid::default();
        for two_contiguous_loop_tile_positions in explored_loop.windows(2) {
            let first_contiguous_loop_space_position = two_contiguous_loop_tile_positions[0].into();
            let second_contiguous_loop_space_position =
                two_contiguous_loop_tile_positions[1].into();

            // Marks the tile and the space between this tile and the next tile as being
            // spaces covered by the loop. The latter is subtler yet crucial to later on
            // explore the space that squeezes in between pipes that are part of the loop.
            grid.get_mut_space(first_contiguous_loop_space_position)
                .state = SpaceState::LoopTile;
            grid.get_mut_space(
                first_contiguous_loop_space_position
                    .space_position_between(second_contiguous_loop_space_position),
            )
            .state = SpaceState::LoopTile;
        }

        // We step through the grid by steps of 2 and starting at row/col 1 to iterate through
        // actual tiles, i.e. strictly odd rows/columns.
        for i in (1..SPACE_GRID_SIZE).step_by(2) {
            for j in (1..SPACE_GRID_SIZE).step_by(2) {
                let space = grid.get_mut_space(space_pos! { i, j });
                // Ignore tiles that were previously tagged as being part of the loop.
                if !space.state.is_part_of_loop() {
                    space.state = SpaceState::NonLoopTile { visited: false };
                }
            }
        }

        grid
    }

    fn visit_all_spaces_outside_of_the_loop(&mut self) {
        // We know that the space (0,0) is unvisited, and that it is not part of the loop.
        // This will be our starting point to visit all spaces outside of the loop.
        // This will only contain positions for spaces that are not covered by the loop.
        let mut space_positions_to_visit = vec![space_pos! { 0, 0 }];

        while let Some(space_position) = space_positions_to_visit.pop() {
            let space = self.get_mut_space(space_position);
            match space.state {
                SpaceState::LoopTile => continue,
                SpaceState::NonLoopTile { ref mut visited } => {
                    if *visited {
                        continue;
                    }
                    *visited = true;
                }
                SpaceState::InBetween { ref mut visited } => {
                    if *visited {
                        continue;
                    }
                    *visited = true;
                }
            }

            let SpacePosition { row, col } = space_position;
            if row > 0 {
                space_positions_to_visit.push(space_pos! { row-1, col });
            }
            if col < SPACE_GRID_SIZE - 1 {
                space_positions_to_visit.push(space_pos! { row, col+1 });
            }
            if row < SPACE_GRID_SIZE - 1 {
                space_positions_to_visit.push(space_pos! { row+1, col });
            }
            if col > 0 {
                space_positions_to_visit.push(space_pos! { row, col-1 });
            }
        }
    }

    fn get_mut_space(&mut self, pos: SpacePosition) -> &mut Space {
        &mut self.spaces[pos.row][pos.col]
    }
}

pub struct Day10Solver {}

impl Solver for Day10Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day10/input.txt").unwrap();
        let grid = TileGrid::new(file.lines());
        let explored_loop = grid.explore_loop();
        println!(
            "The farthest point along the loop from the starting position is {}",
            explored_loop.len() / 2
        );
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day10/input.txt").unwrap();
        let tile_grid = TileGrid::new(file.lines());
        let explored_loop = tile_grid.explore_loop();

        let mut space_grid = SpaceGrid::new(&explored_loop);
        space_grid.visit_all_spaces_outside_of_the_loop();

        let num_enclosed_tiles: usize = space_grid
            .spaces
            .iter()
            .map(|row_of_spaces| {
                row_of_spaces
                    .iter()
                    .filter(|space| space.state.is_unvisited_non_loop_tile())
                    .count()
            })
            .sum();

        println!("The number of tiles enclosed by the loop is {num_enclosed_tiles}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn position_beyond_row_bound_panics() {
        TilePosition::new(TILE_GRID_SIZE, 0);
    }

    #[test]
    #[should_panic]
    fn position_beyond_column_bound_panics() {
        TilePosition::new(0, TILE_GRID_SIZE);
    }

    #[test]
    fn vertical_tile_at_origin_never_connects() {
        let tile = Tile::new('|'.into(), tile_pos! { 0, 0 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
    }

    #[test]
    fn vertical_tile_at_last_row_never_connects() {
        let tile = Tile::new('|'.into(), tile_pos! { TILE_GRID_SIZE-1, 1 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-2, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-3, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 2})
            .is_none());
    }

    #[test]
    fn vertical_tile_in_valid_position_connects_sometimes() {
        let tile = Tile::new('|'.into(), tile_pos! { 2, 2 });
        // Two valid connection positions.
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {1, 2}),
            Some(tile_pos! {3, 2})
        );
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {3, 2}),
            Some(tile_pos! {1, 2})
        );

        // Rest are invalid positions to connect from.
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 3})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 2})
            .is_none());
    }

    #[test]
    fn horizontal_tile_at_origin_never_connects() {
        let tile = Tile::new('-'.into(), tile_pos! { 0, 0 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
    }

    #[test]
    fn horizontal_tile_at_last_col_never_connects() {
        let tile = Tile::new('-'.into(), tile_pos! { 1, TILE_GRID_SIZE-1 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, TILE_GRID_SIZE-1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, TILE_GRID_SIZE-2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, TILE_GRID_SIZE-3})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, TILE_GRID_SIZE-1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, TILE_GRID_SIZE-1})
            .is_none());
    }

    #[test]
    fn horizontal_tile_in_valid_position_connects_sometimes() {
        let tile = Tile::new('-'.into(), tile_pos! { 2, 2 });
        // Two valid connection positions.
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {2, 1}),
            Some(tile_pos! {2, 3})
        );
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {2, 3}),
            Some(tile_pos! {2, 1})
        );

        // Rest are invalid positions to connect from.
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {3, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
    }

    #[test]
    fn north_east_tile_at_first_row_never_connects() {
        let tile = Tile::new('L'.into(), tile_pos! { 0, 1 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 2})
            .is_none());
    }

    #[test]
    fn north_east_tile_at_last_col_never_connects() {
        let tile = Tile::new('L'.into(), tile_pos! { 1, TILE_GRID_SIZE-1 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, TILE_GRID_SIZE-1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, TILE_GRID_SIZE-2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, TILE_GRID_SIZE-3})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, TILE_GRID_SIZE-1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, TILE_GRID_SIZE-1})
            .is_none());
    }

    #[test]
    fn north_east_tile_in_valid_position_connects_sometimes() {
        let tile = Tile::new('L'.into(), tile_pos! { 2, 2 });
        // Two valid connection positions.
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {1, 2}),
            Some(tile_pos! {2, 3})
        );
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {2, 3}),
            Some(tile_pos! {1, 2})
        );

        // Rest are invalid positions to connect from.
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {3, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
    }

    #[test]
    fn north_west_tile_at_first_row_never_connects() {
        let tile = Tile::new('J'.into(), tile_pos! { 0, 1 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 2})
            .is_none());
    }

    #[test]
    fn north_west_tile_at_first_col_never_connects() {
        let tile = Tile::new('J'.into(), tile_pos! { 2, 0 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {0, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {3, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 1})
            .is_none());
    }

    #[test]
    fn north_west_tile_in_valid_position_connects_sometimes() {
        let tile = Tile::new('J'.into(), tile_pos! { 2, 2 });
        // Two valid connection positions.
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {1, 2}),
            Some(tile_pos! {2, 1})
        );
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {2, 1}),
            Some(tile_pos! {1, 2})
        );

        // Rest are invalid positions to connect from.
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 3})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {3, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
    }

    #[test]
    fn south_west_tile_at_last_row_never_connects() {
        let tile = Tile::new('7'.into(), tile_pos! { TILE_GRID_SIZE-1, 2 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE - 2, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 3})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-2, 1})
            .is_none());
    }

    #[test]
    fn south_west_tile_at_first_col_never_connects() {
        let tile = Tile::new('7'.into(), tile_pos! { 2, 0 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {3, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {4, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 1})
            .is_none());
    }

    #[test]
    fn south_west_tile_in_valid_position_connects_sometimes() {
        let tile = Tile::new('7'.into(), tile_pos! { 2, 2 });
        // Two valid connection positions.
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {2, 1}),
            Some(tile_pos! {3, 2})
        );
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {3, 2}),
            Some(tile_pos! {2, 1})
        );

        // Rest are invalid positions to connect from.
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 3})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 4})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
    }

    #[test]
    fn south_east_tile_at_last_row_never_connects() {
        let tile = Tile::new('F'.into(), tile_pos! { TILE_GRID_SIZE-1, 2 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE - 2, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 0})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-1, 3})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {TILE_GRID_SIZE-2, 1})
            .is_none());
    }

    #[test]
    fn south_east_tile_at_last_col_never_connects() {
        let tile = Tile::new('F'.into(), tile_pos! { 2, TILE_GRID_SIZE-1 });
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, TILE_GRID_SIZE-1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, TILE_GRID_SIZE-1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {3, TILE_GRID_SIZE-1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {4, TILE_GRID_SIZE-1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, TILE_GRID_SIZE-2})
            .is_none());
    }

    #[test]
    fn south_east_tile_in_valid_position_connects_sometimes() {
        let tile = Tile::new('F'.into(), tile_pos! { 2, 2 });
        // Two valid connection positions.
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {2, 3}),
            Some(tile_pos! {3, 2})
        );
        assert_eq!(
            tile.try_connect_to_next_tile_from(tile_pos! {3, 2}),
            Some(tile_pos! {2, 3})
        );

        // Rest are invalid positions to connect from.
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 2})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 4})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {1, 1})
            .is_none());
        assert!(tile
            .try_connect_to_next_tile_from(tile_pos! {2, 0})
            .is_none());
    }

    #[test]
    fn ground_tile_never_connects() {
        let tile = Tile::new('.'.into(), tile_pos! { 2, 2 });

        for i in 0..TILE_GRID_SIZE {
            for j in 0..TILE_GRID_SIZE {
                assert!(tile
                    .try_connect_to_next_tile_from(tile_pos! {i, j})
                    .is_none());
            }
        }
    }

    #[test]
    #[should_panic]
    fn start_tile_connection_panics() {
        let tile = Tile::new('S'.into(), tile_pos! { 2, 2 });
        tile.try_connect_to_next_tile_from(tile_pos! {2, 2});
    }
}
