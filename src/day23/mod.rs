use hashbrown::{HashMap, HashSet};
use std::str::Lines;

use crate::solver::Solver;

const GRID_SIZE: usize = 141;
const START_POSITION: RowCol = (0, 1);
const END_POSITION: RowCol = (GRID_SIZE - 1, GRID_SIZE - 2);

type RowCol = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Path,
    Forest,
    RightSlope,
    DownSlope,
    LeftSlope,
    UpSlope,
}

impl Tile {
    fn is_forest(&self) -> bool {
        *self == Tile::Forest
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Path,
            '#' => Self::Forest,
            '>' => Self::RightSlope,
            'v' => Self::DownSlope,
            '<' => Self::LeftSlope,
            '^' => Self::UpSlope,
            _ => unreachable!(),
        }
    }
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn new(lines: Lines) -> Self {
        let grid = lines
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        Self { grid }
    }

    fn compact(&self, slippery_slope: bool) -> CompactGrid {
        let mut compact_grid = CompactGrid::default();
        compact_grid.nodes.insert(START_POSITION, Vec::new());
        compact_grid.nodes.insert(END_POSITION, Vec::new());

        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if (i, j) == START_POSITION || (i, j) == END_POSITION {
                    continue;
                }

                let tile = &self.grid[i][j];
                if tile.is_forest() {
                    continue;
                }

                // Ignore slippery slope for this part of the compaction. We just want to know how many
                // neighbours there are for non-forest tiles. If it has more than 2 neighbours, then it
                // is a 3- or 4-way crossroad. This will become a node in our graph. Otherwise, it is a point
                // that will get compacted away.
                let neighbours = self.get_next_positions((i, j), *tile, false);
                let valid_neighbours: Vec<_> = neighbours
                    .into_iter()
                    .filter(|(new_row, new_col)| !self.grid[*new_row][*new_col].is_forest())
                    .collect();
                if valid_neighbours.len() > 2 {
                    compact_grid.nodes.insert((i, j), Vec::new());
                }
            }
        }

        let nodes: Vec<_> = compact_grid.nodes.keys().cloned().collect();
        for node in nodes {
            let (row, col) = node;
            let paths = self.get_next_positions(node, self.grid[row][col], slippery_slope);
            let valid_paths: Vec<_> = paths
                .into_iter()
                .filter(|(new_row, new_col)| !self.grid[*new_row][*new_col].is_forest())
                .collect();

            'next_path: for (mut new_row, mut new_col) in valid_paths {
                let mut visited_nodes: HashSet<RowCol> = HashSet::from([node, (new_row, new_col)]);
                let mut weight = 1;

                'next_position: loop {
                    for (tentative_row, tentative_col) in self.get_next_positions(
                        (new_row, new_col),
                        self.grid[new_row][new_col],
                        slippery_slope,
                    ) {
                        if self.grid[tentative_row][tentative_col].is_forest()
                            || visited_nodes.contains(&(tentative_row, tentative_col))
                        {
                            continue;
                        }

                        weight += 1;
                        new_row = tentative_row;
                        new_col = tentative_col;
                        visited_nodes.insert((new_row, new_col));

                        // We found a path that leads to a node in the graph (but not the origin node of the path).
                        // Let's add a directed edge from the origin node to the found node, and try a new path.
                        if compact_grid.nodes.contains_key(&(new_row, new_col)) {
                            compact_grid.nodes.get_mut(&node).unwrap().push(Edge {
                                destination: (new_row, new_col),
                                weight,
                            });

                            continue 'next_path;
                        }

                        continue 'next_position;
                    }

                    // If we tried all positions orthogonal to the current one and none of them are valid,
                    // the path that is being explored leads to no where. Drop it, and move to a new path
                    // to explore.
                    continue 'next_path;
                }
            }
        }

        compact_grid
    }

    fn get_next_positions(
        &self,
        current_position: RowCol,
        current_tile: Tile,
        slippery_slope: bool,
    ) -> Vec<RowCol> {
        let mut next_positions = Vec::new();
        let (row, col) = current_position;

        match current_tile {
            Tile::Forest => unreachable!(),
            Tile::RightSlope if slippery_slope => {
                if col < GRID_SIZE - 1 {
                    next_positions.push((row, col + 1));
                }
            }
            Tile::DownSlope if slippery_slope => {
                if row < GRID_SIZE - 1 {
                    next_positions.push((row + 1, col));
                }
            }
            Tile::LeftSlope if slippery_slope => {
                if col > 0 {
                    next_positions.push((row, col - 1));
                }
            }
            Tile::UpSlope if slippery_slope => {
                if row > 0 {
                    next_positions.push((row - 1, col));
                }
            }
            _ => {
                // Optimization. This intersection must only direct
                // down, as this is the only path to the end position.
                // During compaction, this will have the effect of ignoring
                // this intersection altogether.
                if row == 135 && col == 133 {
                    next_positions.push((row + 1, col));
                    return next_positions;
                }
                if col < GRID_SIZE - 1 {
                    next_positions.push((row, col + 1));
                }
                if row < GRID_SIZE - 1 {
                    next_positions.push((row + 1, col));
                }
                if col > 0 {
                    next_positions.push((row, col - 1));
                }
                if row > 0 {
                    next_positions.push((row - 1, col));
                }
            }
        }

        next_positions
    }
}

#[derive(Debug)]
struct Edge {
    destination: RowCol,
    weight: usize,
}

#[derive(Default, Debug)]
struct CompactGrid {
    nodes: HashMap<RowCol, Vec<Edge>>,
}

impl CompactGrid {
    fn find_longest_path(&self) -> usize {
        let mut visited_positions = [[false; GRID_SIZE]; GRID_SIZE];
        self.recurisvely_find_longest_path(START_POSITION, &mut visited_positions)
    }

    fn recurisvely_find_longest_path(
        &self,
        position: RowCol,
        visited_positions: &mut [[bool; GRID_SIZE]; GRID_SIZE],
    ) -> usize {
        let mut longest_path = 0;
        if position == END_POSITION {
            return longest_path;
        }

        let (row, col) = position;
        visited_positions[row][col] = true;
        let current_edges = self.nodes.get(&position).unwrap();
        for current_edge in current_edges {
            let (destination_row, destination_col) = current_edge.destination;
            if visited_positions[destination_row][destination_col] {
                continue;
            }

            longest_path = longest_path.max(
                current_edge.weight
                    + self
                        .recurisvely_find_longest_path(current_edge.destination, visited_positions),
            );
        }
        visited_positions[row][col] = false;

        longest_path
    }
}

pub struct Day23Solver {}

impl Day23Solver {}

impl Solver for Day23Solver {
    fn solve_part1() {
        let slippery_slope = true;
        let file = std::fs::read_to_string("src/day23/input.txt").unwrap();
        let grid = Grid::new(file.lines());
        let compact_grid = grid.compact(slippery_slope);
        println!(
            "The longest hike considering slopes is {:?}",
            compact_grid.find_longest_path()
        );
    }

    fn solve_part2() {
        let slippery_slope = false;
        let file = std::fs::read_to_string("src/day23/input.txt").unwrap();
        let grid = Grid::new(file.lines());
        let compact_grid = grid.compact(slippery_slope);
        println!(
            "The longest hike ignoring slopes is {:?}",
            compact_grid.find_longest_path()
        );
    }
}
