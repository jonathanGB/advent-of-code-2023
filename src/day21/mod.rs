use std::{
    collections::{hash_map::Entry, HashMap},
    str::Lines,
};

use crate::solver::Solver;

const GRID_SIZE: usize = 131;

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileType {
    Start,
    Garden,
    Rock,
}

impl TileType {
    fn is_start_tile(&self) -> bool {
        *self == Self::Start
    }

    fn can_step(&self) -> bool {
        match self {
            TileType::Start | TileType::Garden => true,
            TileType::Rock => false,
        }
    }
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            'S' => TileType::Start,
            '.' => TileType::Garden,
            '#' => TileType::Rock,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
    // Iteration stepped at in x-translation and y-translation map.
    stepped_at_iteration: HashMap<(i32, i32), usize>,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    // (row, col, x-translation, y-translation)
    start_position: (usize, usize, i32, i32),
}

impl Grid {
    fn explore_garden_plots(&mut self, steps: usize, infinite_map: bool) -> u32 {
        let mut accessed_positions = vec![self.start_position];
        let mut new_accessed_positions = Vec::new();

        for i in 1..=steps {
            for (row, col, x_translation, y_translation) in accessed_positions.drain(..) {
                let mut next_positions_to_attempt = Vec::new();

                if row > 0 {
                    next_positions_to_attempt.push((row - 1, col, x_translation, y_translation));
                } else if infinite_map {
                    next_positions_to_attempt.push((
                        GRID_SIZE - 1,
                        col,
                        x_translation - 1,
                        y_translation,
                    ));
                }

                if row < GRID_SIZE - 1 {
                    next_positions_to_attempt.push((row + 1, col, x_translation, y_translation));
                } else if infinite_map {
                    next_positions_to_attempt.push((0, col, x_translation + 1, y_translation));
                }

                if col > 0 {
                    next_positions_to_attempt.push((row, col - 1, x_translation, y_translation));
                } else if infinite_map {
                    next_positions_to_attempt.push((
                        row,
                        GRID_SIZE - 1,
                        x_translation,
                        y_translation - 1,
                    ));
                }

                if col < GRID_SIZE - 1 {
                    next_positions_to_attempt.push((row, col + 1, x_translation, y_translation));
                } else if infinite_map {
                    next_positions_to_attempt.push((row, 0, x_translation, y_translation + 1));
                }

                for (row, col, x_translation, y_translation) in next_positions_to_attempt {
                    let tile = &mut self.grid[row][col];
                    if !tile.tile_type.can_step() {
                        continue;
                    }

                    let stepped_at_iteration = tile
                        .stepped_at_iteration
                        .entry((x_translation, y_translation));
                    let add_new_access_position = match stepped_at_iteration {
                        Entry::Occupied(mut occupied_entry) => {
                            let iteration_stepped = occupied_entry.get_mut();
                            if *iteration_stepped < i {
                                *iteration_stepped = i;
                                true
                            } else {
                                false
                            }
                        }
                        Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(i);
                            true
                        }
                    };

                    if add_new_access_position {
                        new_accessed_positions.push((row, col, x_translation, y_translation));
                    }
                }
            }

            accessed_positions.extend(new_accessed_positions.drain(..));
        }

        accessed_positions.len() as u32
    }

    fn new(lines: Lines) -> Self {
        let mut start_position = (0, 0, 0, 0);
        let grid = lines
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, character)| {
                        let tile_type = TileType::from(character);
                        if tile_type.is_start_tile() {
                            start_position = (row, col, 0, 0);
                        }
                        Tile {
                            tile_type,
                            stepped_at_iteration: HashMap::from([((0, 0), 0)]),
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            grid,
            start_position,
        }
    }
}

pub struct Day21Solver {}

impl Solver for Day21Solver {
    fn solve_part1() {
        let steps = 64;
        let file = std::fs::read_to_string("src/day21/input.txt").unwrap();
        let mut grid = Grid::new(file.lines());
        let explored_garden_plots = grid.explore_garden_plots(steps, false);
        println!(
            "The elf can reach {} garden plots in {} steps.",
            explored_garden_plots, steps
        );
    }

    fn solve_part2() {
        let steps = 5000;
        let file = std::fs::read_to_string("src/day21/input.txt").unwrap();
        let mut grid = Grid::new(file.lines());
        let explored_garden_plots = grid.explore_garden_plots(steps, true);
        println!(
            "The elf can reach {} garden plots in {} steps.",
            explored_garden_plots, steps
        );
    }
}
