use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    str::Lines,
};

use crate::solver::Solver;

const X_Y_GRID_SIZE: usize = 10;

// Note that we order bricks first by their z_start, and then by their z_end.
// The rest of the fields don't impact the ordering.
#[derive(Debug, PartialEq, Eq)]
struct Brick {
    x_start: u8,
    x_end: u8,
    y_start: u8,
    y_end: u8,
    z_start: usize,
    z_end: usize,
    bricks_on_top: HashSet<usize>,
    bricks_underneath: HashSet<usize>,
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.z_start.cmp(&other.z_start) {
            Ordering::Equal => self.z_end.cmp(&other.z_end),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Brick {
    fn get_xy_coordinates(&self) -> Vec<(usize, usize)> {
        let mut xy_coordinates = Vec::new();

        for x in self.x_start..=self.x_end {
            for y in self.y_start..=self.y_end {
                xy_coordinates.push((x as usize, y as usize));
            }
        }

        xy_coordinates
    }
}

#[derive(Debug, Default)]
struct BrickRef {
    index: Option<usize>,
    max_z: usize,
}

#[derive(Default)]
struct XYGrid {
    zs: [[BrickRef; X_Y_GRID_SIZE]; X_Y_GRID_SIZE],
}

impl XYGrid {
    fn brick_fall_z(&self, brick_xy_coordinates: &[(usize, usize)]) -> usize {
        let mut max_z = 0;

        for (x, y) in brick_xy_coordinates {
            max_z = max_z.max(self.zs[*x][*y].max_z);
        }

        max_z
    }

    // Note that this returns the updated input vector, which we drained and moved to the output.
    // The reason why is to deflect the borrowchecker.
    fn free_fall(&mut self, mut bricks: Vec<Brick>) -> Vec<Brick> {
        let mut fallen_bricks = Vec::<Brick>::new();
        fallen_bricks.reserve(bricks.len());

        for (brick_index, mut brick) in bricks.drain(..).enumerate() {
            let brick_xy_coordinates = brick.get_xy_coordinates();
            let brick_fall_z = self.brick_fall_z(&brick_xy_coordinates);
            let brick_new_z_start = brick_fall_z + 1;
            let brick_new_z_end = brick.z_end - brick.z_start + brick_new_z_start;

            for (x, y) in brick_xy_coordinates {
                let under_brick_ref = &mut self.zs[x][y];
                if under_brick_ref.max_z == brick_fall_z && under_brick_ref.index.is_some() {
                    let under_brick_index = under_brick_ref.index.unwrap();

                    fallen_bricks[under_brick_index]
                        .bricks_on_top
                        .insert(brick_index);
                    brick.bricks_underneath.insert(under_brick_index);
                }

                under_brick_ref.index = Some(brick_index);
                under_brick_ref.max_z = brick_new_z_end;
            }

            brick.z_start = brick_new_z_start;
            brick.z_end = brick_new_z_end;
            fallen_bricks.push(brick);
        }

        fallen_bricks
    }
}

// What we know from inspecting the input:
// No bricks in diagonal: only varies in x, y, or z (or none, if it's a single cube).
// Start is smaller or equal to end, i.e. no bricks upside down.
// Lowest z-index possible is 1.
// All x and y coordinates are within [0-9].

pub struct Day22Solver {}

impl Day22Solver {
    fn count_safely_disintegrable_bricks(bricks: &[Brick]) -> usize {
        let mut count = 0;

        'next_brick: for brick in bricks {
            let brick_indices_on_top = &brick.bricks_on_top;

            for brick_index_on_top in brick_indices_on_top {
                let brick_on_top = &bricks[*brick_index_on_top];

                if brick_on_top.bricks_underneath.len() == 1 {
                    continue 'next_brick;
                }
            }

            count += 1;
        }

        count
    }

    fn count_sum_of_other_fallable_bricks(bricks: &[Brick]) -> usize {
        let mut count = 0;

        for i in 0..bricks.len() {
            let mut collapsed_bricks = vec![false; bricks.len()];
            let mut bricks_to_visit = VecDeque::from([i]);
            collapsed_bricks[i] = true;

            while let Some(brick_index) = bricks_to_visit.pop_front() {
                let brick = &bricks[brick_index];

                'bricks_on_top: for brick_index_on_top in &brick.bricks_on_top {
                    if collapsed_bricks[*brick_index_on_top] {
                        continue;
                    }

                    let brick_on_top = &bricks[*brick_index_on_top];
                    for brick_index_underneath_top in &brick_on_top.bricks_underneath {
                        if !collapsed_bricks[*brick_index_underneath_top] {
                            continue 'bricks_on_top;
                        }
                    }

                    collapsed_bricks[*brick_index_on_top] = true;
                    count += 1;
                    bricks_to_visit.push_back(*brick_index_on_top);
                }
            }
        }

        count
    }

    fn free_fall(bricks: Vec<Brick>) -> Vec<Brick> {
        let mut x_y_grid = XYGrid::default();
        x_y_grid.free_fall(bricks)
    }

    fn parse_and_sort_bricks(lines: Lines) -> Vec<Brick> {
        let mut bricks = Vec::new();
        for line in lines {
            let (start_coordinate, end_coordinate) = line.split_once('~').unwrap();

            let (x_start, y_and_z_start) = start_coordinate.split_once(',').unwrap();
            let (y_start, z_start) = y_and_z_start.split_once(',').unwrap();
            let x_start = x_start.parse().unwrap();
            let y_start = y_start.parse().unwrap();
            let z_start = z_start.parse().unwrap();

            let (x_end, y_and_z_end) = end_coordinate.split_once(',').unwrap();
            let (y_end, z_end) = y_and_z_end.split_once(',').unwrap();
            let x_end = x_end.parse().unwrap();
            let y_end = y_end.parse().unwrap();
            let z_end = z_end.parse().unwrap();

            bricks.push(Brick {
                x_start,
                x_end,
                y_start,
                y_end,
                z_start,
                z_end,
                bricks_on_top: HashSet::new(),
                bricks_underneath: HashSet::new(),
            });
        }

        bricks.sort();
        bricks
    }
}

impl Solver for Day22Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day22/input.txt").unwrap();
        let bricks = Self::parse_and_sort_bricks(file.lines());
        let fallen_bricks = Self::free_fall(bricks);
        println!(
            "The number of bricks that could safely be disintegrated is {}",
            Self::count_safely_disintegrable_bricks(&fallen_bricks)
        );
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day22/input.txt").unwrap();
        let bricks = Self::parse_and_sort_bricks(file.lines());
        let fallen_bricks = Self::free_fall(bricks);
        println!(
            "The total number of bricks that would fall in chain is {}",
            Self::count_sum_of_other_fallable_bricks(&fallen_bricks)
        );
    }
}
