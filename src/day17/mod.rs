use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::solver::Solver;

const GRID_SIZE: usize = 141;
const MAX_CRUCIBLE_STRAIGHT_LINE_DISTANCE: usize = 3;
const MIN_ULTRA_CRUCIBLE_STRAIGHT_LINE_DISTANCE: u8 = 4;
const MAX_ULTRA_CRUCIBLE_STRAIGHT_LINE_DISTANCE: usize = 10;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Debug)]
struct Node<const N: usize> {
    closed_from_up: [bool; N],
    closed_from_right: [bool; N],
    closed_from_down: [bool; N],
    closed_from_left: [bool; N],
    loss: u8,
    h: u16,
}

impl<const N: usize> Default for Node<N> {
    fn default() -> Self {
        Self {
            closed_from_up: [bool::default(); N],
            closed_from_right: [bool::default(); N],
            closed_from_down: [bool::default(); N],
            closed_from_left: [bool::default(); N],
            loss: u8::default(),
            h: u16::default(),
        }
    }
}

impl<const N: usize> Node<N> {
    fn is_closed(&self, from: Direction, index: usize) -> bool {
        let closed_from = match from {
            Direction::Up => &self.closed_from_up,
            Direction::Right => &self.closed_from_right,
            Direction::Down => &self.closed_from_down,
            Direction::Left => &self.closed_from_left,
        };

        closed_from[index]
    }

    fn mark_closed(&mut self, from: Direction, index_start: usize, close_indices_afterwards: bool) {
        let closed_from = match from {
            Direction::Up => &mut self.closed_from_up,
            Direction::Right => &mut self.closed_from_right,
            Direction::Down => &mut self.closed_from_down,
            Direction::Left => &mut self.closed_from_left,
        };

        if close_indices_afterwards {
            closed_from
                .iter_mut()
                .skip(index_start)
                .for_each(|closed| *closed = true);
        } else {
            closed_from[index_start] = true;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
struct Grid<const N: usize> {
    nodes: [[Node<N>; GRID_SIZE]; GRID_SIZE],
}

impl<const N: usize> Grid<N> {
    fn new(losses: [[u8; GRID_SIZE]; GRID_SIZE]) -> Self {
        let mut nodes = std::array::from_fn(|_| std::array::from_fn(|_| Node::default()));
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let node = &mut nodes[row][col];

                node.loss = losses[row][col];
                // The heuristic is the Manhattan distance to the end node.
                // This helps the A* algorithm to find the shortest path a bit quicker
                // than just doing Dijkstra's algorithm.
                node.h = ((GRID_SIZE - 1 - row) + (GRID_SIZE - 1 - col)) as u16;
            }
        }

        // Special handling for the start node. Note that we only update the right
        // and down origins, because these are the only two directions that can
        // reach the start node.
        let start_node = &mut nodes[0][0];
        for closed_from_right in &mut start_node.closed_from_right {
            *closed_from_right = true;
        }
        for closed_from_down in &mut start_node.closed_from_down {
            *closed_from_down = true;
        }

        Self { nodes }
    }

    fn get_node(&self, position: Position) -> &Node<N> {
        &self.nodes[position.row][position.col]
    }

    fn get_mut_node(&mut self, position: Position) -> &mut Node<N> {
        &mut self.nodes[position.row][position.col]
    }

    fn get_neighbour_positions(&self, position: Position) -> Vec<(Direction, Position)> {
        let Position { row, col } = position;
        let mut neighbour_positions = Vec::new();

        if row > 0 {
            neighbour_positions.push((Direction::Down, pos!(row - 1, col)));
        }
        if col < GRID_SIZE - 1 {
            neighbour_positions.push((Direction::Left, pos!(row, col + 1)));
        }
        if row < GRID_SIZE - 1 {
            neighbour_positions.push((Direction::Up, pos!(row + 1, col)));
        }
        if col > 0 {
            neighbour_positions.push((Direction::Right, pos!(row, col - 1)));
        }

        neighbour_positions
    }

    fn is_end_node(&self, position: Position) -> bool {
        position.row == GRID_SIZE - 1 && position.col == GRID_SIZE - 1
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    f: u32,
    g: u32,
    position: Position,
    straight_line_distance: u8,
    from: Direction,
}

pub struct Day17Solver {}

impl Day17Solver {
    // Initializes the grid and priority queue. Note that the priority queue uses a max-heap,
    // therefore the state is reversed to emulate a min-heap.
    fn init<const N: usize>() -> (Grid<N>, BinaryHeap<Reverse<State>>) {
        let file = std::fs::read_to_string("src/day17/input.txt").unwrap();
        let mut losses: [[u8; GRID_SIZE]; GRID_SIZE] =
            std::array::from_fn(|_| std::array::from_fn(|_| 0));
        for (row, line) in file.lines().enumerate() {
            for (col, loss) in line.chars().enumerate() {
                losses[row][col] = loss.to_digit(10).unwrap() as u8;
            }
        }
        let grid = Grid::<N>::new(losses);
        // We use the binary heap as a priority queue. We will insert
        // to start with the neighbours to the right and below the start node,
        // as a special case.
        let mut open = BinaryHeap::new();
        let go_right_loss = grid.get_node(pos!(0, 1)).loss;
        let go_down_loss = grid.get_node(pos!(1, 0)).loss;
        let go_right_h = grid.get_node(pos!(0, 1)).h;
        let go_down_h = grid.get_node(pos!(1, 0)).h;

        let go_right_state = State {
            f: go_right_loss as u32 + go_right_h as u32,
            g: go_right_loss as u32,
            position: pos!(0, 1),
            straight_line_distance: 1,
            from: Direction::Left,
        };
        open.push(Reverse(go_right_state));

        let go_down_state = State {
            f: go_down_loss as u32 + go_down_h as u32,
            g: go_down_loss as u32,
            position: pos!(1, 0),
            straight_line_distance: 1,
            from: Direction::Up,
        };
        open.push(Reverse(go_down_state));

        (grid, open)
    }

    fn solve_problem<const N: usize>(
        mut grid: Grid<N>,
        mut open: BinaryHeap<Reverse<State>>,
        min_consecutive_moves: u8,
    ) {
        let close_indices_afterwards = min_consecutive_moves <= 1;
        while let Some(Reverse(current_state)) = open.pop() {
            if grid.get_node(current_state.position).is_closed(
                current_state.from,
                current_state.straight_line_distance as usize - 1,
            ) {
                continue;
            }

            for (from, neighbour_position) in grid.get_neighbour_positions(current_state.position) {
                if current_state.from.opposite() == from {
                    continue;
                }

                if current_state.from != from
                    && current_state.straight_line_distance < min_consecutive_moves
                {
                    continue;
                }

                let neighbour_straight_line_distance = if current_state.from == from {
                    current_state.straight_line_distance + 1
                } else {
                    1
                };

                if neighbour_straight_line_distance as usize > N {
                    continue;
                }

                let neighbour_node = grid.get_node(neighbour_position);
                if neighbour_node.is_closed(from, neighbour_straight_line_distance as usize - 1) {
                    continue;
                }

                let neighbour_g = current_state.g + neighbour_node.loss as u32;
                if grid.is_end_node(neighbour_position) {
                    if neighbour_straight_line_distance < min_consecutive_moves {
                        continue;
                    }

                    println!("The minimal heat loss to reach the end is {neighbour_g}");
                    return;
                }

                open.push(Reverse(State {
                    f: neighbour_g + neighbour_node.h as u32,
                    g: neighbour_g,
                    position: neighbour_position,
                    straight_line_distance: neighbour_straight_line_distance,
                    from,
                }));
            }

            grid.get_mut_node(current_state.position).mark_closed(
                current_state.from,
                current_state.straight_line_distance as usize - 1,
                close_indices_afterwards,
            );
        }
    }
}

impl Solver for Day17Solver {
    fn solve_part1() {
        let (grid, open) = Self::init::<MAX_CRUCIBLE_STRAIGHT_LINE_DISTANCE>();

        Self::solve_problem(grid, open, 1);
    }

    fn solve_part2() {
        let (grid, open) = Self::init::<MAX_ULTRA_CRUCIBLE_STRAIGHT_LINE_DISTANCE>();

        Self::solve_problem(grid, open, MIN_ULTRA_CRUCIBLE_STRAIGHT_LINE_DISTANCE);
    }
}
