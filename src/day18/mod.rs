use std::fmt::Debug;

use crate::solver::Solver;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from1(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }

    fn from2(value: char) -> Self {
        match value {
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i64,
}

impl Instruction {
    fn from1(value: &str) -> Self {
        let (direction, distance_and_colour) = value.split_once(' ').unwrap();
        let (distance, _) = distance_and_colour.split_once(' ').unwrap();

        Instruction {
            direction: Direction::from1(direction),
            distance: distance.parse().unwrap(),
        }
    }

    fn from2(value: &str) -> Self {
        let (_, suffix) = value.rsplit_once(' ').unwrap();
        let suffix = suffix.strip_prefix("(#").unwrap();
        let suffix = suffix.strip_suffix(')').unwrap();

        let instruction: Vec<_> = suffix.chars().collect();
        let distance = &instruction[..5];
        let distance = i64::from_str_radix(&String::from_iter(distance), 16).unwrap();
        let direction = Direction::from2(instruction[5]);

        Instruction {
            direction,
            distance,
        }
    }
}

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Trench2 {
    vertices: Vec<Coordinate>,
    // Holds the length of the trench.
    length: i64,
}

impl Trench2 {
    fn dig<I>(instructions: I) -> Self
    where
        I: Iterator<Item = Instruction>,
    {
        let mut x = 0;
        let mut y = 0;
        let mut vertices = Vec::new();
        let mut length = 0;

        for Instruction {
            direction,
            distance,
        } in instructions
        {
            match direction {
                Direction::Down => x -= distance,
                Direction::Up => x += distance,
                Direction::Left => y -= distance,
                Direction::Right => y += distance,
            }

            length += distance;
            vertices.push(Coordinate { x, y });
        }

        Self { vertices, length }
    }

    // Digs the interior area of the trench, and returns the total area of
    // the trench and its interior. This uses a combination of shoelace theorem
    // and Pick's theorem to figure out how many interior points there are.
    fn dig_interior(&self) -> i64 {
        let num_vertices = self.vertices.len();
        let mut area = 0;
        for i in 0..num_vertices {
            let xi = self.vertices[i].x;
            let yp1 = self.vertices[(i + 1) % num_vertices].y;
            let ym1 = if i == 0 {
                self.vertices[num_vertices - 1].y
            } else {
                self.vertices[i - 1].y
            };

            area += xi * (yp1 - ym1);
        }
        area /= 2;

        let interior_points = area - (self.length / 2) + 1;

        interior_points + self.length
    }
}

pub struct Day18Solver {}

impl Solver for Day18Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day18/input.txt").unwrap();

        let trench = Trench2::dig(file.lines().map(Instruction::from1));
        let trench_and_interior_area = trench.dig_interior();
        println!(
            "The area contained by the trench is {}",
            trench_and_interior_area
        );
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day18/input.txt").unwrap();

        let trench = Trench2::dig(file.lines().map(Instruction::from2));
        let trench_and_interior_area = trench.dig_interior();
        println!(
            "The area contained by the trench is {}",
            trench_and_interior_area
        );
    }
}
